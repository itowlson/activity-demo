use anyhow::{anyhow, Result};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use crate::protocol::{Webfinger, Link};

mod protocol;

/// A simple Spin HTTP component.
#[http_component]
fn webfinger(req: Request) -> Result<Response> {
    eprintln!("[{}] Webfinger request for {}", chrono::Utc::now().format("%d/%m:%T"), req.uri());

    let host = host(&req)?;
    let preferred_username = spin_sdk::config::get("username")?;

    let subject = format!("acct:{preferred_username}@{host}");

    let requested_resource = query(&req, "resource")?;
    if requested_resource != subject {
        eprintln!("Webfinger rejecting {}", requested_resource);
        return Ok(http::Response::builder().status(404).body(None)?);
    }

    let webfinger = Webfinger {
        subject,
        links: vec![
            Link {
                rel: "self".into(),
                link_type: "application/activity+json".into(),
                href: format!("https://{host}/actor"),
            }
        ],
    };

    let response = serde_json::to_vec_pretty(&webfinger)?;
    Ok(http::Response::builder()
        .status(200)
        .body(Some(response.into()))?)
}

fn full_url(req: &Request) -> Result<String> {
    for (key, value) in req.headers() {
        if key == "spin-full-url" {
            return Ok(value.to_str()?.into());
        }
    }
    Err(anyhow!("Cannot get URL - spin-full-url header not found"))
}

fn host(req: &Request) -> Result<String> {
    let url = full_url(req)?;
    let uri = http::Uri::try_from(&url)?;
    let host = uri.authority().ok_or(anyhow!("Cannot get host - spin-full-url header not a URI"))?;
    Ok(host.to_string())
}

fn query(req: &Request, key: &str) -> Result<String> {
    let url = full_url(req)?;
    let url = url::Url::parse(&url)?;
    for (key, value) in url.query_pairs() {
        if key == key {
            return Ok(value.into());
        }
    }
    Err(anyhow!("Cannot get {key} from query string"))
}
