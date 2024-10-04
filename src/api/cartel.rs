use crate::model::CartelResponse;
use crate::client::Client;
use crate::error::Error;

impl Client{

    pub async fn get_cartel_with_id(&self, cartel_id: u32) -> Result<CartelResponse, Error>{
        Ok(self.get_cartel_with_id_option(Some(cartel_id)).await?)
    }

    pub async fn get_cartel(&self) -> Result<CartelResponse, Error>{
        Ok(self.get_cartel_with_id_option(None).await?)
    }

    async fn get_cartel_with_id_option(&self, cartel_id: Option<u32>) -> Result<CartelResponse, Error>{
        let url = match cartel_id {
            Some(id) => {
                format!("{}/cartel/?id={}&key={}", self.api_url, id, self.api_key)
            }
            None => {
                format!("{}/cartel?key={}", self.api_url, self.api_key)
            }
        };
        //let url = format!("{}/user?key={}", self.api_url, self.api_key);
        let response = self.http_client.get(&url).send().await?;
        let cartel: CartelResponse = response.json().await?;
        Ok(cartel)
    }
}
