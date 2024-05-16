use dotenv::from_filename;
use std::env;

// use bitcoin_utils::generate_address;
// mod bitcoin_utils;
// generate_address();
enum Result<String, VarError> {
    Ok(String),
    Err(VarError),
}


fn main() {
    from_filename(".env").ok();

    let descriptor = env::var("DESCRIPTOR").unwrap();
    dbg!(descriptor);
}
// https://www.youtube.com/watch?v=md-ecvXBGzI (31min)
