use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());

    println!("cargo:rerun-if-changed=proto/white_listed_user.proto");
    println!("cargo:rerun-if-changed=proto/user.proto");
    println!("cargo:rerun-if-changed=proto/file.proto");
    println!("cargo:rerun-if-changed=proto/folder.proto");
    println!("cargo:rerun-if-changed=proto/types.proto");

    tonic_build::configure()
        .compile_protos(
            &[
                "proto/white_listed_user.proto",
                "proto/user.proto",
                "proto/file.proto",
                "proto/folder.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}