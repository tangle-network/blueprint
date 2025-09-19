fn main() {
    // Generate test gRPC service definitions for integration tests.
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_protos(&["proto/grpc_proxy_test.proto"], &["proto"])
        .expect("failed to build gRPC test protos");
}
