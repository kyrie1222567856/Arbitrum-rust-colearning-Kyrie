use ethers::prelude::*;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let provider = Provider::<Http>::try_from(
        "https://sepolia-rollup.arbitrum.io/rpc"
    )?;

    println!("正在连接 Arbitrum Sepolia 测试网...");

    let block_number = provider.get_block_number().await?;

    let chain_id = provider.get_chainid().await?;

    println!("连接成功！");
    println!("当前网络 Chain ID: {}", chain_id);
    println!("当前区块高度: {}", block_number);

    Ok(())
}