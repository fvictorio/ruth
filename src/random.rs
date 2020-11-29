use rand::Rng;

pub fn random_bytes(length: u32) -> String {
    let mut rng = rand::thread_rng();

    let bytes: Vec<_> = (0..length).map(|_| rng.gen::<u8>()).collect();

    hex::encode(bytes)
}

pub fn random_address() -> ethers::types::Address {
    let mut rng = rand::thread_rng();

    let wallet = ethers::signers::Wallet::new(&mut rng);

    wallet.address()
}
