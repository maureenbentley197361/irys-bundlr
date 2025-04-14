use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::io::Result as IoResult;

use crate::{
    bundlr::BundlrBuilder,
    consts::VERSION,
    currency::{
        arweave::ArweaveBuilder, ethereum::EthereumBuilder, solana::SolanaBuilder, CurrencyType,
    },
    error::BundlrError,
    tags::Tag,
};
use reqwest::Url;

#[derive(Serialize, Deserialize)]
pub struct Person {
    name: String,
    age: u16,
}

impl Person {
    fn to_vec(&self) -> IoResult<Vec<u8>> {
        bincode::serialize(self).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}

//   Result<String, BundlrError>

pub async fn run_upload(
    url: Url,
    wallet: &str,
    currency: CurrencyType,
    buffer: Vec<u8>,
) -> Result<String, BundlrError>
where
    Result<String,BundlrError>: Send

{
    // ***************** test
    // let f = File::open(file_path.clone()).expect("Invalid file path");
    // let mut reader = BufReader::new(f);
    // let mut buffer = Vec::new();

    // // Read file into vector.
    // reader.read_to_end(&mut buffer)?;

    let base_tag = Tag::new("User-Agent", &format!("bundlr-sdk-rs/{}", VERSION));
    println!("buffer test");

    println!("buffer = {:?}", buffer.clone());
    match currency {
        CurrencyType::Arweave => {
            let wallet = PathBuf::from_str(wallet)
                .map_err(|err| BundlrError::ParseError(err.to_string()))?;
            let currency = ArweaveBuilder::new().keypair_path(wallet).build()?;
            let bundlr = BundlrBuilder::new()
                .url(url)
                .currency(currency)
                .fetch_pub_info()
                .await?
                .build()?;
            let mut tx = bundlr.create_transaction(buffer, vec![base_tag])?;
            let sig = bundlr.sign_transaction(&mut tx).await;
            assert!(sig.is_ok());
            match bundlr.send_transaction(tx).await {
                Ok(res) => Ok(format!("{}", res)),
                Err(err) => Err(BundlrError::UploadError(err.to_string())),
            }
        }
        CurrencyType::Solana => {
            let currency = SolanaBuilder::new().wallet(wallet).build()?;
            let bundlr = BundlrBuilder::new()
                .url(url)
                .currency(currency)
                .fetch_pub_info()
                .await?
                .build()?;
            let mut tx = bundlr.create_transaction(buffer, vec![base_tag])?;
            let sig = bundlr.sign_transaction(&mut tx).await;
            assert!(sig.is_ok());
            match bundlr.send_transaction(tx).await {
                Ok(res) => Ok(format!("{}", res)),
                Err(err) => Err(BundlrError::UploadError(err.to_string())),
            }
        }
        CurrencyType::Ethereum => {
            let currency = EthereumBuilder::new().wallet(wallet).build()?;
            let bundlr = BundlrBuilder::new()
                .url(url)
                .currency(currency)
                .fetch_pub_info()
                .await?
                .build()?;
            let mut tx = bundlr.create_transaction(buffer, vec![base_tag])?;
            let sig = bundlr.sign_transaction(&mut tx).await;
            assert!(sig.is_ok());
            match bundlr.send_transaction(tx).await {
                Ok(res) => Ok(format!("{}", res)),
                Err(err) => Err(BundlrError::UploadError(err.to_string())),
            }
        }
        CurrencyType::Erc20 => todo!(),
        CurrencyType::Cosmos => todo!(),
    }
}
