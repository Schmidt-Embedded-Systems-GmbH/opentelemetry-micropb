#[cfg(feature = "codegen")]
fn main() {
    let no_clone = micropb_gen::Config::new().no_clone_impl(true);
    let protos = [
        "opentelemetry-proto/opentelemetry/proto/common/v1/common.proto",
        "opentelemetry-proto/opentelemetry/proto/collector/logs/v1/logs_service.proto",
        "opentelemetry-proto/opentelemetry/proto/collector/metrics/v1/metrics_service.proto",
        "opentelemetry-proto/opentelemetry/proto/collector/trace/v1/trace_service.proto",
        "opentelemetry-proto/opentelemetry/proto/resource/v1/resource.proto",
        "opentelemetry-proto/opentelemetry/proto/logs/v1/logs.proto",
        "opentelemetry-proto/opentelemetry/proto/metrics/v1/metrics.proto",
        "opentelemetry-proto/opentelemetry/proto/trace/v1/trace.proto",
    ];
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut std_gen = micropb_gen::Generator::new();
    std_gen
        .add_protoc_arg("-I./opentelemetry-proto")
        .add_protoc_arg("--experimental_allow_proto3_optional")
        // Note: We implement Clone for StdBackend!
        // .configure(".", no_clone.clone())
        .use_container_std();
    std_gen
        .compile_protos(&protos, manifest_dir.clone() + "/src/generated_std.rs")
        .unwrap();

    let mut bumpalo_gen = micropb_gen::Generator::new();
    bumpalo_gen
        .add_protoc_arg("-I./opentelemetry-proto")
        .add_protoc_arg("--experimental_allow_proto3_optional")
        .configure(".", no_clone.clone())
        .configure(".", micropb_gen::Config::new()
            .vec_type("crate::bumpalo::UnsafeVec<$T>")
            .bytes_type("crate::bumpalo::UnsafeVec::<u8>")
            .string_type("crate::bumpalo::UnsafeString")
            .map_type("BumpaloMapsNotSupported")
            // .container_lifetime("'a")
        );
    bumpalo_gen
        .compile_protos(&protos, manifest_dir.clone() + "/src/generated_bumpalo.rs")
        .unwrap();

    let mut area_gen = micropb_gen::Generator::new();
    area_gen
        .add_protoc_arg("-I./opentelemetry-proto")
        .add_protoc_arg("--experimental_allow_proto3_optional")
        .configure(".", no_clone)
        .configure(".", micropb_gen::Config::new()
            .vec_type("crate::area::AreaVec<$T>")
            .bytes_type("crate::area::AreaBytes")
            .string_type("crate::area::AreaString")
            .map_type("AreaMapsNotSupported")
        );
    area_gen
        .compile_protos(&protos, manifest_dir + "/src/generated_area.rs")
        .unwrap();
}

#[cfg(not(feature = "codegen"))]
fn main() {
    panic!("use ./regenerate.sh in project root")
}
