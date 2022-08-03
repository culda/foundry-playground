use log::error;
use log::info;
use log::warn;

use dotenv::dotenv;
use web3::ethabi::ParamType;
use web3::ethabi::Token;
use web3::types::Transaction;
use web3::types::H256;
use web3::types::U256;

use std::env::VarError;
use std::str::FromStr;
use std::{env, process};

use web3::ethabi::decode;
use web3::transports::Http;
use web3::types::TransactionId;

fn parse_tx(tx: &Transaction) {
    // let fn_sig = "multicall(uint256,bytes[])";
    let res = decode(&[ParamType::Uint(256)], &tx.input.0);
    match res {
        Ok(vector) => {
            info!("tokens {:?}", vector);
            if vector.len() > 0 {
                let first = &vector[0];
                // let second = &vector[1];
                // let third = &vector[2];

                match first {
                    Token::Uint(x) => {
                        info!("first {}", first);
                    }
                    _ => {}
                }
                // match second {
                //     Token::Uint(x) => {
                //         info!("second {}", second);
                //     }
                //     _ => {}
                // }
                // match third {
                //     Token::Uint(x) => {
                //         info!("third {}", third);
                //     }
                //     _ => {}
                // }
            } else {
            }
        }
        Err(e) => {
            error!("{:?}", e);
        }
    };
}

fn get_rpc_endpoint() -> Result<String, VarError> {
    env::var("RPC_URL")
}

#[tokio::main]
async fn main() -> web3::Result {
    dotenv().ok();
    env_logger::init();

    let http_node_endpoint = match get_rpc_endpoint() {
        Ok(r) => r,
        Err(e) => {
            error!("could not find node, {:?}", e);
            process::exit(1)
        }
    };

    let hash: H256 = match H256::from_str(
        "0xda4deaadc11bd5df3069efd42d32171fdb390f473ff7b042f3819ae0e369b259",
    ) {
        Ok(r) => r,
        Err(e) => {
            error!("could not fetch tx, {:?}", e);
            process::exit(1)
        }
    };

    let tx_id = TransactionId::from(hash);

    let transport = Http::new(http_node_endpoint.as_str())?;
    let web3 = web3::Web3::new(transport);
    let res = web3.eth().transaction(tx_id).await?;

    if let Some(txn) = res {
        info!("{:?}", txn);
        parse_tx(&txn);
    } else {
        warn!("No transaction found");
    }
    Ok(())
}
