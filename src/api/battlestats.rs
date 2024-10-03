use crate::model::BattlestatsResponse;
use crate::client::Client;
use crate::error::Error;

impl Client {
    pub async fn get_battlestats(&self) -> Result<BattlestatsResponse, Error>{
        let url = format!("{}/user?type=battlestats&key={}", self.api_url, self.api_key);
        let response = self.http_client.get(&url).send().await?;
        let battlestats: BattlestatsResponse = response.json().await?;
        Ok(battlestats)
    }
}
