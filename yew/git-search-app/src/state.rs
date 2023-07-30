use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use yew::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub entries: Vec<Entry>,
    pub filter: Filter,
    pub edit_value: String,
}
