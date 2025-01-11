use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    get, main, post,
    web::Json,
    App, Error, HttpRequest, HttpServer, Responder,
};
use dotenv::dotenv;
use line_bot_sdk_rust::
    client::LINE
;
use serde_json::Value;
use std::env;

mod GitHub;
use GitHub::RequestBody::*;

#[post("/github")]
async fn github(
    request: HttpRequest,
    body: Json<PushRequestBody>,
) -> Result<impl Responder, Error> {
    if let Err(e) = env::var("ACCESSTOKEN") {
        return Err(ErrorInternalServerError(
            "Can't get access token for Line Client",
        ));
    }
    let client = LINE::new(env::var("ACCESSTOKEN").unwrap());

    if let Some(event) = request.headers().get("x-github-event") {
        if event != "push" {
            return Ok("Receieved");
        }
    } else {
        return Err(ErrorBadRequest("Request is not from GitHub"));
    }
    for commit in &body.commits {
        //TODO: Handle commits data and send message
    }
    println!("{:?}", body);
    Ok("Finished")
}

#[get("/")]
async fn root() -> impl Responder {
    "Hello World"
}

#[main]
async fn main() -> std::io::Result<()> {
    //Enable actix logging
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    //Load env
    dotenv().ok();

    HttpServer::new(|| App::new().service(root).service(github))
        .bind(("localhost", 3000))?
        .run()
        .await
}
