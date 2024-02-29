use std::{ops::RangeInclusive, time::Duration};

use tikv_client::{RawClient, TransactionClient};
use tokio::time;

pub async fn get_client() -> RawClient {
    let client;
    match tikv_client::RawClient::new(vec!["127.0.0.1:2379",]).await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to put initial value into TiKV: {:?}", err);
            println!("");
            // Attempt reconnection
            loop {
                match tikv_client::RawClient::new(vec!["127.0.0.1:2379"]).await {
                    Ok(new_client) => {
                        println!("Reconnected to TiKV");
                        client = new_client;
                        break;
                    }
                    Err(err) => {
                        eprintln!("Failed to reconnect to TiKV: {:?}\n", err);
                    }
                }
                time::sleep(Duration::from_secs(10)).await;
            }
            client
        }
    }
}

pub async fn add_record(key: String, value: String) {
    let client = get_client().await;
    client.put(key.to_string(), value).await.unwrap();
    println!("Record Added With Key :{:?}", key);
}

pub async fn get_record(key: String) -> String {
    let client = get_client().await;
    let value: Result<Option<Vec<u8>>, tikv_client::Error> = client.get(key.to_string()).await;
    if let Ok(value) = value {
        if let Some(value) = value {
            String::from_utf8(value).unwrap()
        } else {
            return "".to_string();
        }
    } else {
        return "".to_string();
    }
}
pub async fn delete_record(key: String) {
    let client = get_client().await;
    let value = client.delete(key.to_string()).await;
    if let Ok(_) = value {
        println!("Value with key : {:?} is deleted..", key);
    } else {
        println!("failed to delete value");
    }
}

pub async fn put_batch_record(data: Vec<(String, String)>) {
    let client = get_client().await;
    let value = client.batch_put(data).await;
    if let Ok(_) = value {
        println!("Data successfully inserted");
    }
}

pub async fn delete_batch(keys: Vec<String>) {
    let client = get_client().await;
    let value = client.batch_delete(keys).await;
    if let Ok(_) = value {
        println!("Data deleted..");
    } else {
        println!("failed to delete records");
    }
}

pub async fn batch_scan(keys: Vec<RangeInclusive<String>>, limit: u32) {
    let client = get_client().await;
    let value = client.batch_scan(keys, limit).await;
    if let Ok(value) = value {
        println!("Data fetched");
        println!("{:#?}", value);
    } else {
        println!("failed to fetch records");
    }
}
