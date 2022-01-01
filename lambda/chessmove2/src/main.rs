use lambda_runtime::{handler_fn, Context, Error};
use lambda_payloads::chessmove2::*;
use myopic_brain::anyhow;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: ChooseMoveEvent, _: Context) -> Result<ChooseMoveOutput, Error> {
    todo!()
//    let first_name = event["firstName"].as_str().unwrap_or("world");
//
//    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}

