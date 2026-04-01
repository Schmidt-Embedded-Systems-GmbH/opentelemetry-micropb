use bytes::{Bytes, BytesMut};
use core::marker::PhantomData;
use h2::client;
use http::{HeaderMap, Method, Request, StatusCode};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;

use crate::grpc::backend::OtlpBackend;

#[derive(Debug)]
pub enum ClientError {
    Connection(String),
    Grpc(u8, String),
    HttpStatus(StatusCode, String),
    Encoding(String),
    Decoding(String),
    Http(String),
}

impl core::fmt::Display for ClientError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Connection(msg) => write!(f, "Connection error: {msg}"),
            Self::Grpc(status, msg) => write!(f, "gRPC error {status}: {msg}"),
            Self::HttpStatus(status, msg) => write!(f, "HTTP status {status}: {msg}"),
            Self::Encoding(msg) => write!(f, "Encoding error: {msg}"),
            Self::Decoding(msg) => write!(f, "Decoding error: {msg}"),
            Self::Http(msg) => write!(f, "HTTP error: {msg}"),
        }
    }
}

impl std::error::Error for ClientError {}

pub struct GrpcClient<B>
where
    B: OtlpBackend,
{
    client: h2::client::SendRequest<Bytes>,
    h2_connection_task: JoinHandle<()>,
    addr: String,
    _backend: PhantomData<fn() -> B>,
}

impl<B> Drop for GrpcClient<B>
where
    B: OtlpBackend,
{
    fn drop(&mut self) {
        self.h2_connection_task.abort();
    }
}

