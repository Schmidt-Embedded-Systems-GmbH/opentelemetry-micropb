#![allow(nonstandard_style, unused, irrefutable_let_patterns)]

#[cfg(feature = "use_area")]
pub mod area;

#[cfg(feature = "use_std")]
mod generated_std;

#[cfg(feature = "use_std")]
pub mod std {
    pub use crate::generated_std::opentelemetry_::proto_::*;
}

#[cfg(feature = "use_bumpalo")]
mod generated_bumpalo;

#[cfg(feature = "use_bumpalo")]
pub mod bumpalo;

#[cfg(feature = "use_area")]
mod generated_area;

#[cfg(feature = "use_area")]
pub mod area_proto {
    pub use crate::generated_area::opentelemetry_::proto_::*;
}

#[cfg(any(feature = "h2-client", feature = "h2-server"))]
pub mod grpc;
