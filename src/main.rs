#![allow(unused_variables)]
use std::env;
use bdk::{bitcoin::Network, database::MemoryDatabase, Wallet};


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

    let wallet = Wallet::new(
        descriptor.as_str(),
        None,
        Network:: Signet,
        MemoryDatabase::default())?;

    dbg!(wallet);
    Ok(())
}


// https://www.youtube.com/watch?v=md-ecvXBGzI (63min)
