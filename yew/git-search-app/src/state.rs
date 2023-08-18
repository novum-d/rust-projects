use openapi::apis::Error;
use openapi::{apis::default_api::SearchRepositoriesGetError, models::Repo};

#[derive(Debug)]
pub struct State {
    pub entries: FetchState<Vec<Repo>>,
    pub keyword: String,
    pub is_light_mode: bool,
}

#[derive(Debug)]
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(Error<SearchRepositoriesGetError>),
}
