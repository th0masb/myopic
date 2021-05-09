extern crate bytes;
extern crate dotenv;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

use simple_logger::SimpleLogger;

use crate::params::ApplicationParameters;

mod challenge;
mod eventprocessor;
mod events;
mod gamestart;
mod lichess;
mod params;
mod streamloop;
mod userstatus;
mod validity;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap_or_else(|e| panic!("Unable to init logger: {}", e));

    let parameters = ApplicationParameters::load()
        .unwrap_or_else(|e| panic!("Unable to load application params: {}", e));

    streamloop::stream(parameters).await
}
