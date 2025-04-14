use std::f32::consts::E;
use std::time::Duration;
use std::{path::PathBuf, str::FromStr};

use race_bundlr_sdk::currency::CurrencyType;
use race_bundlr_sdk::{
    bundlr::BundlrBuilder, client::upload::run_upload, currency::solana::SolanaBuilder,
    error::BundlrError,
};
use reqwest::Url;

use bincode;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::Result as IoResult;
use std::io::{self, Read};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    name: String,
    age: u16,
}

impl Person {
    fn to_vec(&self) -> IoResult<Vec<u8>> {
        bincode::serialize(self).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}

fn read_from_file(file_path: &str) -> io::Result<Person> {
    // 打开文件
    let mut file = File::open(file_path)?;

    // 读取文件内容到 Vec<u8>
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // 反序列化 Vec<u8> 到 MyStruct
    let my_struct =
        bincode::deserialize(&buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(my_struct)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // let file_path = "/home/race/Iyrs-rust/rust-sdk-master/EDEeUQeJ74W4RdU7L-LPT_vDnKTvQbGiKdZENAFctkU";
    // match read_from_file(file_path) {
    //     Ok(p) => {
    //         println!("p : {:?}",p);
    //     },
    //     Err(e) => {
    //         println!("failed read from file : {}",e);
    //     }
    // }

    let person = Person {
        name: "wdnmd".to_string(),
        age: 999,
    };
    let buffer = person.to_vec().map_err(BundlrError::from).expect("REASON");

    let url_str = "https://node1.irys.xyz";
    let url = Url::parse(url_str).expect("Invalid URL");

    let work = run_upload(
        url,
        "bHvZk4Hqq19njiNAL5pVYBYd5LoRPGp92tYEFR4YbEsUNFCctj57Q2e8pkbyHDkD6jQua4BEip4TG9LE2hQn6GR",
        CurrencyType::Solana,
        buffer,
    );
    match tokio::time::timeout(Duration::from_millis(1000 * 30), work).await {
        Ok(res) => match res {
            Ok(ok) => println!("{}", ok),
            Err(err) => println!("[Err] {}", err),
        },
        Err(err) => println!("Error running task: {}", err),
    }

    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), BundlrError> {
//     let url = Url::parse("https://node1.bundlr.network").unwrap();
//     let currency = SolanaBuilder::new().wallet(
//         "kNykCXNxgePDjFbDWjPNvXQRa8U12Ywc19dFVaQ7tebUj3m7H4sF4KKdJwM7yxxb3rqxchdjezX9Szh8bLcQAjb")
//         .build()
//         .expect("Could not create Solana instance");
//     let mut bundlr = BundlrBuilder::new()
//         .url(url)
//         .currency(currency)
//         .fetch_pub_info()
//         .await?
//         .build()?;

//     let file = PathBuf::from_str("res/gen_bundles/bundle_2").unwrap();
//     //let file = PathBuf::from_str("res/test.json").unwrap();
//     let res = bundlr.upload_file(file).await;
//     match res {
//         Ok(()) => println!("[ok]"),
//         Err(err) => println!("[err] {}", err),
//     }
//     Ok(())
// }
