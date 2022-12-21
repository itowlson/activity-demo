use anyhow::{anyhow, Result};
use http::Uri;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use crate::protocol::{Actor, ActorPublicKey, IconLink};

mod protocol;

/// A simple Spin HTTP component.
#[http_component]
fn actor(req: Request) -> Result<Response> {
    eprintln!("[{}] Actor request for {}", chrono::Utc::now().format("%d/%m:%T"), req.uri());

    let host = host(&req)?;
    let preferred_username = spin_sdk::config::get("username")?;
    let public_key_pem = spin_sdk::config::get("public_key")?;

    let actor = Actor {
        context: vec![
            "https://www.w3.org/ns/activitystreams".into(),
            "https://w3id.org/security/v1".into(),
        ],
        id: format!("https://{host}/actor"),
        actor_type: "Person".into(),
        preferred_username,
        // TODO: derive the next two from config or other data source
        name: Some("Yes That Alice".into()),
        summary: Some("Just trying to get a message through to Bob".into()),
        icon: IconLink {
            url: format!("https://{host}/static/profile.png"),
            media_type: "image/png".into(),
        },
        inbox: format!("https://{host}/inbox"),
        outbox: Some(format!("https://{host}/outbox")),
        followers: None, // Some(format!("https://{host}/followers")),
        following: None, // Some(format!("https://{host}/following")),
        public_key: ActorPublicKey {
            id: format!("https://{host}/actor#main-key"),
            owner: format!("https://{host}/actor"),
            public_key_pem
        }
    };

    let response = serde_json::to_vec_pretty(&actor)?;
    Ok(http::Response::builder()
        .status(200)
        .body(Some(response.into()))?)
}

fn host(req: &Request) -> Result<String> {
    for (key, value) in req.headers() {
        if key == "spin-full-url" {
            let uri = Uri::try_from(value.to_str()?)?;
            let host = uri.authority().ok_or(anyhow!("Cannot get host - spin-full-url header not a URI"))?;
            return Ok(host.to_string());
        }
    }
    Err(anyhow!("Cannot get host - spin-full-url header not found"))
}
