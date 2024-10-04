use crate::Client;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

#[tokio::test]
async fn basic_test(){
    dotenv().ok();
    let test_client = Client::new(env::var("qez_key").unwrap());
    let qez = test_client.get_user_basic().await;
    let zuni = test_client.get_user_basic_with_id(2012).await;
    println!("qez: {:?}", qez);
    println!("zuni: {:?}", zuni); 
}
