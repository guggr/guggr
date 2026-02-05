use anyhow::Result;
use scheduler::adapters::outbound::rabbitmq::RabbitMQPublisher;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .json()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let config = todo!();

    let publisher = RabbitMQPublisher::new(todo!(), todo!())?;
    publisher.setup_schema().await?;

    Ok(())
}
