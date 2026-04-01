#![cfg(all(
    feature = "use_std",
    feature = "h2-client",
    feature = "h2-server"
))]

use opentelemetry_micropb::grpc::{
    GrpcClient, GrpcServer, GrpcServerConfig, OtlpBackend, OtlpService, StdBackend,
};
use opentelemetry_proto::tonic::collector::logs::v1::logs_service_client::LogsServiceClient;
use opentelemetry_proto::tonic::collector::logs::v1::logs_service_server::{
    LogsService, LogsServiceServer,
};
use opentelemetry_proto::tonic::collector::logs::v1::{
    ExportLogsServiceRequest as TonicExportLogsServiceRequest,
    ExportLogsServiceResponse as TonicExportLogsServiceResponse,
};
use opentelemetry_proto::tonic::collector::metrics::v1::metrics_service_client::MetricsServiceClient;
use opentelemetry_proto::tonic::collector::metrics::v1::metrics_service_server::{
    MetricsService, MetricsServiceServer,
};
use opentelemetry_proto::tonic::collector::metrics::v1::{
    ExportMetricsServiceRequest as TonicExportMetricsServiceRequest,
    ExportMetricsServiceResponse as TonicExportMetricsServiceResponse,
};
use opentelemetry_proto::tonic::collector::trace::v1::trace_service_client::TraceServiceClient;
use opentelemetry_proto::tonic::collector::trace::v1::trace_service_server::{
    TraceService, TraceServiceServer,
};
use opentelemetry_proto::tonic::collector::trace::v1::{
    ExportTraceServiceRequest as TonicExportTraceServiceRequest,
    ExportTraceServiceResponse as TonicExportTraceServiceResponse,
};
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::{Channel, Server};

struct NoopBackendService<B>(PhantomData<fn() -> B>);

impl<B> Clone for NoopBackendService<B> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<B> Default for NoopBackendService<B> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<B> OtlpService<B> for NoopBackendService<B>
where
    B: OtlpBackend,
{
    fn export_spans(
        &self,
        _meta: http::request::Parts,
        _request: B::ExportTraceServiceRequest,
    ) -> Result<B::ExportTraceServiceResponse, (u16, String)> {
        Ok(Default::default())
    }

    fn export_logs(
        &self,
        _meta: http::request::Parts,
        _request: B::ExportLogsServiceRequest,
    ) -> Result<B::ExportLogsServiceResponse, (u16, String)> {
        Ok(Default::default())
    }

    fn export_metrics(
        &self,
        _meta: http::request::Parts,
        _request: B::ExportMetricsServiceRequest,
    ) -> Result<B::ExportMetricsServiceResponse, (u16, String)> {
        Ok(Default::default())
    }
}

#[derive(Clone, Default)]
struct NoopTonicService;

#[tonic::async_trait]
impl TraceService for NoopTonicService {
    async fn export(
        &self,
        _request: tonic::Request<TonicExportTraceServiceRequest>,
    ) -> Result<tonic::Response<TonicExportTraceServiceResponse>, tonic::Status> {
        Ok(tonic::Response::new(Default::default()))
    }
}

#[tonic::async_trait]
impl LogsService for NoopTonicService {
    async fn export(
        &self,
        _request: tonic::Request<TonicExportLogsServiceRequest>,
    ) -> Result<tonic::Response<TonicExportLogsServiceResponse>, tonic::Status> {
        Ok(tonic::Response::new(Default::default()))
    }
}

#[tonic::async_trait]
impl MetricsService for NoopTonicService {
    async fn export(
        &self,
        _request: tonic::Request<TonicExportMetricsServiceRequest>,
    ) -> Result<tonic::Response<TonicExportMetricsServiceResponse>, tonic::Status> {
        Ok(tonic::Response::new(Default::default()))
    }
}

trait BackendHarness: OtlpBackend {
    type ClientScopeHolder;

    fn server_scope_provider() -> Self::ScopeProvider;
    fn client_scope_holder() -> Self::ClientScopeHolder;
    fn client_scope(holder: &Self::ClientScopeHolder) -> &Self::Scope;

    fn make_trace_request(holder: &Self::ClientScopeHolder) -> Self::ExportTraceServiceRequest {
        Self::with_scope(Self::client_scope(holder), Default::default)
    }

    fn make_logs_request(holder: &Self::ClientScopeHolder) -> Self::ExportLogsServiceRequest {
        Self::with_scope(Self::client_scope(holder), Default::default)
    }

