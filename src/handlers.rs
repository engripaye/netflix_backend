use axum::{ extract ::{ path, query, state }, json};
use crate::models::{ TdmbResponse, VideoResponse, PageQuery, SearchQuery };
use crate::state::AppState;

pub async fn root() -> &'static str {
    "Netflix Backend is online!"
}

pub async fn get_trending_movies(
    State(state): State<AppState>,
    Query(query): Query<PageQuery>,
    ) -> Json<TdmbResponse> {
    let page = query.page.unwrap_or(1);
    let url = format!(

