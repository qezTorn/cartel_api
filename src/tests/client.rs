use crate::Client;
extern crate dotenv;
use dotenv::dotenv;
use std::env;


#[test]
fn client_test(){
    dotenv().ok();
    let test_client = Client::new(env::var("qez_key").unwrap());
    println!("{}", test_client.api_key);
}
