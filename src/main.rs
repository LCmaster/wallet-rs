mod eth_wallet;
mod state;
use eth_wallet::{public_key_address, EthWallet};
use state::{AppState, AppStateLogic};

use core::result::Result::Ok;
use std::{
    collections::{HashMap, VecDeque},
    env,
};

use anyhow::Result;
use inquire::{Confirm, Select};
use web3::{contract::Error, transports::WebSocket, types::Address, Web3, futures::executor};

struct CreateWallet;
impl AppStateLogic for CreateWallet {
    fn run(&self) -> AppState {
        let (sec_key, pub_key) = eth_wallet::generate_keypairs();
        let new_wallet = EthWallet::new(&sec_key, &pub_key);

        println!("New account created successfully");
        let ans = Confirm::new("Save the account?")
            .with_default(true)
            .prompt();
        if let Ok(true) = ans {
            new_wallet.save_to_file();
            println!("Account saved");
        }
        println!("Account selected");
        AppState::ExecuteLogic(Box::new(WalletSelected(new_wallet)))
    }
}
struct WalletSelected(EthWallet);
impl AppStateLogic for WalletSelected {
    fn run(&self) -> AppState {
        let wallet = self.0.clone();
        
        let mut network: String;
        if let Ok(env_var) = env::var("INFURA_NETWORK") {
            network = env_var;
        } else {
            //TODO: Print error message
            return AppState::Quit;
        }

        let mut endpoint: String;
        if let Ok(env_var) = env::var("INFURA_ENDPOINT") {
            endpoint = env_var;
        } else {
            //TODO: Print error message
            return AppState::Quit;
        }

        let mut api_key: String;
        if let Ok(env_var) = env::var("INFURA_API_KEY") {
            api_key = env_var;
        } else {
            //TODO: Print error message
            return AppState::Quit;
        }
        
        if endpoint.ends_with('/') {
            endpoint.pop();
        }
        
        let url_token = vec!["wss://", &network, ".", &endpoint, "/", &api_key];
        let node_url = url_token.join("");
        let connection_result = executor::block_on(eth_wallet::connect(&node_url));

        if let Ok(web3) = connection_result {
            let ans = Select::new(
                "Select an action",
                vec!["Show balance", "Send ETH to...", "Go back", "Quit"]
            )
            .prompt();

            match ans {
                Ok("Show balance") => {
                    let res = executor::block_on(wallet.get_balance(&web3));
                    if let Ok(balance) = res {
                        println!("Balance: {} ETH", eth_wallet::wei_to_eth(balance));
                    }
                    AppState::ExecuteLogic(Box::new(WalletSelected(wallet)))
                },
                Ok("Go back") => return AppState::ExecuteLogic(Box::new(AppInit)),
                Ok("Quit") => AppState::Quit,
                _ => AppState::Quit,
            }
        } else {
            //TODO: Print error message
            AppState::Quit
        }
    }
}

struct AppInit;
impl AppStateLogic for AppInit {
    fn run(&self) -> AppState {
        let mut options: Vec<(String, AppState)> = vec![];

        if let Ok(wallet) = eth_wallet::EthWallet::load_from_file() {
            if let Ok(public_key) = wallet.get_public_key() {
                let public_key_address = public_key_address(&public_key).to_string();
                options.push((
                    public_key_address,
                    AppState::ExecuteLogic(Box::new(WalletSelected(wallet))),
                ));
            }
        }

        options.push((
            String::from("Create a new account"),
            AppState::ExecuteLogic(Box::new(CreateWallet)),
        ));
        options.push((String::from("Quit"), AppState::Quit));

        let ans = Select::new(
            "Select an account",
            options.iter().map(|opt| opt.0.to_owned()).collect(),
        )
        .prompt();

        if let Ok(choice) = ans {
            let index = options.iter().position(|opt| opt.0 == choice).unwrap();
            options.remove(index).1
        } else {
            AppState::Quit
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let mut state = state::AppState::ExecuteLogic(Box::new(AppInit));
    while let Some(next_state) = AppState::exec(state) {
        state = next_state;
    }

    Ok(())
}
