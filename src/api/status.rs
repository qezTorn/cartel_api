use crate::model::StatusResponse;
use crate::client::Client;
use crate::error::Error;

impl Client {
    pub async fn get_status(&self) -> Result<StatusResponse, Error>{
        let url = format!("{}/user?type=status&key={}", self.api_url, self.api_key);
        let response = self.http_client.get(&url).send().await?;
        let status: StatusResponse = response.json().await?;
        Ok(status)
    }
}
