use reqwest::Error;
use tokio::main;

use crate::common::Message;

pub mod common;

#[main]
async fn main() -> Result<(), Error> {
    let url = "http://127.0.0.1:3030/people";
    let response = reqwest::get(url).await?.json::<Message>().await?;

    println!("{:?}", response);
    Ok(())
}