    fn make_metrics_request(
        holder: &Self::ClientScopeHolder,
    ) -> Self::ExportMetricsServiceRequest {
        Self::with_scope(Self::client_scope(holder), Default::default)
    }

    fn assert_trace_response(
        holder: &Self::ClientScopeHolder,
        response: &Self::ExportTraceServiceResponse,
    ) {
        let actual = Self::encode(Self::client_scope(holder), response).unwrap();
        let expected = Self::with_scope(Self::client_scope(holder), || {
            let response = Self::ExportTraceServiceResponse::default();
            Self::encode(Self::client_scope(holder), &response).unwrap()
        });
        assert_eq!(actual, expected);
    }

    fn assert_logs_response(
        holder: &Self::ClientScopeHolder,
        response: &Self::ExportLogsServiceResponse,
    ) {
        let actual = Self::encode(Self::client_scope(holder), response).unwrap();
        let expected = Self::with_scope(Self::client_scope(holder), || {
            let response = Self::ExportLogsServiceResponse::default();
            Self::encode(Self::client_scope(holder), &response).unwrap()
        });
        assert_eq!(actual, expected);
    }

    fn assert_metrics_response(
        holder: &Self::ClientScopeHolder,
        response: &Self::ExportMetricsServiceResponse,
    ) {
        let actual = Self::encode(Self::client_scope(holder), response).unwrap();
        let expected = Self::with_scope(Self::client_scope(holder), || {
            let response = Self::ExportMetricsServiceResponse::default();
            Self::encode(Self::client_scope(holder), &response).unwrap()
        });
        assert_eq!(actual, expected);
    }
}

impl BackendHarness for StdBackend {
    type ClientScopeHolder = ();

    fn server_scope_provider() -> Self::ScopeProvider {}

    fn client_scope_holder() -> Self::ClientScopeHolder {}

    fn client_scope(_holder: &Self::ClientScopeHolder) -> &Self::Scope {
        &()
    }
}

#[cfg(feature = "use_bumpalo")]
impl BackendHarness for opentelemetry_micropb::grpc::BumpaloBackend {
    type ClientScopeHolder = bumpalo::Bump;

    fn server_scope_provider() -> Self::ScopeProvider {
        opentelemetry_micropb::grpc::BumpaloScopeConfig { capacity: 4096 }
    }

    fn client_scope_holder() -> Self::ClientScopeHolder {
        bumpalo::Bump::with_capacity(4096)
    }

    fn client_scope(holder: &Self::ClientScopeHolder) -> &Self::Scope {
        holder
    }
}

#[cfg(feature = "use_area")]
struct AreaClientScopeHolder {
    _pool: Arc<opentelemetry_micropb::area::AreaPool>,
    area: opentelemetry_micropb::area::Area,
}

#[cfg(feature = "use_area")]
impl BackendHarness for opentelemetry_micropb::grpc::AreaBackend {
    type ClientScopeHolder = AreaClientScopeHolder;

    fn server_scope_provider() -> Self::ScopeProvider {
        Arc::new(opentelemetry_micropb::area::AreaPool::new(4096, 32))
    }

    fn client_scope_holder() -> Self::ClientScopeHolder {
        let pool = Arc::new(opentelemetry_micropb::area::AreaPool::new(4096, 32));
        let area = pool.checkout().unwrap();
        AreaClientScopeHolder { _pool: pool, area }
    }

    fn client_scope(holder: &Self::ClientScopeHolder) -> &Self::Scope {
        &holder.area
    }
}

async fn spawn_our_server<B>() -> (SocketAddr, JoinHandle<()>)
where
    B: BackendHarness,
{
    let server = GrpcServer::<B, NoopBackendService<B>>::listen(
        GrpcServerConfig {
            bind_addr: "127.0.0.1".to_owned(),
            port: 0,
        },
        B::server_scope_provider(),
        NoopBackendService::<B>::default(),
    )
    .await
    .unwrap();
    let local_addr = server.local_addr().unwrap();
    let handle = tokio::spawn(async move { server.run().await });
    sleep(Duration::from_millis(50)).await;
    (local_addr, handle)
}

async fn spawn_tonic_server() -> (SocketAddr, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let local_addr = listener.local_addr().unwrap();
    let incoming = TcpListenerStream::new(listener);
    let service = NoopTonicService;
    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(TraceServiceServer::new(service.clone()))
            .add_service(LogsServiceServer::new(service.clone()))
            .add_service(MetricsServiceServer::new(service))
            .serve_with_incoming(incoming)
            .await
            .unwrap();
    });
    sleep(Duration::from_millis(50)).await;
    (local_addr, handle)
}

