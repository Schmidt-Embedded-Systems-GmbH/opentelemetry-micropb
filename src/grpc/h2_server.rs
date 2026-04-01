use bytes::{Bytes, BytesMut};
use core::marker::PhantomData;
use http::header::{HeaderMap, CONTENT_TYPE};
use http::{HeaderValue, Method, StatusCode};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{timeout, Duration};

use crate::grpc::backend::OtlpBackend;
use crate::grpc::service::OtlpService;

pub struct GrpcServerConfig {
    pub bind_addr: String,
    pub port: u16,
}

pub struct GrpcServer<B, S>
where
    B: OtlpBackend,
    S: OtlpService<B>,
{
    listener: TcpListener,
    service: S,
    scope_provider: B::ScopeProvider,
    _backend: PhantomData<fn() -> B>,
}

impl<B, S> GrpcServer<B, S>
where
    B: OtlpBackend,
    S: OtlpService<B>,
{
    pub async fn listen(
        config: GrpcServerConfig,
        scope_provider: B::ScopeProvider,
        service: S,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let GrpcServerConfig { bind_addr, port } = config;
        let addr = format!("{bind_addr}:{port}");
        let listener = TcpListener::bind(&addr).await?;
        Ok(Self {
            listener,
            service,
            scope_provider,
            _backend: PhantomData,
        })
    }

    pub fn local_addr(&self) -> Result<std::net::SocketAddr, std::io::Error> {
        self.listener.local_addr()
    }

    pub async fn serve_next(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    {
        let (stream, _) = self.listener.accept().await?;
        let service = self.service.clone();
        let scope_provider = self.scope_provider.clone();
        serve_connection_until_closed::<B, S>(stream, scope_provider, service).await
    }

    pub async fn run(self) {
        let listener = self.listener;
        let service = self.service;
        let scope_provider = self.scope_provider;

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let service = service.clone();
                    let scope_provider = scope_provider.clone();
                    tokio::spawn(async move {
                        if let Err(err) =
                            serve_connection_until_closed::<B, S>(
                                stream,
                                scope_provider,
                                service,
                            )
                            .await
                        {
                            eprintln!("h2 server connection error: {err}");
                        }
                    });
                }
                Err(err) => {
                    eprintln!("h2 server accept error: {err}");
                }
            }
        }
    }
}

async fn serve_connection_until_closed<B, S>(
    stream: TcpStream,
    scope_provider: B::ScopeProvider,
    service: S,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    B: OtlpBackend,
    S: OtlpService<B>,
{
    let handshake = timeout(Duration::from_secs(10), h2::server::handshake(stream))
        .await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::TimedOut, "http/2 handshake timed out"))?;

    let mut connection = match handshake {
        Ok(conn) => conn,
        Err(err) => {
            if err
                .to_string()
                .contains("connection closed before reading preface")
            {
                return Ok(());
            }
            return Err(err.into());
        }
    };

    while let Some(request) = connection.accept().await {
        let (request, respond) = request?;
        let service = service.clone();
        let scope_provider = scope_provider.clone();
        tokio::spawn(async move {
            if let Err(err) =
                handle_request::<B, S>(scope_provider, request, respond, service).await
            {
                eprintln!("h2 server request handling error: {err}");
            }
        });
    }

    Ok(())
}

#[derive(Clone, Copy)]
enum ServiceType {
    Traces,
    Logs,
    Metrics,
}

