mod backend;
mod service;

#[cfg(feature = "h2-client")]
mod h2_client;

#[cfg(feature = "h2-server")]
mod h2_server;

pub use backend::OtlpBackend;
pub use backend::StdBackend;
#[cfg(feature = "use_area")]
pub use backend::AreaBackend;
#[cfg(feature = "use_bumpalo")]
pub use backend::BumpaloBackend;
#[cfg(feature = "use_bumpalo")]
pub use backend::BumpaloScopeConfig;
pub use service::OtlpService;

#[cfg(feature = "h2-client")]
pub use h2_client::{ClientError, GrpcClient};

#[cfg(feature = "h2-server")]
pub use h2_server::{GrpcServer, GrpcServerConfig};
