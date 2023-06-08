use std::path::PathBuf;

use re_build_tools::{is_tracked_env_var_set, write_file_if_necessary};

fn main() -> Result<(), std::io::Error> {
    if std::env::var("CI").is_ok() {
        // No need to run this on CI (which means setting up `protoc` etc) since the code is committed
        // anyway.
        return Ok(());
    }
    if !is_tracked_env_var_set("IS_IN_RERUN_WORKSPACE") {
        // Only run if we are in the rerun workspace, not on users machines (if we ever publish the example).
        return Ok(());
    }

    match protoc_prebuilt::init("22.0") {
        Ok((protoc_bin, _)) => {
            std::env::set_var("PROTOC", protoc_bin);
        }
        Err(err) => {
            eprintln!("Failed to install protoc: {err} - falling back to system 'protoc'");
        }
    }

    prost_build::compile_protos(
        &[
            "proto/a_r_capture_metadata.proto",
            "proto/annotation_data.proto",
            "proto/object.proto",
        ],
        &["proto"],
    )?;

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let src_path = PathBuf::from(out_dir).join("objectron.proto.rs");
    let dst_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/objectron.rs");

    // `include!()` will break LSP & GitHub navigation, so create an actual source file to make the
    // UX reasonable.

    let bytes = [
        b"// This file was autogenerated by `build.rs`. Do not edit.\n\n".to_vec(),
        b"#![allow(clippy::all, clippy::doc_markdown)]\n".to_vec(),
        std::fs::read(src_path)?,
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    // `cargo` has an implicit `rerun-if-changed=src/**` clause, which will act against us in this
    // instance.
    // Make sure to _not_ rewrite identical data, so as to avoid being stuck in an infinite build
    // loop when using tools like e.g. `bacon` that watch the filesystem for any changes to the
    // project's files.
    write_file_if_necessary(dst_path, &bytes)
}
