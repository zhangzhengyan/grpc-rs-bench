fn main() {
    let proto_root = "./";
    let proto_out = "src/";
    println!("cargo:rerun-if-changed={}", proto_root);
    protoc_grpcio::compile_grpc_protos(&["pingpong.proto"], &[proto_root], &proto_out, None)
        .expect("Failed to compile gRPC definitions!");

    
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/proto")
        .compile(&["pingpong.proto"], &["."])
        .unwrap();
}