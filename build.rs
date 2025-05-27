fn main() {
    #[cfg(target_os = "macos")]
    {
        tonic_build::configure()
            .build_server(true)
            //.build_client(true)
            .out_dir("src/proto")
            .compile_protos(&["sparkplug_b.proto"], &["."]) // 编译 proto 文件
            .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    }
}
