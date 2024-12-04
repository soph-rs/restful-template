use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct Pagination {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}
#[derive(Deserialize)]
pub struct Payload {
    pub title: String,
    pub text: String,
}
