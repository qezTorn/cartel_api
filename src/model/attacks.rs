use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct AttacksResponse {
    #[serde(rename = "attacks")]
    pub attacks: Vec<Attack>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Attack {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "initiatorId")]
    pub initiator_id: i64,
    #[serde(rename = "initiatorCartelId")]
    pub initiator_cartel_id: i64,
    #[serde(rename = "targetId")]
    pub target_id: i64,
    #[serde(rename = "targetCartelId")]
    pub target_cartel_id: i64,
    #[serde(rename = "repGained")]
    pub rep_gained: f64,
    #[serde(rename = "cashMugged")]
    pub cash_mugged: i64,
    #[serde(rename = "fairFightMultiplier")]
    pub fair_fight_multiplier: i64,
    #[serde(rename = "attackType")]
    pub attack_type: String,
    #[serde(rename = "outcome")]
    pub outcome: String,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "defenderRepGain")]
    pub defender_rep_gain: i64,
    #[serde(rename = "isWar")]
    pub is_war: Value,
    #[serde(rename = "warId")]
    pub war_id: Value,
    #[serde(rename = "itemAwarded")]
    pub item_awarded: Value,
}

