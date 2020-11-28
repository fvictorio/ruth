use rand::Rng;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum RandomEntity {
    /// Generate a random address
    Address {
        /// The number of addresses to generate
        #[structopt(short, long, default_value = "1")]
        count: u32,
    },

    /// Generate a random bytes string
    Bytes {
        /// The length (in bytes) of the generated string
        #[structopt(short, long, default_value = "32")]
        length: u32,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ruth",
    about = "A set of command line tools for Ethereum developers"
)]
enum RuthCommands {
    /// Generate a random value
    Random {
        #[structopt(subcommand)]
        entity: RandomEntity,
    },
}

fn random_bytes(length: u32) -> String {
    let mut rng = rand::thread_rng();

    let bytes: Vec<_> = (0..length).map(|_| rng.gen::<u8>()).collect();

    hex::encode(bytes)
}

fn random_address() -> ethers::types::Address {
    let mut rng = rand::thread_rng();

    let wallet = ethers::signers::Wallet::new(&mut rng);

    wallet.address()
}

fn main() {
    let opt = RuthCommands::from_args();

    if let RuthCommands::Random { entity } = opt {
        match entity {
            RandomEntity::Address { count } => {
                for _ in 0..count {
                    let address = random_address();
                    let address = ethers::utils::to_checksum(&address, None);
                    println!("{}", address);
                }
            }
            RandomEntity::Bytes { length } => {
                println!("0x{}", random_bytes(length));
            }
        }
    }
}
