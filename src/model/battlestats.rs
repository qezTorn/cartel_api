use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BattlestatsResponse {
    pub strength: f64,
    pub agility: f64,
    pub defence: f64,
    pub accuracy: f64,
    pub intelligence: f64,
}
