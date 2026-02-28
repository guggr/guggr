use std::{fs::File, io::Write};

use api_service::init_app;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let path = if args.len() >= 2 {
        args[1].clone()
    } else {
        "api-service/openapi.json".to_owned()
    };

    let (_, openapi_spec) = init_app(None, None, None);

    let mut file = File::create(path)?;
    file.write_all(openapi_spec.to_pretty_json()?.as_bytes())?;

    Ok(())
}
