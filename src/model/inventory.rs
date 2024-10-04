use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct InventoryResponse {
    #[serde(rename = "inventory")]
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Item {
    #[serde(rename = "itemId")]
    pub item_id: i64,
    #[serde(rename = "quantity")]
    pub quantity: i64,
    #[serde(rename = "equipped")]
    pub equipped: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct InventoryAdvancedResponse {
    #[serde(rename = "inventory")]
    pub items: Vec<ItemAdvanced>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct ItemAdvanced {
    #[serde(rename = "itemId")]
    pub item_id: i64,
    #[serde(rename = "quantity")]
    pub quantity: i64,
    #[serde(rename = "equipped")]
    pub equipped: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "secondaryType")]
    pub secondary_type: Option<String>,
    #[serde(rename = "effect")]
    pub effect: Option<String>,
    #[serde(rename = "damage")]
    pub damage: Option<i64>,
    #[serde(rename = "accuracy")]
    pub accuracy: Option<i64>,
    #[serde(rename = "clipSize")]
    pub clip_size: Option<i64>,
    #[serde(rename = "minFireRate")]
    pub min_fire_rate: Option<i64>,
    #[serde(rename = "maxFireRate")]
    pub max_fire_rate: Option<i64>,
    #[serde(rename = "armour")]
    pub armour: Option<i64>,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "rank")]
    pub rank: i64,
}
