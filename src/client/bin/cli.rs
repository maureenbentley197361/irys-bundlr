use std::time::Duration;

use race_bundlr_sdk::{
    client::{
        balance::run_balance, fund::run_fund, price::run_price, upload::run_upload,
        withdraw::run_withdraw,
    },
    currency::CurrencyType,
};

use reqwest::Url;

const DEFAULT_BYTE_AMOUNT: u64 = 1;
const DEFAULT_TIMEOUT: u64 = 1000 * 30; //30 secs
const DEFAULT_TIMEOUT_FUND: u64 = 1000 * 60 * 30; //30 mins

struct Args {
    command: Command,
}

enum Command {
    ///Gets the specified user's balance for the current Bundlr node
    Balance {
        //Address to query balance
        address: String,

        //Timeout for operation
        timeout: Option<u64>,

        //Host address
        host: Url,

        //Currency type
        currency: CurrencyType,
    },
    ///Funds your account with the specified amount of atomic units
    Fund {
        //Amounts, in winston, to send in funding
        amount: u64,

        //Timeout for operation
        timeout: Option<u64>,

        //Path to wallet
        wallet: String,

        //Host address
        host: Url,

        //Currency type
        currency: CurrencyType,
    },
    ///Sends a fund withdrawal request
    Withdraw {
        //Amounts, in winston, to send in withdraw
        amount: u64,

        //Timeout for operation
        timeout: Option<u64>,

        //Path to wallet
        wallet: String,

        //Host address
        host: Url,

        //Currency type
        currency: CurrencyType,
    },
    ///Uploads a specified file
    Upload {
        //Timeout for operation
        timeout: Option<u64>,

        //Path to wallet
        wallet: String,

        //Host address
        host: Url,

        //Currency type
        currency: CurrencyType,
    },
    ///Uploads a folder (with a manifest)
    UploadDir {},
    ///Check how much of a specific currency is required for an upload of <amount> bytes
    Price {
        //Amounts of bytes to calculate pricing
        byte_amount: Option<u64>,

        //Timeout for operation
        timeout: Option<u64>,

        //Host address
        host: Url,

        //Currency type
        currency: CurrencyType,
    },
}

impl Command {
    async fn execute(self) {
        match self {
            Command::Balance {
                address,
                timeout,
                host,
                currency,
            } => {
                let work = run_balance(host, &address, currency);
                let timeout = timeout.unwrap_or(DEFAULT_TIMEOUT);
                match tokio::time::timeout(Duration::from_millis(timeout), work).await {
                    Ok(res) => match res {
                        Ok(ok) => println!("[Ok] {}", ok),
                        Err(err) => println!("[Err] {}", err),
                    },
                    Err(err) => println!("Error running task: {}", err),
                }
            }
            Command::Fund {
                amount,
                timeout,
                wallet,
                host,
                currency,
            } => {
                let work = run_fund(amount, host, &wallet, currency);
                let timeout = timeout.unwrap_or(DEFAULT_TIMEOUT_FUND);
                match tokio::time::timeout(Duration::from_millis(timeout), work).await {
                    Ok(res) => match res {
                        Ok(ok) => println!("[Ok] {}", ok),
                        Err(err) => println!("[Err] {}", err),
                    },
                    Err(err) => println!("Error running task: {}", err),
                }
            }
            Command::Withdraw {
                amount,
                timeout,
                wallet,
                host,
                currency,
            } => {
                let work = run_withdraw(amount, host, &wallet, currency);
                let timeout = timeout.unwrap_or(DEFAULT_TIMEOUT);
                match tokio::time::timeout(Duration::from_millis(timeout), work).await {
                    Ok(res) => match res {
                        Ok(ok) => println!("[Ok] {}", ok),
                        Err(err) => println!("[Err] {}", err),
                    },
                    Err(err) => println!("Error running task: {}", err),
                }
            }
            Command::Upload {
                timeout,
                wallet,
                host,
                currency,
            } => {
                let bytes: Vec<u8> = Vec::new();
                let work = run_upload(host, &wallet, currency, bytes);
                let timeout = timeout.unwrap_or(DEFAULT_TIMEOUT);
                match tokio::time::timeout(Duration::from_millis(timeout), work).await {
                    Ok(res) => match res {
                        Ok(ok) => println!("[Ok] {}", ok),
                        Err(err) => println!("[Err] {}", err),
                    },
                    Err(err) => println!("Error running task: {}", err),
                }
            }
            Command::UploadDir {} => todo!(),
            Command::Price {
                byte_amount,
                timeout,
                host,
                currency,
            } => {
                let byte_amount = byte_amount.unwrap_or(DEFAULT_BYTE_AMOUNT);
                let work = run_price(host, currency, byte_amount);
                let timeout = timeout.unwrap_or(DEFAULT_TIMEOUT);
                match tokio::time::timeout(Duration::from_millis(timeout), work).await {
                    Ok(res) => match res {
                        Ok(ok) => println!("[Ok] {}", ok),
                        Err(err) => println!("[Err] {}", err),
                    },
                    Err(err) => println!("Error running task: {}", err),
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // let args = Args::parse();

    // args.command.execute().await;
}
