extern crate google_sheets4 as sheets;
use sheets::{
    api::{Spreadsheet, SpreadsheetProperties},
    oauth2, Sheets,
};

pub struct SheetsManager {
    hub: Sheets,
}

impl SheetsManager {
    pub async fn initialise(&self, sheet_name: String) {
        // Create a sheet if it does not exist
        Self::create_sheet_if_not_existing(self, sheet_name).await;
    }

    async fn create_sheet_if_not_existing(&self, sheet_name: String) {
        // self.hub.spreadsheets().get(spreadsheet_id)
        println!("Creating new spreadsheet with name {sheet_name}");

        let mut properties = SpreadsheetProperties::default();
        properties.title = Some(sheet_name);

        let mut req = Spreadsheet::default();
        req.properties = Some(properties);

        if let Ok((response, new_sheet)) = self.hub.spreadsheets().create(req).doit().await {
            println!("sheet {:?}", new_sheet);
            println!("response {:?}", response);
        }
    }
}

pub async fn login(
    google_sheets_client_id: String,
    google_sheets_client_secret: String,
) -> SheetsManager {
    let secret: oauth2::ApplicationSecret = oauth2::ApplicationSecret {
        project_id: Some("discord-bot-10-manner".to_string()),
        auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
        token_uri: "https://accounts.google.com/o/oauth2/token".to_string(),
        auth_provider_x509_cert_url: Some("https://www.googleapis.com/oauth2/v1/certs".to_string()),
        redirect_uris: vec![
            "urn:ietf:wg:oauth:2.0:oob".to_string(),
            "http://localhost".to_string(),
            "http://127.0.0.1:34767".to_string(),
        ],
        client_x509_cert_url: None, // TODO: what/where this
        client_email: None,         // TODO: wat
        client_id: google_sheets_client_id,
        client_secret: google_sheets_client_secret,
    };

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .unwrap();

    let sheets_client = Sheets::new(
        sheets::hyper::Client::builder().build(
            sheets::hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        auth,
    );
    SheetsManager { hub: sheets_client }
}
