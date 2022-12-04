mod eth_wallet;

fn main() {
    let (sec_key, pub_key) = eth_wallet::generate_keypair();
    let pub_address = eth_wallet::public_key_address(&pub_key);

    println!("secret key: {}", &sec_key.to_string());
    println!("public key: {}", &pub_key.to_string());
    println!("public address: {:?}", pub_address);
}
