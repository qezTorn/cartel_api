use serde_this_or_that::as_i64;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CooldownResponse {
    #[serde(deserialize_with="as_i64")]
    pub drug_cooldown: i64,
    #[serde(deserialize_with="as_i64")]
    pub medical_cooldown: i64,
    #[serde(deserialize_with="as_i64")]
    pub booster_cooldown: i64,
}
