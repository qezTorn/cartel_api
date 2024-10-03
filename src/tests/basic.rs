use crate::Client;

#[tokio::test]
async fn basic_test(){
    let test_client = Client::new("C4FEB900-7405-49A2-85E");
    let qez = test_client.get_user_basic().await;
    let zuni = test_client.get_user_basic().await;
    println!("qez: {:?}", qez);
    println!("zuni: {:?}", zuni); 
}
