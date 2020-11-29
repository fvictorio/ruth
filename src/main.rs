mod random;

use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use std::convert::TryFrom;
use structopt::StructOpt;

use random::*;

#[derive(Debug, StructOpt)]
enum RandomEntity {
    /// Generate a random address
    #[structopt(alias = "a")]
    Address {
        /// The number of addresses to generate
        #[structopt(short, long, default_value = "1")]
        count: u32,
    },

    /// Generate a random bytes string
    #[structopt(alias = "b")]
    Bytes {
        /// The length (in bytes) of the generated string
        #[structopt(short, long, default_value = "32")]
        length: u32,
    },
}

#[derive(Debug, StructOpt)]
enum GetEntity {
    /// Get a block given its number
    #[structopt(alias = "a")]
    Block {
        /// The number of the block to fetch
        number: u64,
    },

    /// Generate a transaction object given its hash
    #[structopt(alias = "b")]
    Tx {
        /// The hash of the transaction to fetch
        tx_hash: String,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ruth",
    about = "A set of command line tools for Ethereum developers"
)]
enum RuthCommands {
    /// Generate a random value
    #[structopt(alias = "r")]
    Random {
        #[structopt(subcommand)]
        entity: RandomEntity,
    },
    /// Get some value from a network
    Get {
        #[structopt(subcommand)]
        entity: GetEntity,
    },
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(long, short, default_value = "http://localhost:8545", global = true)]
    network: String,

    #[structopt(subcommand)]
    cmd: RuthCommands,
}

fn get_network(network_flag: String) -> String {
    let result = match &network_flag[..] {
        "mainnet" => String::from("https://mainnet.infura.io/v3/76fb6c10f1584483a45a0a28e91b07ad"),
        "ropsten" => String::from("https://ropsten.infura.io/v3/76fb6c10f1584483a45a0a28e91b07ad"),
        "rinkeby" => String::from("https://rinkeby.infura.io/v3/76fb6c10f1584483a45a0a28e91b07ad"),
        "goerli" => String::from("https://goerli.infura.io/v3/76fb6c10f1584483a45a0a28e91b07ad"),
        "kovan" => String::from("https://kovan.infura.io/v3/76fb6c10f1584483a45a0a28e91b07ad"),
        _ => network_flag,
    };

    result
}

#[tokio::main]
pub async fn main() {
    let opt = Opt::from_args();

    let network = get_network(opt.network);

    match opt.cmd {
        RuthCommands::Random { entity } => match entity {
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
        },
        RuthCommands::Get { entity } => match entity {
            GetEntity::Block { number } => {
                let provider = Provider::<Http>::try_from(network)
                    .expect("could not instantiate HTTP Provider");

                let block = provider.get_block(number).await.unwrap();

                if let Some(block) = block {
                    let block_json = serde_json::to_string_pretty(&block).unwrap();

                    println!("{}", block_json);
                } else {
                    eprintln!("Block `{}` not found", number);
                }
            }
            GetEntity::Tx { tx_hash } => {
                let tx_hash_str = tx_hash.clone();

                let tx_hash = if tx_hash.starts_with("0x") {
                    String::from(&tx_hash[2..])
                } else {
                    tx_hash
                };

                let provider = Provider::<Http>::try_from(network)
                    .expect("could not instantiate HTTP Provider");

                let tx_hash = hex::decode(tx_hash).expect("invalid transaction hash");

                assert!(tx_hash.len() == 32);

                let mut tx_hash_bytes = [0u8; 32];

                for (i, b) in tx_hash.iter().enumerate() {
                    tx_hash_bytes[i] = *b;
                }

                let tx_hash = H256::try_from(tx_hash_bytes).expect("Invalid transaction hash");

                let tx = provider.get_transaction(tx_hash).await.unwrap();

                if let Some(tx) = tx {
                    let tx_json = serde_json::to_string_pretty(&tx).unwrap();

                    println!("{}", tx_json);
                } else {
                    eprintln!("Transaction with hash `{}` not found", tx_hash_str);
                }
            }
        },
    }
}
