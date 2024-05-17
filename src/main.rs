#![allow(unused_variables)]
use std::{env, path::Path};
use bdk::{bitcoin::Network, blockchain::ElectrumBlockchain, database::SqliteDatabase, electrum_client::Client, SyncOptions, Wallet};


// use bitcoin_utils::generate_address;
// mod bitcoin_utils;
// generate_address();

// enum Result<String, VarError> {
//     Ok(String),
//     Err(VarError),
// }

fn main() -> anyhow::Result<()> {

    dotenv::from_filename(".env.example")?;
    let descriptor = env::var("DESCRIPTOR")?;
    // dbg!(descriptor);

    let wallet_db = Path::new("./testing.db");

    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);

    let wallet = Wallet::new(
        descriptor.as_str(),
        None,
        Network:: Testnet,
        SqliteDatabase::new(wallet_db))?;

    wallet.sync(&blockchain, SyncOptions::default())?;

    let new_address = wallet.get_address(bdk::wallet::AddressIndex::New)?
                                        .address
                                        .to_string()
                                        .to_lowercase();
    let balance = wallet.get_balance()?;

    dbg!(balance);
    dbg!(new_address);
    Ok(())
}


// https://www.youtube.com/watch?v=md-ecvXBGzI (63min)
