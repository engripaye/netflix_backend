use serde::{Deserialize, Serialize}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Movie {
    pub id: i32,
    pub title: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub vote_average: Option<f64>,
    pub release_date: Option<String>,
    pub media_type: Option<String>,

    }

#derive(Clone, Debug, Serialize, Deserialize)]
pub struct TdmbResponse {
    pub page: i32,
    pub results: Vec<Movie>,
    pub total_pages: i32,

    }

#derive(Clone, Debug, Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    pub key: String,
    pub site: String,
    pub r#type: String,
    pub name: String,


    }

#derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoResponse {
    pub id: i32,
    pub results: Vec<Video>,
    }

// Query Params
#derive(Deserialize)]
pub struct PageQuery {
    pub page: Option<i32>,
}