fn main() {
    prost_build::compile_protos(&["src/IR.proto"], &["src/"]).unwrap();
}