async fn tonic_channel(addr: SocketAddr) -> Channel {
    Channel::from_shared(format!("http://{addr}"))
        .unwrap()
        .connect()
        .await
        .unwrap()
}

async fn assert_tonic_trace_to_our_server<B>()
where
    B: BackendHarness,
{
    let (addr, handle) = spawn_our_server::<B>().await;
    let mut client = TraceServiceClient::new(tonic_channel(addr).await);
    let response = client
        .export(TonicExportTraceServiceRequest::default())
        .await
        .unwrap()
        .into_inner();
    assert_eq!(response, TonicExportTraceServiceResponse::default());
    handle.abort();
}

async fn assert_tonic_logs_to_our_server<B>()
where
    B: BackendHarness,
{
    let (addr, handle) = spawn_our_server::<B>().await;
    let mut client = LogsServiceClient::new(tonic_channel(addr).await);
    let response = client
        .export(TonicExportLogsServiceRequest::default())
        .await
        .unwrap()
        .into_inner();
    assert_eq!(response, TonicExportLogsServiceResponse::default());
    handle.abort();
}

async fn assert_tonic_metrics_to_our_server<B>()
where
    B: BackendHarness,
{
    let (addr, handle) = spawn_our_server::<B>().await;
    let mut client = MetricsServiceClient::new(tonic_channel(addr).await);
    let response = client
        .export(TonicExportMetricsServiceRequest::default())
        .await
        .unwrap()
        .into_inner();
    assert_eq!(response, TonicExportMetricsServiceResponse::default());
    handle.abort();
}

async fn assert_our_trace_client_to_tonic_server<B>()
where
    B: BackendHarness,
{
    let (addr, handle) = spawn_tonic_server().await;
    let client = GrpcClient::<B>::connect(addr.to_string()).await.unwrap();
    let holder = B::client_scope_holder();
    let response = client
        .export_traces(B::client_scope(&holder), B::make_trace_request(&holder))
        .await
        .unwrap();
    B::assert_trace_response(&holder, &response);
    handle.abort();
}

async fn assert_our_logs_client_to_tonic_server<B>()
where
    B: BackendHarness,
{
    let (addr, handle) = spawn_tonic_server().await;
    let client = GrpcClient::<B>::connect(addr.to_string()).await.unwrap();
    let holder = B::client_scope_holder();
    let response = client
        .export_logs(B::client_scope(&holder), B::make_logs_request(&holder))
        .await
        .unwrap();
    B::assert_logs_response(&holder, &response);
    handle.abort();
}

async fn assert_our_metrics_client_to_tonic_server<B>()
where
    B: BackendHarness,
{
    let (addr, handle) = spawn_tonic_server().await;
    let client = GrpcClient::<B>::connect(addr.to_string()).await.unwrap();
    let holder = B::client_scope_holder();
    let response = client
        .export_metrics(B::client_scope(&holder), B::make_metrics_request(&holder))
        .await
        .unwrap();
    B::assert_metrics_response(&holder, &response);
    handle.abort();
}

macro_rules! backend_transport_tests {
    ($module:ident, $backend:ty) => {
        mod $module {
            use super::*;

            #[tokio::test]
            async fn tonic_trace_client_works() {
                assert_tonic_trace_to_our_server::<$backend>().await;
            }

            #[tokio::test]
            async fn tonic_logs_client_works() {
                assert_tonic_logs_to_our_server::<$backend>().await;
            }

            #[tokio::test]
            async fn tonic_metrics_client_works() {
                assert_tonic_metrics_to_our_server::<$backend>().await;
            }

            #[tokio::test]
            async fn our_trace_client_works() {
                assert_our_trace_client_to_tonic_server::<$backend>().await;
            }

            #[tokio::test]
            async fn our_logs_client_works() {
                assert_our_logs_client_to_tonic_server::<$backend>().await;
            }

            #[tokio::test]
            async fn our_metrics_client_works() {
                assert_our_metrics_client_to_tonic_server::<$backend>().await;
            }
        }
    };
}

backend_transport_tests!(std_backend, StdBackend);

#[cfg(feature = "use_bumpalo")]
backend_transport_tests!(bumpalo_backend, opentelemetry_micropb::grpc::BumpaloBackend);

#[cfg(feature = "use_area")]
backend_transport_tests!(area_backend, opentelemetry_micropb::grpc::AreaBackend);
