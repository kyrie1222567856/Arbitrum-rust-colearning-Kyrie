use ethers::prelude::*;
use std::convert::TryFrom;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(
        "https://sepolia-rollup.arbitrum.io/rpc"
    )?;

    let wallet_address = "0x45E288d718B1142E6310DC92A3E4aF2Cc568f1A4"; 
    let address = Address::from_str(wallet_address)?;

    println!("正在查询地址: {}", wallet_address);

    let balance_wei = provider.get_balance(address, None).await?;
    let balance_eth = ethers::utils::format_units(balance_wei, "ether")?;

    println!("当前余额: {} ETH", balance_eth);

    Ok(())
}