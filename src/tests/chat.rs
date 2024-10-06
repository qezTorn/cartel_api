use crate::Client;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

#[tokio::test]
async fn chat_test(){
    dotenv().ok();
    let test_client = Client::new(env::var("qez_key").unwrap());
    let chat = test_client.get_chats().await;

    for msg in chat.unwrap().global_chat{
        println!("{msg:?}");
    }
}

#[tokio::test]
async fn cartel_chat_test(){
    dotenv().ok();
    let jexxic_client =  Client::new(env::var("jexxic_key").unwrap());
    println!("{:?}",env::var("jexxic_key").unwrap());
    let cartel_chats = jexxic_client.get_chats_cartel().await;

    for msg in cartel_chats.unwrap(){
        println!("{msg:?}");
    }
}

#[tokio::test]
async fn global_chat_test(){
    dotenv().ok();
    let jexxic_client =  Client::new(env::var("jexxic_key").unwrap());
    println!("{:?}",env::var("jexxic_key").unwrap());
    let global_chats = jexxic_client.get_chats_global().await;

    for msg in global_chats.unwrap(){
        println!("{msg:?}");
    }
}

#[tokio::test]
async fn trade_chat_test(){
    dotenv().ok();
    let jexxic_client =  Client::new(env::var("jexxic_key").unwrap());
    println!("{:?}",env::var("jexxic_key").unwrap());
    let trade_chats = jexxic_client.get_chats_trade().await;

    for msg in trade_chats.unwrap(){
        println!("{msg:?}");
    }
}
