// This example requires the following input to succeed:
// { "command": "do something" }
mod models;

use std::env::var;
use models::{FailureResponse};
use common_libs::{Request,SuccessResponse};
use lambda_runtime::{handler_fn, Context, Error};

type Response = Result<SuccessResponse, FailureResponse>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(event: Request, ctx: Context) -> Response {
    let command = event.command;

    let mut env_str = var("ENV_STR")
        .expect("A ENV_STR must be set in this app's Lambda environment variables.");

    #[cfg(feature = "my-dev-feature")]
    {
        println!("feature is my-dev-feature.");
    }

    let resp = SuccessResponse {
        req_id: ctx.request_id,
        msg: format!("Hello from Lambda 1! The command {} executed and ENV_STR is {}", command,env_str),
    };
    Ok(resp)
}
