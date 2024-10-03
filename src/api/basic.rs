use crate::model::BasicResponse;
use crate::client::Client;
use crate::error::Error;

impl Client {
    pub async fn get_user_basic_with_id(&self, user_id: Option<u32>) -> Result<BasicResponse, Error>{
        let url = match user_id {
            Some(id) => {
                format!("{}/user/?id={}&key={}", self.api_url, id, self.api_key)
            }
            None => {
                format!("{}/user?key={}", self.api_url, self.api_key)
            }
        };
        //let url = format!("{}/user?key={}", self.api_url, self.api_key);
        let response = self.http_client.get(&url).send().await?;
        let basic: BasicResponse = response.json().await?;
        Ok(basic)
    }
    pub async fn get_user_basic(&self) -> Result<BasicResponse, Error>{
        Ok(self.get_user_basic_with_id(None).await?)
    }
}
