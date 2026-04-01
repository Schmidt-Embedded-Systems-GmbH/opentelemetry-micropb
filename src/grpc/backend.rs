use core::convert::Infallible;
use micropb::{DecodeError, MessageDecode, MessageEncode, PbDecoder, PbEncoder};
#[cfg(feature = "use_area")]
use std::sync::Arc;

pub trait OtlpBackend: Send + Sync + 'static {
    type Scope;
    type ScopeProvider: Clone + Send + Sync + 'static;

    type ExportTraceServiceRequest: MessageDecode + MessageEncode + Default;
    type ExportTraceServiceResponse: MessageDecode + MessageEncode + Default;
    type ExportLogsServiceRequest: MessageDecode + MessageEncode + Default;
    type ExportLogsServiceResponse: MessageDecode + MessageEncode + Default;
    type ExportMetricsServiceRequest: MessageDecode + MessageEncode + Default;
    type ExportMetricsServiceResponse: MessageDecode + MessageEncode + Default;

    fn with_scope<R>(scope: &Self::Scope, f: impl FnOnce() -> R) -> R;

    fn with_scoped<R>(
        provider: &Self::ScopeProvider,
        f: impl FnOnce(&Self::Scope) -> R,
    ) -> Result<R, String>;

    fn decode<M>(scope: &Self::Scope, data: &[u8]) -> Result<M, DecodeError<Infallible>>
    where
        M: MessageDecode + Default,
    {
        Self::with_scope(scope, || {
            let mut message = M::default();
            let mut decoder = PbDecoder::new(data);
            message.decode(&mut decoder, data.len())?;
            Ok(message)
        })
    }

    fn encode<M>(scope: &Self::Scope, message: &M) -> Result<Vec<u8>, String>
    where
        M: MessageEncode,
    {
        struct VecWrite(Vec<u8>);

        impl micropb::PbWrite for VecWrite {
            type Error = std::io::Error;

            fn pb_write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
                self.0.extend_from_slice(data);
                Ok(())
            }
        }

        Self::with_scope(scope, || {
            let mut encoder = PbEncoder::new(VecWrite(Vec::new()));
            message
                .encode(&mut encoder)
                .map_err(|err| err.to_string())?;
            Ok(encoder.into_writer().0)
        })
    }
}

pub struct StdBackend;

impl OtlpBackend for StdBackend {
    type Scope = ();
    type ScopeProvider = ();
    type ExportTraceServiceRequest = crate::std::collector_::trace_::v1_::ExportTraceServiceRequest;
    type ExportTraceServiceResponse =
        crate::std::collector_::trace_::v1_::ExportTraceServiceResponse;
    type ExportLogsServiceRequest = crate::std::collector_::logs_::v1_::ExportLogsServiceRequest;
    type ExportLogsServiceResponse =
        crate::std::collector_::logs_::v1_::ExportLogsServiceResponse;
    type ExportMetricsServiceRequest =
        crate::std::collector_::metrics_::v1_::ExportMetricsServiceRequest;
    type ExportMetricsServiceResponse =
        crate::std::collector_::metrics_::v1_::ExportMetricsServiceResponse;

    fn with_scope<R>(_scope: &Self::Scope, f: impl FnOnce() -> R) -> R {
        f()
    }

    fn with_scoped<R>(
        _provider: &Self::ScopeProvider,
        f: impl FnOnce(&Self::Scope) -> R,
    ) -> Result<R, String> {
        Ok(f(&()))
    }
}

#[cfg(feature = "use_bumpalo")]
pub struct BumpaloBackend;

#[cfg(feature = "use_bumpalo")]
#[derive(Clone, Copy, Debug)]
pub struct BumpaloScopeConfig {
    pub capacity: usize,
}

#[cfg(feature = "use_bumpalo")]
impl OtlpBackend for BumpaloBackend {
    type Scope = bumpalo::Bump;
    type ScopeProvider = BumpaloScopeConfig;
    type ExportTraceServiceRequest =
        crate::bumpalo::collector_::trace_::v1_::ExportTraceServiceRequest;
    type ExportTraceServiceResponse =
        crate::bumpalo::collector_::trace_::v1_::ExportTraceServiceResponse;
    type ExportLogsServiceRequest = crate::bumpalo::collector_::logs_::v1_::ExportLogsServiceRequest;
    type ExportLogsServiceResponse =
        crate::bumpalo::collector_::logs_::v1_::ExportLogsServiceResponse;
    type ExportMetricsServiceRequest =
        crate::bumpalo::collector_::metrics_::v1_::ExportMetricsServiceRequest;
    type ExportMetricsServiceResponse =
        crate::bumpalo::collector_::metrics_::v1_::ExportMetricsServiceResponse;

    fn with_scope<R>(scope: &Self::Scope, f: impl FnOnce() -> R) -> R {
        crate::bumpalo::with_pool_in_scope(scope, f)
    }

    fn with_scoped<R>(
        provider: &Self::ScopeProvider,
        f: impl FnOnce(&Self::Scope) -> R,
    ) -> Result<R, String> {
        let bump = bumpalo::Bump::with_capacity(provider.capacity);
        Ok(crate::bumpalo::with_pool_in_scope(&bump, || f(&bump)))
    }
}

#[cfg(feature = "use_area")]
pub struct AreaBackend;

#[cfg(feature = "use_area")]
impl OtlpBackend for AreaBackend {
    type Scope = crate::area::Area;
    type ScopeProvider = Arc<crate::area::AreaPool>;
    type ExportTraceServiceRequest =
        crate::area_proto::collector_::trace_::v1_::ExportTraceServiceRequest;
    type ExportTraceServiceResponse =
        crate::area_proto::collector_::trace_::v1_::ExportTraceServiceResponse;
    type ExportLogsServiceRequest =
        crate::area_proto::collector_::logs_::v1_::ExportLogsServiceRequest;
    type ExportLogsServiceResponse =
        crate::area_proto::collector_::logs_::v1_::ExportLogsServiceResponse;
    type ExportMetricsServiceRequest =
        crate::area_proto::collector_::metrics_::v1_::ExportMetricsServiceRequest;
    type ExportMetricsServiceResponse =
        crate::area_proto::collector_::metrics_::v1_::ExportMetricsServiceResponse;

    fn with_scope<R>(scope: &Self::Scope, f: impl FnOnce() -> R) -> R {
        crate::area::with_area_in_scope(scope, f)
    }

    fn with_scoped<R>(
        provider: &Self::ScopeProvider,
        f: impl FnOnce(&Self::Scope) -> R,
    ) -> Result<R, String> {
        let area = provider
            .checkout()
            .ok_or_else(|| "area pool exhausted".to_owned())?;
        Ok(crate::area::with_area_in_scope(&area, || f(&area)))
    }
}
