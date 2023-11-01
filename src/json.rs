use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonData(Vec<JsonStructure>);


/// Example :
/// 
/// ```json
///{
/// "id": 27891,
/// "name": "ribbit",
/// "organization_name": "ribbit",
/// "platform": "GitLab",
/// "repository_url": "https://gitlab.inria.fr/ribbit/ribbit",
/// "description": "🐸 https://ribbit.gitlabpages.inria.fr/ribbit/ 🐸",
/// "default_branch": "master",
/// "is_fork": null,
/// "is_archived": false,
/// "creation_date": "2021-02-21T16:52:46Z",
/// "last_update": "2023-10-26T13:54:39Z",
/// "last_modification": "2023-10-26T13:54:39Z",
/// "homepage": null,
/// "stars_count": 0,
/// "forks_count": 1,
/// "license": null,
/// "open_issues_count": 0,
/// "language": null,
/// "topics": "",
/// "software_heritage_exists": false,
/// "software_heritage_url": null
///}
///```
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct JsonStructure {
  pub id: u32,
  pub name: String,
  pub organization_name: String,
  pub platform: String,
  pub repository_url: String,
  pub description: String,
  pub default_branch: String,
  pub is_fork: bool,
  pub is_archived: bool,
  pub creation_date: String,
  pub last_update: String,
  pub last_modification: String,
  pub homepage: String,
  pub stars_count: u32,
  pub forks_count: u32,
  pub license: String,
  pub open_issues_count: u32,
  pub language: String,
  pub topics: String,
  pub software_heritage_exists: bool,
  pub software_heritage_url: String,
}


#[cfg(test)]
mod tests {

}