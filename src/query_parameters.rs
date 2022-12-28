use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    pub q: String,
}