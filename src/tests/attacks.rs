use crate::Client;
extern crate dotenv;
use dotenv::dotenv;
use std::env;
use crate::model::Range;

//This doesnt work 
//I think it has to do with my limited understanding of the from and to
#[tokio::test]
async fn attacks_test(){
    dotenv().ok();
    let test_client = Client::new(env::var("qez_key").unwrap());


    let qez = test_client.get_attacks().await;
    println!("qez: {:?}", qez);

    let range = Range::new()
        .from(1727903530)
        .to(1727917930)
        .build();
    let qez = test_client.get_attacks_with_range(range).await;
    println!("old qez: {:?}", qez);
}
