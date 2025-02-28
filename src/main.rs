use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    get, main, post,
    web::Json,
    App, Error, HttpRequest, HttpServer, Responder,
};
use dotenv::dotenv;
use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{
            flex_box::Layout,
            flex_button::{Height, Style},
            flex_text::Weight,
            Action, BroadcastRequest, FlexBox, FlexBubble, FlexButton, FlexComponent,
            FlexContainer, FlexMessage, FlexText, Message, UriAction,
        },
    },
};
use std::env;

use chrono::prelude::*;
use chrono_tz::Asia::Taipei;

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
        let request = BroadcastRequest {
            messages: vec![Message::Flex(FlexMessage {
                alt_text: format!("{} pushed his/her changes", commit.author.name).to_owned(),
                contents: Box::new(FlexContainer::Bubble(FlexBubble {
                    body: Some(Box::new(FlexBox::new(
                        Layout::Vertical,
                        vec![
                            FlexComponent::Text(FlexText {
                                text: Some("Commit Pushed".to_owned()),
                                weight: Some(Weight::Bold),
                                size: Some("xl".to_owned()),
                                wrap: Some(true),
                                ..Default::default()
                            }),
                            FlexComponent::Box(FlexBox {
                                layout: Layout::Vertical,
                                margin: Some("lg".to_owned()),
                                spacing: Some("sm".to_owned()),
                                contents: vec![
                                    FlexComponent::Text(FlexText {
                                        text: Some(
                                            format!(
                                                "{} pushed his/her change to the repo",
                                                commit.author.name
                                            )
                                            .to_owned(),
                                        ),
                                        wrap: Some(true),
                                        ..Default::default()
                                    }),
                                    FlexComponent::Box(FlexBox {
                                        layout: Layout::Baseline,
                                        spacing: Some("sm".to_owned()),
                                        contents: vec![
                                            FlexComponent::Text(FlexText {
                                                text: Some("ID".to_owned()),
                                                color: Some("#aaaaaa".to_owned()),
                                                size: Some("sm".to_owned()),
                                                flex: Some(2),
                                                ..Default::default()
                                            }),
                                            FlexComponent::Text(FlexText {
                                                text: Some(commit.id[..7].to_owned()),
                                                wrap: Some(false),
                                                color: Some("#666666".to_owned()),
                                                size: Some("sm".to_owned()),
                                                flex: Some(6),
                                                ..Default::default()
                                            }),
                                        ],
                                        ..Default::default()
                                    }),
                                    FlexComponent::Box(FlexBox {
                                        layout: Layout::Baseline,
                                        spacing: Some("sm".to_owned()),
                                        contents: vec![
                                            FlexComponent::Text(FlexText {
                                                text: Some("Committer".to_owned()),
                                                color: Some("#aaaaaa".to_owned()),
                                                size: Some("sm".to_owned()),
                                                flex: Some(0),
                                                wrap: Some(true),
                                                ..Default::default()
                                            }),
                                            FlexComponent::Text(FlexText {
                                                text: Some(commit.author.name.to_owned()),
                                                color: Some("#666666".to_owned()),
                                                size: Some("sm".to_owned()),
                                                flex: Some(5),
                                                ..Default::default()
                                            }),
                                        ],
                                        ..Default::default()
                                    }),
                                    FlexComponent::Box(FlexBox {
                                        layout: Layout::Baseline,
                                        spacing: Some("sm".to_owned()),
                                        contents: vec![
                                            FlexComponent::Text(FlexText {
                                                text: Some("Message".to_owned()),
                                                color: Some("#aaaaaa".to_owned()),
                                                size: Some("sm".to_owned()),
                                                flex: Some(1),
                                                wrap: Some(true),
                                                ..Default::default()
                                            }),
                                            FlexComponent::Text(FlexText {
                                                text: Some(commit.message.to_owned()),
                                                color: Some("#666666".to_owned()),
                                                size: Some("sm".to_owned()),
                                                flex: Some(3),
                                                wrap: Some(false),
                                                ..Default::default()
                                            }),
                                        ],
                                        ..Default::default()
                                    }),
                                    FlexComponent::Box(FlexBox {
                                        layout: Layout::Baseline,
                                        spacing: Some("sm".to_owned()),
                                        contents: vec![
                                            FlexComponent::Text(FlexText {
                                                text: Some("Time".to_owned()),
                                                color: Some("#aaaaaa".to_owned()),
                                                size: Some("sm".to_owned()),
                                                flex: Some(1),
                                                wrap: Some(true),
                                                ..Default::default()
                                            }),
                                            FlexComponent::Text(FlexText {
                                                text: Some(
                                                    commit
                                                        .timestamp
                                                        .parse::<DateTime<Utc>>()
                                                        .unwrap()
                                                        .with_timezone(&Taipei)
                                                        .format("%Y/%m/%d %H:%M:%S")
                                                        .to_string(),
                                                ),
                                                color: Some("#666666".to_owned()),
                                                size: Some("sm".to_owned()),
                                                flex: Some(3),
                                                wrap: Some(true),
                                                ..Default::default()
                                            }),
                                        ],
                                        ..Default::default()
                                    }),
                                ],
                                ..Default::default()
                            }),
                        ],
                    ))),
                    footer: Some(Box::new(FlexBox {
                        r#type: Some("box".to_owned()),
                        layout: Layout::Vertical,
                        spacing: Some("sm".to_owned()),
                        contents: vec![FlexComponent::Button(FlexButton {
                            style: Some(Style::Link),
                            height: Some(Height::Sm),
                            action: Box::new(Action::Uri(UriAction {
                                label: Some("Check the commit".to_owned()),
                                uri: Some(commit.url.to_owned()),
                                ..Default::default()
                            })),
                            ..Default::default()
                        })],
                        ..Default::default()
                    })),
                    ..Default::default()
                })),
                ..Default::default()
            })],
            notification_disabled: Some(false),
        };
        // let res = serde_json::to_string_pretty(&request.messages[0])
        //     .map_err(|e| ErrorBadRequest(e.to_string()))?;
        // println!("res: {res}");
        let result = client.messaging_api_client.broadcast(request, None).await;
        // match result {
        //     Ok(r) => println!("{:#?}", r),
        //     Err(e) => println!("{:#?}", e),
        // }
    }
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
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