async fn handle_request<B, S>(
    scope_provider: B::ScopeProvider,
    request: http::Request<h2::RecvStream>,
    mut respond: h2::server::SendResponse<Bytes>,
    service: S,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    B: OtlpBackend,
    S: OtlpService<B>,
{
    let wants_trailers = request
        .headers()
        .get("te")
        .and_then(|value| value.to_str().ok())
        .map(|value| value.split(',').any(|part| part.trim().eq_ignore_ascii_case("trailers")))
        .unwrap_or(false);

    if request.method() != Method::POST {
        let response = http::Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(())?;
        respond.send_response(response, true)?;
        return Ok(());
    }

    match request.headers().get(CONTENT_TYPE) {
        Some(content_type) if is_grpc_content_type(content_type) => {}
        Some(_) => {
            let response = http::Response::builder()
                .status(StatusCode::UNSUPPORTED_MEDIA_TYPE)
                .body(())?;
            respond.send_response(response, true)?;
            return Ok(());
        }
        None => {
            let response = http::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(())?;
            respond.send_response(response, true)?;
            return Ok(());
        }
    }

    let service_type = match request.uri().path() {
        path if path.ends_with("/opentelemetry.proto.collector.trace.v1.TraceService/Export") => {
            ServiceType::Traces
        }
        path if path.ends_with("/opentelemetry.proto.collector.logs.v1.LogsService/Export") => {
            ServiceType::Logs
        }
        path if path.ends_with("/opentelemetry.proto.collector.metrics.v1.MetricsService/Export") =>
        {
            ServiceType::Metrics
        }
        _ => {
            let response = http::Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(())?;
            respond.send_response(response, true)?;
            return Ok(());
        }
    };

    let (parts, mut body) = request.into_parts();
    let mut data = BytesMut::new();

    while let Some(chunk) = body.data().await {
        let chunk = chunk?;
        data.extend_from_slice(&chunk);
        body.flow_control().release_capacity(chunk.len())?;
    }

    if data.len() < 5 {
        send_grpc_error(
            &mut respond,
            wants_trailers,
            3,
            "invalid gRPC frame: too short",
        )
        .await?;
        return Ok(());
    }

    if data[0] != 0 {
        send_grpc_error(&mut respond, wants_trailers, 12, "compression not supported").await?;
        return Ok(());
    }

    let message_length = u32::from_be_bytes([data[1], data[2], data[3], data[4]]) as usize;
    if data.len() != message_length + 5 {
        send_grpc_error(
            &mut respond,
            wants_trailers,
            3,
            "invalid gRPC frame: length mismatch",
        )
        .await?;
        return Ok(());
    }

    let message_data = &data[5..];
    enum RequestOutcome {
        Ok(Bytes),
        GrpcError(u8, String),
    }
    let response_data = B::with_scoped(&scope_provider, |scope| match service_type {
        ServiceType::Traces => {
            let request_msg = match B::decode::<B::ExportTraceServiceRequest>(scope, message_data) {
                Ok(msg) => msg,
                Err(err) => {
                    return RequestOutcome::GrpcError(
                        3,
                        format!("failed to decode trace protobuf: {err:?}"),
                    )
                }
            };
            let response_msg = match service.export_spans(parts, request_msg) {
                Ok(msg) => msg,
                Err((status, msg)) => {
                    return RequestOutcome::GrpcError(u8::try_from(status).unwrap_or(13), msg)
                }
            };
            match encode_grpc_message::<B, _>(scope, &response_msg) {
                Ok(bytes) => RequestOutcome::Ok(bytes),
                Err(err) => RequestOutcome::GrpcError(13, err.to_string()),
            }
        }
        ServiceType::Logs => {
            let request_msg = match B::decode::<B::ExportLogsServiceRequest>(scope, message_data) {
                Ok(msg) => msg,
                Err(err) => {
                    return RequestOutcome::GrpcError(
                        3,
                        format!("failed to decode logs protobuf: {err:?}"),
                    )
                }
            };
            let response_msg = match service.export_logs(parts, request_msg) {
                Ok(msg) => msg,
                Err((status, msg)) => {
                    return RequestOutcome::GrpcError(u8::try_from(status).unwrap_or(13), msg)
                }
            };
            match encode_grpc_message::<B, _>(scope, &response_msg) {
                Ok(bytes) => RequestOutcome::Ok(bytes),
                Err(err) => RequestOutcome::GrpcError(13, err.to_string()),
            }
        }
        ServiceType::Metrics => {
            let request_msg =
                match B::decode::<B::ExportMetricsServiceRequest>(scope, message_data) {
                    Ok(msg) => msg,
                    Err(err) => {
                        return RequestOutcome::GrpcError(
                            3,
                            format!("failed to decode metrics protobuf: {err:?}"),
                        )
                    }
                };
            let response_msg = match service.export_metrics(parts, request_msg) {
                Ok(msg) => msg,
                Err((status, msg)) => {
                    return RequestOutcome::GrpcError(u8::try_from(status).unwrap_or(13), msg)
                }
            };
            match encode_grpc_message::<B, _>(scope, &response_msg) {
                Ok(bytes) => RequestOutcome::Ok(bytes),
                Err(err) => RequestOutcome::GrpcError(13, err.to_string()),
            }
        }
    })
    .map_err(|err| -> Box<dyn std::error::Error + Send + Sync> { err.into() })?;

    let response_data = match response_data {
        RequestOutcome::Ok(bytes) => bytes,
        RequestOutcome::GrpcError(code, msg) => {
            send_grpc_error(&mut respond, wants_trailers, code, &msg).await?;
            return Ok(());
        }
    };

    let mut response = http::Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/grpc");
    if !wants_trailers {
        response = response.header("grpc-status", "0");
    }
    let response = response.body(())?;

    let mut stream = respond.send_response(response, false)?;
    if wants_trailers {
        stream.send_data(response_data, false)?;

        let mut trailers = HeaderMap::new();
        trailers.insert("grpc-status", HeaderValue::from_static("0"));
        stream.send_trailers(trailers)?;
    } else {
        stream.send_data(response_data, true)?;
    }
    Ok(())
}

fn encode_grpc_message<B, M>(
    scope: &B::Scope,
    message: &M,
) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>>
where
    B: OtlpBackend,
    M: micropb::MessageEncode,
{
    let payload = B::encode(scope, message)
        .map_err(|err| -> Box<dyn std::error::Error + Send + Sync> { err.into() })?;
    let mut buf = BytesMut::with_capacity(payload.len() + 5);
    buf.extend_from_slice(&[0u8, 0, 0, 0, 0]);
    buf.extend_from_slice(&payload);
    let message_len = payload.len() as u32;
    buf[1..5].copy_from_slice(&message_len.to_be_bytes());
    Ok(buf.into())
}

async fn send_grpc_error(
    respond: &mut h2::server::SendResponse<Bytes>,
    wants_trailers: bool,
    status: u8,
    error_message: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut response = http::Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/grpc");
    if !wants_trailers {
        response = response
            .header("grpc-status", status.to_string())
            .header("grpc-message", error_message);
    }
    let response = response.body(())?;

    if wants_trailers {
        let mut stream = respond.send_response(response, false)?;
        let mut trailers = HeaderMap::new();
        trailers.insert("grpc-status", HeaderValue::from_str(&status.to_string())?);
        trailers.insert("grpc-message", HeaderValue::from_str(error_message)?);
        stream.send_trailers(trailers)?;
    } else {
        respond.send_response(response, true)?;
    }
    Ok(())
}

fn is_grpc_content_type(value: &HeaderValue) -> bool {
    value
        .to_str()
        .ok()
        .map(|content_type| {
            let media_type = content_type
                .split(';')
                .next()
                .map(str::trim)
                .unwrap_or(content_type);
            media_type.eq_ignore_ascii_case("application/grpc")
                || media_type
                    .strip_prefix("application/grpc+")
                    .is_some_and(|suffix| !suffix.is_empty())
        })
        .unwrap_or(false)
}
