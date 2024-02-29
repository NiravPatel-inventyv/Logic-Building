use std::time::Duration;

use tikv_client::{Error, Key, KvPair, RawClient};
use tokio::time;

pub async fn get_client() -> RawClient {
    let client;
    match tikv_client::RawClient::new(vec!["127.0.0.1:2379"]).await {
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

pub async fn add_record(key: String, value: String) -> bool {
    let client = get_client().await;
    if let Ok(_res) = client.put(key.to_string(), value).await {
        true
    } else {
        false
    }
}

pub async fn get_record(key: String) -> String {
    let client = get_client().await;
    if is_valid(&key) {
        let value: Result<Option<Vec<u8>>, tikv_client::Error> = client.get(key.to_string()).await;
        if let Some(value) = value.unwrap() {
            String::from_utf8(value).unwrap()
        } else {
            String::from(" ")
        }
    } else {
        String::from(" ")
    }
}
pub async fn delete_record(key: String) -> bool {
    let client = get_client().await;
    if is_valid(&key) {
        let value = client.delete(key.to_string()).await;
        if let Ok(_) = value {
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub async fn batch_put_records(data: Vec<(String, String)>) -> bool {
    let client = get_client().await;
    let value = client.batch_put(data).await;
    if let Ok(_) = value {
        true
    } else {
        false
    }
}

pub async fn scan_data(start: String, end: String, batch: u32) -> Vec<KvPair> {
    let client = get_client().await;
    let value: Result<Vec<KvPair>, Error> = client.scan(start..end, batch).await;
    if let Ok(val) = value {
        val
    } else {
        vec![KvPair::new("".to_string(), "".to_string())]
    }
}


fn is_valid(key: &String) -> bool {
    if key.len() != 0 && key.contains("-") {
        true
    } else {
        false
    }
}
