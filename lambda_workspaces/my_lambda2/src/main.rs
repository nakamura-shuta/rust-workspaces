// This example requires the following input to succeed:
// { "command": "do something" }
mod models;

use common_libs::{Request, SuccessResponse};
use models::Foo;

use anyhow::{Result};
use lambda_runtime::{handler_fn, Context as LambdaContext, Error};

type Response = Result<SuccessResponse>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(event: Request, ctx: LambdaContext) -> Response {
    // extract some useful info from the request
    let command = event.command;

    let f = Foo {
        msg: "Foo Struct".into(),
    };
    println!("{:#?}", f);
    // prepare the response
    let resp = SuccessResponse {
        req_id: ctx.request_id,
        msg: format!("Hello from Lambda 2! The command {} executed.", command),
    };
    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}
