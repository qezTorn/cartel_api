use crate::client::Client;
use crate::error::Error;
use crate::model::{ChatResponse, CartelChat, TradeChat, GlobalChat};
//use crate::model::CartelChat;


//Note this section is possibly incomplete as the documentation doesnt exist yet...
//I would guess that you can probably specify what chats to pull and possibly a range too
//that will be implmented in the future. qez - 10/8/2024
impl Client {
    pub async fn get_chats(&self) -> Result<ChatResponse, Error>{
        let url = format!("{}/chat/?&key={}", self.api_url, self.api_key);
        let response = self.http_client.get(&url).send().await?;
        let chats: ChatResponse = response.json().await?;
        Ok(chats)
    }

    pub async fn get_chats_cartel(&self) -> Result<Vec<CartelChat>, Error>{
        let chats = self.get_chats().await?;
        Ok(chats.cartel_chat)
    }
    pub async fn get_chats_trade(&self) -> Result<Vec<TradeChat>, Error>{
        let chats = self.get_chats().await?;
        Ok(chats.trade_chat)
    }
    pub async fn get_chats_global(&self) -> Result<Vec<GlobalChat>, Error>{
        let chats = self.get_chats().await?;
        Ok(chats.global_chat)
    }
}
