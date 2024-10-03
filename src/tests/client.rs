use crate::Client;


#[test]
fn client_test(){
    let test = Client::new("C4FEB900-7405-49A2-85E");
    println!("{}", test.api_key);
}
