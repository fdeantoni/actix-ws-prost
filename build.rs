fn main() {
    prost_build::compile_protos(&["src/items.proto"],
                                &["src/"]).unwrap();
}
