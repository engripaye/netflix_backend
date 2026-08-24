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