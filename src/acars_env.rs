pub enum AcarsEnv {
    WebhookUrl
}

impl AcarsEnv {
    pub(crate) fn value(&self) -> &str {
        match *self {
            AcarsEnv::WebhookUrl => "WEBHOOK_URL"
        }
    }
}
