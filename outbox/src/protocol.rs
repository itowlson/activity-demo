use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollectionSummary {
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub total_items: u32,
    pub first: String,
    pub last: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollectionPage {
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub next: String,
    pub prev: String,
    pub part_of: String,
    pub ordered_items: Vec<Activity>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub actor: String,
    pub published: String,
    pub to: Vec<String>,
    pub object: ActivityObject,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityObject {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub name: Option<String>,
    pub content: String,
    // Mastodon uses this as CW
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    pub published: String,
    pub url: String,
    pub attributed_to: String,
    pub to: Vec<String>,
}
