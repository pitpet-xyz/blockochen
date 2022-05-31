use blockochen::Blockochen;
use blockochen::Request;

use anyhow::Result;
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
                return Err(anyhow::anyhow!("Blockchain was already initialised!"))
            }
            NewBlockchain => {
                init = true;
            }
            LoadBlockchain { json } => {
                init = true;
                chain = serde_json::from_str(json.as_str())?;
            }
            AddBlock { birth_data, data } => {
                println!("{}", chain.add(birth_data, data));
            }
            GetTTL { birth_hash } => {
                println!("{}", chain.get_ttl(&birth_hash).unwrap());
            }
            GetEvents { birth_hash } => {
                println!(
                    "{}",
                    serde_json::to_string(&chain.get_events(&birth_hash)).unwrap()
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
