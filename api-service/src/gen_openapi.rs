use std::{fs::File, io::Write};

use api_service::init_app_openapi;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let path = if args.len() >= 2 {
        args[1].clone()
    } else {
        "api-service/openapi.json".to_owned()
    };

    let (_, a) = init_app_openapi(None, None, false);

    let mut file = File::create(path)?;
    file.write_all(a.to_pretty_json()?.as_bytes())?;

    Ok(())
}
