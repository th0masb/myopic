use lambda_runtime::{service_fn, Error, LambdaEvent};
use simple_logger::SimpleLogger;


#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .without_timestamps()
        .init()?;
    lambda_runtime::run(service_fn(game_handler)).await?;
    Ok(())
}

async fn game_handler(event: LambdaEvent<()>) -> Result<(), Error> {
    Ok(())
}