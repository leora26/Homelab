use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());

    let proto_root = PathBuf::from("../../backend/homelab_back/libs/proto/proto");

    let protos = &[
        proto_root.join("types.proto"),
        proto_root.join("file.proto"),
        proto_root.join("folder.proto"),
        proto_root.join("user.proto"),
        proto_root.join("label.proto"),
        proto_root.join("file_label.proto"),
        proto_root.join("global_file.proto"),
        proto_root.join("white_listed_user.proto"),
    ];

    for proto in protos {
        println!("cargo:rerun-if-changed={}", proto.display());
    }

    tonic_build::configure()
        .build_server(false) // Optimization: We don't need server code in the client
        .compile_protos(protos, &[proto_root])?;

    tauri_build::build();

    Ok(())
}
