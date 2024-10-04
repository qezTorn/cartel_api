use crate::Client;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

#[tokio::test]
async fn inventory_test(){
    dotenv().ok();

    let test_client = Client::new(env::var("qez_key").unwrap());
    let qez = test_client.get_inventory().await;
    let velthir_client = Client::new(env::var("velthir_key").unwrap());
    let velthir = velthir_client.get_inventory().await;

    println!(": {:?}", qez); 
    println!(": {:?}", velthir); 

    for i in qez.unwrap().items.iter(){
        println!("{:?}", i);
    }
}
