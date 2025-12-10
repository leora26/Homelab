use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    unsafe { std::env::set_var("PROTOC", protobuf_src::protoc()); }

    println!("cargo:rerun-if-changed=proto/nas.proto");

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(
            &[
                "proto/nas.proto",
                "proto/white_listed_user.proto",
                "proto/user.proto",
                "proto/file.proto",
                "proto/folder.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}