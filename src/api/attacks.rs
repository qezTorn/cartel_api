use crate::client::Client;
use crate::error::Error;
use crate::model::AttacksResponse;
use crate::model::Range;

impl Client {
    pub async fn get_attacks_with_range(&self, range: Range) -> Result<AttacksResponse, Error>{
        let url = format!("{}/user?type=attacks&key={}&{}", self.api_url, self.api_key, range.use_range());
        let response = self.http_client.get(&url).send().await?;
        let attacks: AttacksResponse = response.json().await?;
        Ok(attacks)
    }
    pub async fn get_attacks(&self) -> Result<AttacksResponse, Error>{
        Ok(self.get_attacks_with_range(Range::new().build()).await?)
    }



}
