use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedResponse {
    pub user_id: i64,
    pub name: String,
    pub status: String,
    pub age: i64,
    pub level: i64,
    pub reputation: f64,
    pub cartel_id: Option<i64>,
    pub jail_release: i64,
    pub hospital_release: i64,
    pub current_life: i64,
    pub max_life: i64,
    pub attacks_won: i64,
    pub defends_lost: i64,
    pub user_type: String,
    pub friend_count: i64,
    pub enemy_count: i64,
    pub current_bounty_reward: Option<String>,
    pub last_active: i64,
    pub cartel_name: Option<String>,
}
