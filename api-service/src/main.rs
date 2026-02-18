use api_service::telemetry::init_tracing;

fn main() {
    init_tracing();
    println!("Hello, world!");
}