impl<B> GrpcClient<B>
where
    B: OtlpBackend,
{
    pub async fn connect(addr: String) -> Result<Self, ClientError> {
        let stream = TcpStream::connect(&addr)
            .await
            .map_err(|e| ClientError::Connection(e.to_string()))?;

        let (client, h2_connection) = client::handshake(stream)
            .await
            .map_err(|e| ClientError::Connection(format!("HTTP/2 handshake failed: {e}")))?;

        let h2_connection_task = tokio::spawn(async move {
            if let Err(e) = h2_connection.await {
                eprintln!("h2 client connection error: {e}");
            }
        });

        Ok(Self {
            client,
            h2_connection_task,
            addr,
            _backend: PhantomData,
        })
    }

    pub fn addr(&self) -> &str {
        &self.addr
    }

    pub async fn export_traces(
        &self,
        scope: &B::Scope,
        request: B::ExportTraceServiceRequest,
    ) -> Result<B::ExportTraceServiceResponse, ClientError> {
        self.send_request(
            scope,
            request,
            "/opentelemetry.proto.collector.trace.v1.TraceService/Export",
        )
        .await
    }

    pub async fn export_logs(
        &self,
        scope: &B::Scope,
        request: B::ExportLogsServiceRequest,
    ) -> Result<B::ExportLogsServiceResponse, ClientError> {
        self.send_request(
            scope,
            request,
            "/opentelemetry.proto.collector.logs.v1.LogsService/Export",
        )
        .await
    }

    pub async fn export_metrics(
        &self,
        scope: &B::Scope,
        request: B::ExportMetricsServiceRequest,
    ) -> Result<B::ExportMetricsServiceResponse, ClientError> {
        self.send_request(
            scope,
            request,
            "/opentelemetry.proto.collector.metrics.v1.MetricsService/Export",
        )
        .await
    }

    async fn send_request<Req, Resp>(
        &self,
        scope: &B::Scope,
        request: Req,
        path: &str,
    ) -> Result<Resp, ClientError>
    where
        Req: micropb::MessageEncode,
        Resp: micropb::MessageDecode + Default,
    {
        let mut ready_client = self
            .client
            .clone()
            .ready()
            .await
            .map_err(|e| ClientError::Connection(format!("Client not ready: {e}")))?;

        let request_data = encode_grpc_message::<B, _>(scope, &request)?;

        let http_request = Request::builder()
            .method(Method::POST)
            .uri(path)
            .header("content-type", "application/grpc")
            .header("te", "trailers")
            .body(())
            .map_err(|e| ClientError::Http(format!("Failed to build request: {e}")))?;

        let (response, mut request_stream) = ready_client
            .send_request(http_request, false)
            .map_err(|e| ClientError::Http(format!("Failed to send request: {e}")))?;

        request_stream
            .send_data(request_data, true)
            .map_err(|e| ClientError::Http(format!("Failed to send data: {e}")))?;

        let response = response
            .await
            .map_err(|e| ClientError::Http(format!("Failed to receive response: {e}")))?;

        let http_status = response.status();
        let response_headers = response.headers().clone();

        if !http_status.is_success() {
            let message = response
                .headers()
                .get(http::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .map(|v| format!("unexpected content-type {v}"))
                .unwrap_or_else(|| "non-success HTTP response".to_owned());
            return Err(ClientError::HttpStatus(http_status, message));
        }

        let mut body = response.into_body();
        let mut response_data = BytesMut::new();

        while let Some(chunk) = body.data().await {
            let chunk =
                chunk.map_err(|e| ClientError::Http(format!("Failed to read response chunk: {e}")))?;
            response_data.extend_from_slice(&chunk);
            body.flow_control()
                .release_capacity(chunk.len())
                .map_err(|e| ClientError::Http(format!("Flow control error: {e}")))?;
        }

        let trailers = body
            .trailers()
            .await
            .map_err(|e| ClientError::Http(format!("Failed to read response trailers: {e}")))?;
        if let Some(trailers) = trailers.as_ref() {
            check_grpc_status(trailers)?;
        } else {
            check_grpc_status(&response_headers)?;
        }

        decode_grpc_message::<B, Resp>(scope, &response_data)
    }
}

fn check_grpc_status(headers: &HeaderMap) -> Result<(), ClientError> {
    if let Some(grpc_status) = headers.get("grpc-status") {
        if let Ok(status_str) = grpc_status.to_str() {
            if let Ok(status_code) = status_str.parse::<u8>() {
                if status_code != 0 {
                    let error_message = headers
                        .get("grpc-message")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("Unknown error");
                    return Err(ClientError::Grpc(status_code, error_message.to_owned()));
                }
            }
        }
    }
    Ok(())
}

fn encode_grpc_message<B, M>(scope: &B::Scope, message: &M) -> Result<Bytes, ClientError>
where
    B: OtlpBackend,
    M: micropb::MessageEncode,
{
    let payload = B::encode(scope, message).map_err(ClientError::Encoding)?;
    let mut buf = BytesMut::with_capacity(payload.len() + 5);
    buf.extend_from_slice(&[0u8, 0, 0, 0, 0]);
    buf.extend_from_slice(&payload);
    let message_len = payload.len() as u32;
    buf[1..5].copy_from_slice(&message_len.to_be_bytes());
    Ok(buf.into())
}

fn decode_grpc_message<B, M>(scope: &B::Scope, data: &[u8]) -> Result<M, ClientError>
where
    B: OtlpBackend,
    M: micropb::MessageDecode + Default,
{
    if data.len() < 5 {
        return Err(ClientError::Decoding(
            "Response too short for gRPC frame".to_owned(),
        ));
    }

    if data[0] != 0 {
        return Err(ClientError::Decoding(
            "Compressed responses not supported".to_owned(),
        ));
    }

    let message_length = u32::from_be_bytes([data[1], data[2], data[3], data[4]]) as usize;
    if data.len() != message_length + 5 {
        return Err(ClientError::Decoding(format!(
            "gRPC frame length mismatch: expected {}, got {}",
            message_length + 5,
            data.len()
        )));
    }

    B::decode(scope, &data[5..]).map_err(|e| ClientError::Decoding(format!("{e:?}")))
}
