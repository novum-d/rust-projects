use openapi::apis::Error;
use openapi::{apis::default_api::SearchRepositoriesGetError, models::Repo};
use serde_derive::{Deserialize, Serialize};

pub struct State {
    pub entries: FetchState<Vec<Repo>>,
    pub keyword: String,
}

pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(Error<SearchRepositoriesGetError>),
}
