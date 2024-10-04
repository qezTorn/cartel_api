use crate::Client;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

#[tokio::test]
async fn basic_test(){
    dotenv().ok();
    let test_client = Client::new(env::var("qez_key").unwrap());
    let qez = test_client.get_cartel().await;
    let velthir_test_client = Client::new(env::var("velthir_key").unwrap());
    let velthir = velthir_test_client.get_cartel().await;
    println!("qez cartel info: {:?}", qez);
    println!("velthir cartel info: {:?}", velthir);
    let abusement = test_client.get_cartel_with_id(9).await;
    println!("abusement cartel info: {:?}", abusement);
}
