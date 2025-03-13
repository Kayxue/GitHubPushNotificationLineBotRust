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
use xitca_web::{
    codegen::route, error::Error, handler::json::LazyJson, http::HeaderMap, middleware::Logger, App,
};

use chrono::prelude::*;
use chrono_tz::Asia::Taipei;

mod GitHub;
use GitHub::RequestBody::*;

mod CustomError;
use CustomError::BadRequest;
use CustomError::InternalServerError;

mod Middleware;

#[route("/github",method = post)]
async fn github(
    header: HeaderMap,
    body: Option<LazyJson<PushRequestBody<'_>>>,
) -> Result<&'static str, Error> {
    if let Some(event) = header.get("x-github-event") {
        if event != "push" {
            return Ok("Receieved");
        }
    } else {
        return Err(BadRequest::new("Request is not from GitHub").into());
    }
    if let Err(_) = env::var("ACCESSTOKEN") {
        return Err(InternalServerError::new("Can't get access token for Line Client").into());
    }
    let client = LINE::new(env::var("ACCESSTOKEN").unwrap());
    if let None = body {
        return Err(BadRequest::new("Invalid request body").into());
    }
    let validBody = body.unwrap();
    let PushRequestBody { commits, .. } = validBody.deserialize()?;
    for commit in &commits {
        let request = BroadcastRequest {
            messages: vec![Message::FlexMessage(FlexMessage {
                alt_text: format!("{} pushed his/her changes", commit.author.name).to_owned(),
                contents: Box::new(FlexContainer::FlexBubble(FlexBubble {
                    body: Some(Box::new(FlexBox{
                        r#type: Some("box".to_owned()),
                        layout:Layout::Vertical,
                        contents:vec![
                            FlexComponent::FlexText(FlexText {
                                text: Some("Commit Pushed".to_owned()),
                                weight: Some(Weight::Bold),
                                size: Some("xl".to_owned()),
                                wrap: Some(true),
                                ..Default::default()
                            }),
                            FlexComponent::FlexBox(FlexBox {
                                layout: Layout::Vertical,
                                margin: Some("lg".to_owned()),
                                spacing: Some("sm".to_owned()),
                                contents: vec![
                                    FlexComponent::FlexText(FlexText {
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
                                    FlexComponent::FlexBox(FlexBox {
                                        layout: Layout::Baseline,
                                        spacing: Some("sm".to_owned()),
                                        contents: vec![
                                            FlexComponent::FlexText(FlexText {
                                                text: Some("ID".to_owned()),
                                                color: Some("#aaaaaa".to_owned()),
                                                size: Some("sm".to_owned()),
                                                flex: Some(2),
                                                ..Default::default()
                                            }),
                                            FlexComponent::FlexText(FlexText {
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
                                    FlexComponent::FlexBox(FlexBox {
                                        layout: Layout::Baseline,
                                        spacing: Some("sm".to_owned()),
                                        contents: vec![
                                            FlexComponent::FlexText(FlexText {
                                                text: Some("Committer".to_owned()),
                                                color: Some("#aaaaaa".to_owned()),
                                                size: Some("sm".to_owned()),
                                                flex: Some(0),
                                                wrap: Some(true),
                                                ..Default::default()
                                            }),
                                            FlexComponent::FlexText(FlexText {
                                                text: Some(commit.author.name.to_owned()),
                                                color: Some("#666666".to_owned()),
                                                size: Some("sm".to_owned()),
                                                flex: Some(5),
                                                ..Default::default()
                                            }),
                                        ],
                                        ..Default::default()
                                    }),
                                    FlexComponent::FlexBox(FlexBox {
                                        layout: Layout::Baseline,
                                        spacing: Some("sm".to_owned()),
                                        contents: vec![
                                            FlexComponent::FlexText(FlexText {
                                                text: Some("Message".to_owned()),
                                                color: Some("#aaaaaa".to_owned()),
                                                size: Some("sm".to_owned()),
                                                flex: Some(1),
                                                wrap: Some(true),
                                                ..Default::default()
                                            }),
                                            FlexComponent::FlexText(FlexText {
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
                                    FlexComponent::FlexBox(FlexBox {
                                        layout: Layout::Baseline,
                                        spacing: Some("sm".to_owned()),
                                        contents: vec![
                                            FlexComponent::FlexText(FlexText {
                                                text: Some("Time".to_owned()),
                                                color: Some("#aaaaaa".to_owned()),
                                                size: Some("sm".to_owned()),
                                                flex: Some(1),
                                                wrap: Some(true),
                                                ..Default::default()
                                            }),
                                            FlexComponent::FlexText(FlexText {
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
                    ..Default::default()
                })),
                    footer: Some(Box::new(FlexBox {
                        r#type: Some("box".to_owned()),
                        layout: Layout::Vertical,
                        spacing: Some("sm".to_owned()),
                        contents: vec![FlexComponent::FlexButton(FlexButton {
                            style: Some(Style::Link),
                            height: Some(Height::Sm),
                            action: Box::new(Action::UriAction(UriAction {
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
        let _result = client.messaging_api_client.broadcast(request, None).await;
        // match _result {
        //     Ok(r) => println!("OK{:#?}", r),
        //     Err(e) => println!("Error{:#?}", e),
        // }
    }
    Ok("Finished")
}

#[route("/",method = get)]
async fn root() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //Load env
    dotenv().ok();

    App::new()
        .at_typed(root)
        .at_typed(github)
        .enclosed_fn(Middleware::error_handler)
        .enclosed(Logger::new())
        .serve()
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
