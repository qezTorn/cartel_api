use crate::Client;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

#[tokio::test]
async fn cooldown_test(){
    dotenv().ok();
    let test_client = Client::new(env::var("qez_key").unwrap());
    let qez = test_client.get_cooldowns().await;
    println!(": {:?}", qez); 
}
