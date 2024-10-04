use crate::model::InventoryAdvancedResponse;
use crate::model::InventoryResponse;
use crate::client::Client;
use crate::error::Error;

impl Client {
    pub async fn get_inventory(&self) -> Result<InventoryResponse, Error>{
        let url = format!("{}/inventory?key={}", self.api_url, self.api_key);
        let response = self.http_client.get(&url).send().await?;
        let inventory: InventoryResponse = response.json().await?;
        Ok(inventory)
    }
    pub async fn get_inventory_advanced(&self) -> Result<InventoryAdvancedResponse, Error>{
        let url = format!("{}/inventory?type=advanced&key={}", self.api_url, self.api_key);
        let response = self.http_client.get(&url).send().await?;
        let inventory: InventoryAdvancedResponse = response.json().await?;
        Ok(inventory)
    }
}
