use std::{default, fmt};

use openapi::apis::Error;
use openapi::{apis::default_api::SearchRepositoriesGetError, models::Repo};

#[derive(Debug)]
pub struct State {
    pub entries: FetchState<Vec<Repo>>,
    pub keyword: String,
    pub isLightMode: bool,
}

#[derive(Debug)]
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(Error<SearchRepositoriesGetError>),
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Theme::Light => write!(f, "light"),
            Theme::Dark => write!(f, "dark"),
        }
    }
}
