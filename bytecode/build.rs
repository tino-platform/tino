extern crate capnpc;

fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("src/capnp")
        .file("src/capnp/trussfile.capnp")
        .run()
        .expect("compiling schema");
}
