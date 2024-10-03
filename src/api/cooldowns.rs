use crate::model::CooldownResponse;
use crate::client::Client;
use crate::error::Error;

impl Client {
    pub async fn get_cooldowns(&self) -> Result<CooldownResponse, Error>{
        let url = format!("{}/user?type=cooldowns&key={}", self.api_url, self.api_key);
        let response = self.http_client.get(&url).send().await?;
        let cooldowns: CooldownResponse = response.json().await?;
        Ok(cooldowns)
    }
}
