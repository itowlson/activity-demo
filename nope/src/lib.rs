use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn nope(req: Request) -> Result<Response> {
    eprintln!("[{}] Umatched request, uri = '{}'", chrono::Utc::now().format("%d/%m:%T"), req.uri());
    Ok(http::Response::builder()
        .status(404)
        .body(None)?)
}
