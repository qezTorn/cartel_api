//https://cartelempire.online/api/cartel?type=basic&key=

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct CartelResponse {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "created")]
    pub created: i64,
    #[serde(rename = "reputation")]
    pub reputation: f64,
    #[serde(rename = "status")]
    pub status: String,
}

