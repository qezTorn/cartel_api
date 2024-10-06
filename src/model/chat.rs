use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatResponse {
    pub global_chat: Vec<GlobalChat>,
    pub cartel_chat: Vec<CartelChat>,
    pub trade_chat: Vec<TradeChat>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalChat {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub posted: String,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CartelChat {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub posted: String,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeChat {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub posted: String,
    pub message: String,
}

