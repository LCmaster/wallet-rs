use core::result::Result::Ok;
use std::env;

use anyhow::Result;

use crate::eth_wallet::EthWallet;

mod eth_wallet;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let eth_wallet = if let Ok(wallet) = eth_wallet::EthWallet::load_from_file() {
        wallet
    } else {
        let (sec_key, pub_key) = eth_wallet::generate_keypairs();
        let new_wallet = EthWallet::new(&sec_key, &pub_key);
        new_wallet
    };

    eth_wallet.save_to_file();

    // println!("");
    println!("Ethereum Walet");
    println!("Account {}", &eth_wallet.public_address);

    let endpoint = env::var("INFURA_NETWORK_ENDPOINT")?;
    let web3 = eth_wallet::connect(&endpoint).await?;

    let balance = eth_wallet.get_balance(&web3).await?;
    println!("Balance: {} ETH", eth_wallet::wei_to_eth(balance));

    Ok(())
}
