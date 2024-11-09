#[cfg(feature = "codegen")]
fn main() {
    micropb_gen::Generator::new()
        .add_protoc_arg("-I./opentelemetry-proto")
        .add_protoc_arg("--experimental_allow_proto3_optional")
        .use_container_std()
        .compile_protos(
            &[
                "opentelemetry-proto/opentelemetry/proto/common/v1/common.proto",
                "opentelemetry-proto/opentelemetry/proto/resource/v1/resource.proto",
                "opentelemetry-proto/opentelemetry/proto/logs/v1/logs.proto",
                "opentelemetry-proto/opentelemetry/proto/metrics/v1/metrics.proto",
                "opentelemetry-proto/opentelemetry/proto/trace/v1/trace.proto",
            ],
            std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/src/generated_std.rs"
        ).unwrap();

    micropb_gen::Generator::new()
        .add_protoc_arg("-I./opentelemetry-proto")
        .add_protoc_arg("--experimental_allow_proto3_optional")
        .configure(".", micropb_gen::Config::new()
            .vec_type("crate::bumpalo::UnsafeVec")
            .string_type("crate::bumpalo::UnsafeString")
            .map_type("BumpaloMapsNotSupported")
            // .container_lifetime("'a")
        )
        .compile_protos(
            &[
                "opentelemetry-proto/opentelemetry/proto/common/v1/common.proto",
                "opentelemetry-proto/opentelemetry/proto/resource/v1/resource.proto",
                "opentelemetry-proto/opentelemetry/proto/logs/v1/logs.proto",
                "opentelemetry-proto/opentelemetry/proto/metrics/v1/metrics.proto",
                "opentelemetry-proto/opentelemetry/proto/trace/v1/trace.proto",
            ],
            std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/src/generated_bumpalo.rs"
        ).unwrap();
}

#[cfg(not(feature = "codegen"))]
fn main() {
    panic!("use ./regenerate.sh in project root")
}