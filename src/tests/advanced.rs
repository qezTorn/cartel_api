use crate::Client;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

#[tokio::test]
async fn advanced_test(){
    dotenv().ok();
    let test_client = Client::new(env::var("qez_key").unwrap());
    let qez = test_client.get_user_advanced().await;
    let zuni = test_client.get_user_advanced_with_id(2012).await;
    let unafilated = test_client.get_user_advanced_with_id(14020).await;
    println!("qez: {:?}", qez);
    println!("zuni: {:?}", zuni); 
    println!("unafilated: {:?}", unafilated); 
}
