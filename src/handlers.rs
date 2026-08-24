use axum::{ extract ::{ path, query, state }, json};
use crate::models::{ TdmbResponse, VideoResponse, PageQuery, SearchQuery };
use crate::state::AppState;

pub async fn root() -> &'static str {
    "Netflix Backend is online!"
}
