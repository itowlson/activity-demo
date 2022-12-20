use serde::Serialize;

#[derive(Serialize)]
pub struct Webfinger {
    pub subject: String,
    pub links: Vec<Link>,
}

#[derive(Serialize)]
pub struct Link {
    pub rel: String,
    #[serde(rename = "type")]
    pub link_type: String,
    pub href: String,
}
