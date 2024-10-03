use crate::Client;

#[tokio::test]
async fn advanced_test(){
    let test_client = Client::new("C4FEB900-7405-49A2-85E");
    let qez = test_client.get_cooldowns().await;
    println!(": {:?}", qez); 
}
