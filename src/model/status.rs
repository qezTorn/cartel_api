use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct StatusResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "currentLife")]
    pub current_life: i64,
    #[serde(rename = "maxLife")]
    pub max_life: i64,
    #[serde(rename = "currentEnergy")]
    pub current_energy: i64,
    #[serde(rename = "maxEnergy")]
    pub max_energy: i64,
    #[serde(rename = "jailRelease")]
    pub jail_release: i64,
    #[serde(rename = "hospitalRelease")]
    pub hospital_release: i64,
}
