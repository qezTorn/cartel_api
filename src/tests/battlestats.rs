use crate::Client;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

#[tokio::test]
async fn battlestats_test(){
    dotenv().ok();

    let test_client = Client::new(env::var("qez_key").unwrap());
    let qez = test_client.get_battlestats().await;
    let velthir_client = Client::new(env::var("velthir_key").unwrap());
    let velthir = velthir_client.get_battlestats().await;

    println!(": {:?}", qez); 
    println!(": {:?}", velthir); 
}
