#![allow(unused_variables, unused_imports, dead_code)]
use std::{env, ops::Add, path::Path};
use bdk::{bitcoin::Network, blockchain::ElectrumBlockchain, database::SqliteDatabase, electrum_client::Client, SyncOptions, Wallet};
// use bitcoin_utils::generate_address;
use axum::{response::{Html, IntoResponse}, routing::get, Router, Json};
mod bitcoin_utils;

struct AppError(anyhow::Error);

#[derive(serde::Serialize)]
struct AddressResponse {
    address: String,
    index: u64,
}

// generate_address();

// enum Result<String, VarError> {
//     Ok(String),
//     Err(VarError),
// }

// fn main() -> anyhow::Result<()> {

//     dotenv::from_filename(".env.example")?;
//     let descriptor = env::var("DESCRIPTOR")?;
//     // dbg!(descriptor);

//     let wallet_db = Path::new("./testing.db");

//     let client = Client::new("ssl://electrum.blockstream.info:60002")?;
//     let blockchain = ElectrumBlockchain::from(client);

//     let wallet = Wallet::new(
//         descriptor.as_str(),
//         None,
//         Network:: Testnet,
//         SqliteDatabase::new(wallet_db))?;

//     // wallet.sync(&blockchain, SyncOptions::default())?;

//     let new_address = wallet.get_address(bdk::wallet::AddressIndex::New)?
//                                         .address
//                                         .to_string()
//                                         .to_lowercase();
//     let balance = wallet.get_balance()?;

//     dbg!(balance);
//     dbg!(new_address);
//     Ok(())
// }

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Result<impl IntoResponse, AppError> {
    let response = AddressResponse {
        address: "test".to_string(),
        index: 0,
    };
    Ok(Json(response))
}


// https://www.youtube.com/watch?v=md-ecvXBGzI (63min)
