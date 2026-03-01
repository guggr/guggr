use std::{env, path::PathBuf};

use prost_build::Config;
use protify_build::set_up_validators;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../../proto/");

    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let proto_dir = root.join("../../proto");

    // Proto files
    let proto_job = proto_dir.join("job").join("job.proto");
    let proto_http_job_type = proto_dir.join("job").join("types").join("http.proto");
    let proto_ping_job_type = proto_dir.join("job").join("types").join("ping.proto");

    let proto_result = proto_dir.join("result").join("result.proto");
    let proto_http_result_type = proto_dir.join("result").join("types").join("http.proto");
    let proto_ping_result_type = proto_dir.join("result").join("types").join("ping.proto");

    let proto_job_types = proto_dir.join("job_types").join("job_types.proto");

    let proto_files: Vec<PathBuf> = vec![
        proto_job,
        proto_http_job_type,
        proto_ping_job_type,
        proto_result,
        proto_http_result_type,
        proto_ping_result_type,
        proto_job_types,
    ];

    let out_dir = "src";
    let descriptor_path = "src/file_descriptor_set.bin";

    let mut config = Config::new();
    config
        .file_descriptor_set_path(descriptor_path)
        // Required if you use `bytes` fields anywhere
        .bytes(["."])
        .out_dir(out_dir);

    set_up_validators(&mut config, &proto_files, &[&proto_dir], &["guggr.v1"])?;

    config.compile_protos(&proto_files, &[proto_dir])?;

    Ok(())
}
