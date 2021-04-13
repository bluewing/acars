mod acars_opts;
mod acars_error;
mod acars_env;
mod delivery;

use std::io::Read;
use std::env::VarError;
use crate::acars_env::AcarsEnv;

///
///
///
fn main() {
    match std::env::var(AcarsEnv::WebhookUrl.value()) {
        Ok(webhook_url) => {
            send_webhook_event(&webhook_url, configure_webhook_body())
        },
        Err(e) => {
            match e {
                VarError::NotPresent => panic!("Webhook URL not provided!"),
                VarError::NotUnicode(_) => panic!("Webhook URL was not valid unicode.")
            }
        },
    }
}

///
///
///
fn configure_webhook_body() -> String {
    let body_to_send = delivery::webhook_body::WebhookBody {
        username: String::from("ACARS"),
        channel: String::from("#devops"),
        icon_url: None,
        text: String::from("PHPUnit tests failed!")
    };
    serde_json::to_string(&body_to_send).expect("Serde serialization failed!")
}

///
///
///
fn send_webhook_event(webhook_url: &str, mut payload_as_bytes: String) {
    let mut curl = curl::easy::Easy::new();
    curl.url(webhook_url).unwrap();
    curl.post(true).unwrap();
    curl.post_field_size(payload_as_bytes.len() as u64).unwrap();

    let mut transfer = curl.transfer();
    transfer.read_function(|buf| {
        Ok(payload_as_bytes.as_bytes().read(buf).unwrap_or(0))
    }).unwrap();
    match transfer.perform() {
        Ok(_) => println!("Success."),
        Err(_) => panic!("Transfer failed.")
    }
}

