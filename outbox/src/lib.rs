use anyhow::{anyhow, Result};
use http::Method;
use protocol::{OrderedCollectionPage, Activity, OrderedCollectionSummary, ActivityObject};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

mod protocol;

/// A simple Spin HTTP component.
#[http_component]
fn outbox(req: Request) -> Result<Response> {
    let body = req.body().as_ref().map(|b| String::from_utf8_lossy(&b)).unwrap_or("[none]".into());
    eprintln!("[{}] Outbox request for {}, verb = '{:?}', body = '{}'", chrono::Utc::now().format("%d/%m:%T"), req.uri(), req.method(), body);

    match *req.method() {
        Method::GET => get_outbox(&req),
        _ => method_not_allowed(),
    }
}

fn get_outbox(req: &Request) -> Result<Response> {
    match query(req, "page") {
        Ok(s) if s == "true" => get_outbox_paged(req),  // TODO: worry about multiple pages
        _ => get_outbox_summary(req),
    }
}
fn get_outbox_summary(req: &Request) -> Result<Response> {
    let host = host(req)?;

    let summary = OrderedCollectionSummary {
        context: "https://www.w3.org/ns/activitystreams".into(),
        id: format!("https://{host}/outbox"),
        object_type: "OrderedCollection".into(),
        total_items: 2,
        first: format!("https://{host}/outbox?page=true"),
        last: format!("https://{host}/outbox?page=true"),  // TODO
    };

    let response = serde_json::to_vec_pretty(&summary)?;
    Ok(http::Response::builder()
        .status(200)
        .body(Some(response.into()))?)
}

fn get_outbox_paged(req: &Request) -> Result<Response> {
    let host = host(req)?;
    let actor_url = format!("https://{host}/actor");
    // let followers_url = format!("https://{host}/followers");

    let posts = OrderedCollectionPage {
        context: "https://www.w3.org/ns/activitystreams".into(),
        id: format!("https://{host}/outbox?page=true"),
        object_type: "OrderedCollectionPage".into(),
        next: format!("https://{host}/outbox"),
        prev: format!("https://{host}/outbox"),
        part_of: format!("https://{host}/outbox"),
        ordered_items: vec![
            Activity {
                id: format!("https://{host}/FAKE-ID/{}", "fie"),
                item_type: "Create".into(),
                actor: actor_url.clone(),
                published: "2022-12-19T10:11:12Z".into(),
                to: vec![
                    "https://www.w3.org/ns/activitystreams#Public".into(),
                    // followers_url.clone()
                ],
                object: ActivityObject {
                    id: format!("https://{host}/FAKE-ID/{}", "fie"),
                    item_type: "Note".into(),
                    name: Some("[NAME] Has anyone seen Bob?  Not you Eve".into()),
                    content: "Has anyone seen Bob?  Not you Eve".into(),
                    summary: None,
                    published: "2022-12-19T10:11:12Z".into(),
                    attributed_to: actor_url.clone(),
                    url: format!("https://{host}/FAKE-FAKE-FAKE"),
                    to: vec![
                        "https://www.w3.org/ns/activitystreams#Public".into(),
                        // followers_url.clone()
                    ],
                }
            },
            Activity {
                id: format!("https://{host}/FAKE-ID/{}", "arse"),
                item_type: "Create".into(),
                actor: actor_url.clone(),
                published: "2022-12-20T02:03:04Z".into(),
                to: vec![
                    "https://www.w3.org/ns/activitystreams#Public".into(),
                    // followers_url.clone()
                ],
                object: ActivityObject {
                    id: format!("https://{host}/FAKE-ID/{}", "arse"),
                    item_type: "Note".into(),
                    name: Some("[NAME] That Mallory is so naughty".into()),
                    content: "That Mallory is so naughty".into(),
                    summary: None,
                    published: "2022-12-20T02:03:04Z".into(),
                    attributed_to: actor_url.clone(),
                    url: format!("https://{host}/FAKE-FAKE-FAKE"),
                    to: vec![
                        "https://www.w3.org/ns/activitystreams#Public".into(),
                        // followers_url.clone()
                    ],
                }
            },
        ]
    };

    let response = serde_json::to_vec_pretty(&posts)?;
    Ok(http::Response::builder()
        .status(200)
        .body(Some(response.into()))?)
}

fn method_not_allowed() -> Result<Response> {
    Ok(http::Response::builder()
        .status(405)
        .body(None)?)
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
