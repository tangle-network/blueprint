fn main() {
    // Use bundled protoc to ensure CI compatibility
    // SAFETY: This is a build script, and we're setting PROTOC before any proto compilation
    unsafe {
        std::env::set_var("PROTOC", protobuf_src::protoc());
    }

    // Generate test gRPC service definitions for integration tests.
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_protos(&["proto/grpc_proxy_test.proto"], &["proto"])
        .expect("failed to build gRPC test protos");
}
