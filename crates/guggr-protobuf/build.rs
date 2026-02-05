use std::{fs, path::PathBuf};

use protocheck_build::{compile_protos_with_validators};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../../proto/");
    let mut config = prost_build::Config::default();

    config.out_dir("src/gen/proto");

    // Create output directory
    fs::create_dir_all("src/gen/proto").expect("could not create `src/gen/proto` folder");

    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let proto_dir = root.join("../../proto");

    // Proto files
    let proto_job = proto_dir.join("job").join("job.proto");
    let proto_http_job_type = proto_dir.join("job").join("types").join("http.proto");
    let proto_ping_job_type = proto_dir.join("job").join("types").join("ping.proto");

    let proto_result = proto_dir.join("result").join("result.proto");
    let proto_http_result_type = proto_dir.join("result").join("types").join("http.proto");
    let proto_ping_result_type = proto_dir.join("result").join("types").join("ping.proto");

    let proto_files = &[
            proto_job,
            proto_http_job_type,
            proto_ping_job_type,
            proto_result,
            proto_http_result_type,
            proto_ping_result_type,
        ];

    compile_protos_with_validators(
        &mut config,
        &proto_files.clone(),
        &[&proto_dir],
        &["guggr.v1"],
    )?;

    config.compile_protos(proto_files, &[proto_dir])?;

    Ok(())
}
