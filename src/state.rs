#[derive(clone)]
pub struct AppState {
    pub tdmb_api_key: String,
    pub client: reqwest::Client,
}