use crate::Client;

#[tokio::test]
async fn advanced_test(){
    let test_client = Client::new("C4FEB900-7405-49A2-85E");
    let qez = test_client.get_user_advanced().await;
    let zuni = test_client.get_user_advanced_with_id(Some(2012)).await;
    let unafilated = test_client.get_user_advanced_with_id(Some(14020)).await;
    println!("qez: {:?}", qez);
    println!("zuni: {:?}", zuni); 
    println!("unafilated: {:?}", unafilated); 
}
