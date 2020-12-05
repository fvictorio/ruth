mod random;

use anyhow::{anyhow, Context, Result};
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
        number: String,
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
    /// Send a transaction
    Send {
        #[structopt(long, short)]
        from: Option<String>,

        #[structopt(long, short)]
        to: Option<String>,

        #[structopt(long, short)]
        value: Option<u64>,
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

fn string_to_address(address: &str) -> Result<Address> {
    let given_address = String::from(address);

    let address = if address.starts_with("0x") {
        &address[2..]
    } else {
        address
    };

    let address = hex::decode(address).context(format!("Invalid hex: `{}`", given_address))?;

    if address.len() != 20 {
        return Err(anyhow!(
            "Address `{}` has `{}` bytes, expected 20",
            given_address,
            address.len()
        ));
    }

    let mut address_bytes = [0u8; 20];

    for (i, b) in address.iter().enumerate() {
        address_bytes[i] = *b;
    }

    H160::try_from(address_bytes).context(format!("COuldn't convert to address"))
}

#[derive(Debug)]
pub struct RuthError(String);

async fn main_async() -> Result<(), RuthError> {
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
                let provider = Provider::<Http>::try_from(&network[..]).map_err(|err| {
                    RuthError(format!("Error trying to connect to {}: {}", &network, err))
                })?;

                if number == "number" {
                    let block_number = provider
                        .get_block_number()
                        .await
                        .map_err(|err| RuthError(format!("Error getting block number: {}", err)))?;

                    println!("{}", block_number);
                    return Ok(());
                }

                let number = number
                    .parse::<u64>()
                    .map_err(|err| {
                        RuthError(format!("Invalid block number `{}`: {}", number, err))
                    })?;

                let block = provider.get_block(number).await.map_err(|err| {
                    RuthError(format!(
                        "Error getting block `{}` from network `{}`: {}",
                        number, &network, err
                    ))
                })?;

                if let Some(block) = block {
                    let block_json = serde_json::to_string_pretty(&block)
                        .map_err(|err| RuthError(format!("Error parsing block object: {}", err)))?;

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

                let provider = Provider::<Http>::try_from(&network[..]).map_err(|err| {
                    RuthError(format!("Error trying to connect to {}: {}", &network, err))
                })?;

                let tx_hash = hex::decode(tx_hash).map_err(|err| {
                    RuthError(format!("Invalid tx_hash `{}`: {}", tx_hash_str, err))
                })?;

                assert!(tx_hash.len() == 32);

                let mut tx_hash_bytes = [0u8; 32];

                for (i, b) in tx_hash.iter().enumerate() {
                    tx_hash_bytes[i] = *b;
                }

                let tx_hash = H256::try_from(tx_hash_bytes).map_err(|err| {
                    RuthError(format!(
                        "Invalid transaction hash `{}`: {}",
                        tx_hash_str, err
                    ))
                })?;

                let tx = provider.get_transaction(tx_hash).await.map_err(|err| {
                    RuthError(format!(
                        "Error getting tx `{}` from network `{}`: {}",
                        tx_hash_str, &network, err
                    ))
                })?;

                if let Some(tx) = tx {
                    let tx_json = serde_json::to_string_pretty(&tx)
                        .map_err(|err| RuthError(format!("Error parsing block object: {}", err)))?;

                    println!("{}", tx_json);
                } else {
                    eprintln!("Transaction with hash `{}` not found", tx_hash_str);
                }
            }
        },
        RuthCommands::Send { from, to, value } => {
            let provider = Provider::<Http>::try_from(&network[..]).map_err(|err| {
                RuthError(format!("Error trying to connect to {}: {}", &network, err))
            })?;

            let accounts = provider.get_accounts().await.map_err(|err| {
                RuthError(format!(
                    "Error getting accounts from network `{}`: {}",
                    &network, err
                ))
            })?;

            if accounts.is_empty() {
                return Err(RuthError(String::from("The node has no unlocked accounts")));
            }

            let from_address = if let Some(f) = from {
                string_to_address(&f).map_err(|err| {
                    RuthError(format!("Invalid \"from\" address `{}`: {}", &f, err))
                })?
            } else {
                accounts[0]
            };

            let to_address = if let Some(t) = to {
                string_to_address(&t)
                    .map_err(|err| RuthError(format!("Invalid \"to\" address `{}`: {}", &t, err)))?
            } else {
                accounts[0]
            };

            let value = value.map(|v| U256::from(v));

            let tx = provider
                .send_transaction(
                    TransactionRequest {
                        from: Some(from_address),
                        to: Some(NameOrAddress::from(to_address)),
                        value,
                        gas: None,
                        gas_price: None,
                        data: None,
                        nonce: None,
                    },
                    None,
                )
                .await
                .map_err(|err| RuthError(format!("Failed to send transaction: {}", err)))?;

            println!("0x{}", hex::encode(tx.to_fixed_bytes()));
        }
    }

    Ok(())
}

pub fn main() -> Result<(), RuthError> {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(main_async())
}
