use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Bug {
    pub id: isize,
    pub author: String,
    pub title: String,
    pub content: String,
}
