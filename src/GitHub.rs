use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod RequestBody {

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PushRequestBody<'a> {
        pub after: &'a str,
        pub base_ref: Option<&'a str>,
        pub before: &'a str,
        pub commits: Vec<Commit<'a>>,
        pub compare: &'a str,
        pub created: bool,
        pub deleted: bool,
        pub enterprise: Option<Value>,
        pub forced: bool,
        pub head_commit: Option<Commit<'a>>,
        pub installation: Option<Value>,
        pub organization: Option<Value>,
        pub pusher: User<'a>,
        #[serde(rename = "ref")]
        pub refs: &'a str,
        pub repository: Value,
        pub sender: Option<Value>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Commit<'a> {
        pub added: Vec<&'a str>,
        pub author: User<'a>,
        pub committer: User<'a>,
        pub distinct: bool,
        pub id: &'a str,
        pub message: &'a str,
        pub modified: Vec<&'a str>,
        pub removed: Vec<&'a str>,
        pub timestamp: &'a str,
        pub tree_id: &'a str,
        pub url: &'a str,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct User<'a> {
        pub date: Option<&'a str>,
        pub name: &'a str,
        pub email: Option<&'a str>,
        pub username: Option<&'a str>,
    }
}
