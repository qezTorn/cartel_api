use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicResponse {
    pub user_id: i64,
    pub name: String,
    pub status: String,
    pub level: i64,
    pub reputation: f64,
    pub cartel_id: i64,
    pub cartel_name: String,
}
