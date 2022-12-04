use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};

pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = secp256k1::Secp256k1::new();
    let mut random_number_generator = rngs::StdRng::seed_from_u64(111);
    secp.generate_keypair(&mut random_number_generator)
}
