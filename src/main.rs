mod eth_wallet;

fn main() {
    let (sec_key, pub_key) = eth_wallet::generate_keypair();
    println!("Hello, world!");
}
