use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SongIndex {
    cid: String,
    name: String,
    artistes: Vec<String>,
}

impl SongIndex {
    pub fn get_cid(&self) -> &str {
        &self.cid
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_artistes(&self) -> &Vec<String> {
        &self.artistes
    }
}
