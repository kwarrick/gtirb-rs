fn main() {
    prost_build::compile_protos(&["src/proto/IR.proto"], &["src/proto/"])
        .unwrap();
}
