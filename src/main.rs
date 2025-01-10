use actix_web::{
    error::ErrorBadRequest, get, main, post, web::Json, App, Error, HttpRequest, HttpServer,
    Responder,
};
use serde_json::Value;
use std::env;

#[post("/github")]
async fn github(request: HttpRequest, body: Json<Value>) -> Result<impl Responder, Error> {
    if let Some(event) = request.headers().get("x-github-event") {
        if event != "push" {
            return Ok("Receieved");
        }
    } else {
        return Err(ErrorBadRequest("Request is not from GitHub"));
    }
    if let Some(commits) = body["commits"].as_array() {
        for commit in commits {
            //TODO: Handle commits data and send message
        }
    } else {
        return Err(ErrorBadRequest("Can't extract commits from body"));
    }
    Ok("Finished")
}

#[get("/")]
async fn root() -> impl Responder {
    "Hello World"
}

#[main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| App::new().service(root).service(github))
        .bind(("localhost", 3000))?
        .run()
        .await
}
