use blockochen::Blockochen;
use blockochen::Request;

use anyhow::Result;
use serde_json::json;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut chain = Blockochen::new();
    let mut init = false;
    /*
    println!("Examples:");
    println!("{}", serde_json::to_string(&Request::NewBlockchain)?);
    println!(
        "{}",
        serde_json::to_string(&Request::LoadBlockchain {
            json: String::new()
        })?
    );
    println!("{}", serde_json::to_string(&Request::Quit)?);
    println!(
        "{}",
        serde_json::to_string(&Request::AddBlock {
            birth_data: b"test".to_vec(),
            data: b"foobar".to_vec()
        })?
    );
    println!(
        "{}",
        serde_json::to_string(&Request::GetTTL {
            birth_hash: b"test".to_vec()
        })?
    );
    println!(
        "{}",
        serde_json::to_string(&Request::GetEvents {
            birth_hash: b"test".to_vec()
        })?
    );
    */
    loop {
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input)?;
        if input.trim().is_empty() {
            break;
        }
        let request = serde_json::from_str(input.as_str())?;
        use Request::*;
        match request {
            NewBlockchain | LoadBlockchain { .. } if init => {
                println!(
                    "{}",
                    json!({ "type": "error", "message": "Blockchain was already initialised!", })
                );
            }
            NewBlockchain => {
                init = true;
            }
            LoadBlockchain { json } => {
                init = true;
                chain = serde_json::from_str(json.as_str())?;
            }
            SpawnBlock { birth_data, data } => {
                println!("{}", chain.spawn(birth_data, data));
            }
            AddBlock { birth_hash, data } => match chain.add(birth_hash.clone(), data) {
                Ok(hash) => {
                    println!("{}", hash);
                }
                Err(None) => {
                    println!(
                        "{}",
                        json!({"type":"error", "message":format!("birth_hash {} not found in chain.", birth_hash)})
                    );
                }
                Err(Some(_)) => {
                    println!(
                        "{}",
                        json!({"type":"error", "message":format!("birth_hash {} is dead.", birth_hash)})
                    );
                }
            },
            GetTTL { birth_hash } => {
                if let Some(ttl) = chain.get_ttl(birth_hash.clone()) {
                    println!("{}", ttl);
                } else {
                    println!(
                        "{}",
                        json!({"type":"error", "message":format!("birth_hash {} not found in chain.", birth_hash)})
                    );
                }
            }
            GetEvents { birth_hash } => {
                println!(
                    "{}",
                    serde_json::to_string(&chain.get_events(birth_hash)).unwrap()
                );
            }
            Print => {
                println!("{}", serde_json::to_string(&chain)?);
            }
            Quit => {
                break;
            }
        }
    }
    Ok(())
}
