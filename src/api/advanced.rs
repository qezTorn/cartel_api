use crate::client::Client;
use crate::error::Error;
use crate::model::AdvancedResponse;

impl Client {
    pub async fn get_user_advanced_with_id(&self, user_id: Option<u32>) -> Result<AdvancedResponse, Error>{
        let url = match user_id {
            Some(id) => {
                format!("{}/user/?type=advanced&id={}&key={}", self.api_url, id, self.api_key)
            }
            None => {
                format!("{}/user/?type=advanced&key={}", self.api_url, self.api_key)
            }
        };
        //let url = format!("{}/user?key={}", self.api_url, self.api_key);
        let response = self.http_client.get(&url).send().await?;
        let advanced: AdvancedResponse = response.json().await?;
        Ok(advanced)
    }
    pub async fn get_user_advanced(&self) -> Result<AdvancedResponse, Error>{
        Ok(self.get_user_advanced_with_id(None).await?)
    }
}
