use crate::grpc::backend::OtlpBackend;

pub trait OtlpService<B>: Clone + Send + Sync + 'static
where
    B: OtlpBackend,
{
    fn export_spans(
        &self,
        meta: http::request::Parts,
        request: B::ExportTraceServiceRequest,
    ) -> Result<B::ExportTraceServiceResponse, (u16, String)>;

    fn export_logs(
        &self,
        meta: http::request::Parts,
        request: B::ExportLogsServiceRequest,
    ) -> Result<B::ExportLogsServiceResponse, (u16, String)>;

    fn export_metrics(
        &self,
        meta: http::request::Parts,
        request: B::ExportMetricsServiceRequest,
    ) -> Result<B::ExportMetricsServiceResponse, (u16, String)>;
}
