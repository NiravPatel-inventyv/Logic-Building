use std::time::Duration;

use tikv_client::RawClient;
use tokio::time;

pub async fn get_client() -> RawClient {
    let client = RawClient::new(vec!["127.0.0.1:2379"]).await.unwrap();

    client
}
pub async fn add_record(key: String, value: String) {
    let client = get_client().await;
    client.put(key.to_string(), value).await.unwrap();
    println!("Record Added With Key :{:?}", key);
}

pub async fn get_record(key: String) {
    let client = get_client().await;
    let value = client.get(key.to_string()).await;
    if let Ok(value) = value {
        if let Some(value) = value {
            println!("Value  :{:?}", String::from_utf8(value).unwrap());
        } else {
            println!("Value not found");
        }
    }
}
// async fn auto_reconnect(mut client: tikv_client::RawClient) -> tikv_client::RawClient {
//     loop {
//         let connected = match client.get("ping".to_string()).await {
//             Ok(_) => true,
//             Err(_) => false,
//         };

//         if !connected {
//             match tikv_client::RawClient::new(vec!["127.0.0.1:2379"]).await {
//                 Ok(new_client) => {
//                     println!("Reconnected to TiKV");
//                     client = new_client;
//                 }
//                 Err(err) => {
//                     eprintln!("Failed to reconnect to TiKV: {:?}", err);
//                 }
//             }
//         }
//     }
// }
