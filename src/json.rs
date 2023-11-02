use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonData(pub Vec<JsonStructure>);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct JsonStructure {
    pub id: u32,
    pub name: Option<String>,
    pub organization_name: Option<String>,
    pub platform: Option<String>,
    pub repository_url: Option<String>,
    pub description: Option<String>,
    pub default_branch: Option<String>,
    pub is_fork: Option<bool>,
    pub is_archived: Option<bool>,
    pub creation_date: Option<String>,
    pub last_update: Option<String>,
    pub last_modification: Option<String>,
    pub homepage: Option<String>,
    pub stars_count: u32,
    pub forks_count: u32,
    pub license: Option<String>,
    pub open_issues_count: u32,
    pub language: Option<String>,
    pub topics: Option<String>,
    pub software_heritage_exists: Option<bool>,
    pub software_heritage_url: Option<String>,
}

pub fn read_from_file(path: &str) -> JsonData {
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_json::from_str(&contents).unwrap()
}

#[cfg(test)]
mod tests {}
