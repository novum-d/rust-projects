use openapi::apis::Error;
use openapi::{apis::default_api::SearchRepositoriesGetError, models::Repo};

#[derive(Debug)]
pub struct State {
    pub entries: FetchState<Vec<Repo>>,
    pub keyword: String,
}

#[derive(Debug)]
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(Error<SearchRepositoriesGetError>),
}
