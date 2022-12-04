use anyhow::{Ok, Result};

use crate::eth_wallet::EthWallet;

mod eth_wallet;

fn main() -> Result<()> {
    let (sec_key, pub_key) = eth_wallet::generate_keypair();
    let pub_address = eth_wallet::public_key_address(&pub_key);
    let eth_wallet = EthWallet::new(&sec_key, &pub_key);

    println!("secret key: {}", &sec_key.to_string());
    println!("public key: {}", &pub_key.to_string());
    println!("public address: {:?}", pub_address);
    println!("");
    println!("Ethereum Walet");
    println!("{:?}", &eth_wallet);

    eth_wallet.save_to_file();

    let loaded_wallet = eth_wallet::EthWallet::load_from_file();

    println!("");
    println!("Loaded Ethereum Walet");
    println!("{:?}", &loaded_wallet);

    Ok(())
}
