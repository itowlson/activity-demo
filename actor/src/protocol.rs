use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub actor_type: String,
    pub preferred_username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    // `icon` isn't working and I'm not sure whether I messed up the format or
    // something?  Docs seem contradictory
    pub icon: String,

    pub inbox: String,
    pub public_key: ActorPublicKey,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActorPublicKey {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}
