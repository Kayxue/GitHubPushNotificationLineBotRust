use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod RequestBody {

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PushRequestBody {
        pub after: String,
        pub base_ref: Option<String>,
        pub before: String,
        pub commits: Vec<Commit>,
        pub compare: String,
        pub created: bool,
        pub deleted: bool,
        pub enterprise: Option<Value>,
        pub forced: bool,
        pub head_commit: Option<Commit>,
        pub installation: Option<Value>,
        pub organization: Option<Value>,
        pub pusher: User,
        #[serde(rename = "ref")]
        pub refs: String,
        pub repository: Value,
        pub sender: Option<Value>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Commit {
        pub added: Vec<String>,
        pub author: User,
        pub committer: User,
        pub distinct: bool,
        pub id: String,
        pub message: String,
        pub modified: Vec<String>,
        pub removed: Vec<String>,
        pub timestamp: String,
        pub tree_id: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct User {
        pub date: Option<String>,
        pub name: String,
        pub email: Option<String>,
        pub username: Option<String>,
    }
}
