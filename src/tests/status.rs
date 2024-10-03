use crate::Client;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

#[tokio::test]
async fn status_test(){
    dotenv().ok();
    let test_client = Client::new(env::var("qez_key").unwrap());
    let qez = test_client.get_status().await;
    println!(": {:?}", qez); 
}
