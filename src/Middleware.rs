use xitca_web::{
    error::Error,
    handler::Responder,
    http::{StatusCode, WebResponse},
    service::Service,
    WebContext,
};

use crate::CustomError::BadRequest;
use std::error;

pub async fn error_handler<S, C>(s: &S, mut ctx: WebContext<'_, C>) -> Result<WebResponse, Error>
where
    C: 'static,
    S: for<'r> Service<WebContext<'r, C>, Response = WebResponse, Error = Error>,
{
    match s.call(ctx.reborrow()).await {
        Ok(res) => Ok(res),
        Err(e) => {
            // generate http response actively. from here it's OK to early return it in Result::Ok
            // variant as error_handler function's output
            let _res = e.call(ctx.reborrow()).await?;
            // return Ok(_res);

            // upcast trait and downcast to concrete type again.
            // this offers the ability to regain typed error specific error handling.
            // *. this is a runtime feature and not reinforced at compile time.
            if let Some(_e) = e.upcast().downcast_ref::<BadRequest>() {
                return (_e.message.to_owned(), StatusCode::BAD_REQUEST)
                    .respond(ctx)
                    .await;
            }

            // below are error handling feature only enabled by using nightly rust.

            // utilize std::error module for backtrace and more advanced error info.
            let report = error::Report::new(&e).pretty(true).show_backtrace(true);
            // display error report
            println!("{report}");

            // the most basic error handling is to ignore it and return as is. xitca-web is able to take care
            // of error by utilizing it's according trait implements(Debug,Display,Error and Service impls)
            Err(e)
        }
    }
}
