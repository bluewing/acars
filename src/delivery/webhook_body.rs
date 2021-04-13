use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WebhookBody {
    pub(crate) channel: String,
    pub(crate) username: String,
    pub(crate) icon_url: Option<String>,
    pub(crate) text: String
}
