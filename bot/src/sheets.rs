extern crate google_sheets4 as sheets4;

use sheets4::oauth2::ApplicationSecret;
use sheets4::{hyper, hyper_rustls, oauth2, Sheets};
use sheets4::{Error, Result};

use crate::config::AppConfig;

pub async fn login(config: AppConfig) {
    let secret: oauth2::ApplicationSecret = ApplicationSecret {
        project_id: Some("discord-bot-10-manner".to_string()),
        auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
        token_uri: "https://accounts.google.com/o/oauth2/token".to_string(),
        auth_provider_x509_cert_url: Some("https://www.googleapis.com/oauth2/v1/certs".to_string()),
        redirect_uris: vec![
            "urn:ietf:wg:oauth:2.0:oob".to_string(),
            "http://localhost".to_string(),
        ],
        client_x509_cert_url: None, // TODO: what/where this
        client_email: None,         // TODO: wat
        client_id: config.google_sheets_client_id,
        client_secret: config.google_sheets_client_secret,
    };
}
