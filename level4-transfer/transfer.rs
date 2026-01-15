use ethers::prelude::*;
use std::convert::TryFrom;
use dotenv::dotenv;
use std::env;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 加载 .env 文件中的配置
    dotenv().ok();
    
    // 2. 连接 Arbitrum Sepolia 节点
    let provider = Provider::<Http>::try_from(
        "https://sepolia-rollup.arbitrum.io/rpc"
    )?;

    // 3. 从环境变量获取私钥
    let private_key_raw = env::var("PRIVATE_KEY").expect("请在 .env 文件中设置 PRIVATE_KEY");
    
    // 自动清洗私钥
    let private_key = private_key_raw.trim().trim_start_matches("0x");

    let to_address_str = env::var("TO_ADDRESS").expect("请在 .env 文件中设置 TO_ADDRESS");
    let to_address = Address::from_str(&to_address_str)?;

    // 4. 创建钱包
    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(421614u64);
    
    // 5. 将钱包和 Provider 组合成一个 Client
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    println!("-------------------------------------------");
    println!(" 发送方地址: {:?}", client.address());
    println!(" 接收方地址: {:?}", to_address);
    
    // 6. 获取当前建议的 Gas Price
    let current_gas_price = provider.get_gas_price().await?;
    // 为了防止价格波动导致交易失败，手动增加 20% 的 Gas 费，算法：current_gas_price * 1.2
    let adjusted_gas_price = current_gas_price * 120 / 100;

    println!(" 当前网络价格: {} Gwei", ethers::utils::format_units(current_gas_price, "gwei")?);
    println!(" 调整后支付价格: {} Gwei (已加 20% 缓冲)", ethers::utils::format_units(adjusted_gas_price, "gwei")?);

    // 7. 构建交易对象
    let amount_in_eth = "0.001"; 
    let amount_wei = ethers::utils::parse_ether(amount_in_eth)?;

    let tx = TransactionRequest::new()
        .to(to_address)
        .value(amount_wei)
        .gas_price(adjusted_gas_price); // 这里使用调整后的价格

    println!(" 准备发送 {} ETH...", amount_in_eth);

    // 8. 发送交易并等待上链确认
    let pending_tx = client.send_transaction(tx, None).await?;
    
    println!(" 交易已发送，等待链上确认... (Hash: {:?})", pending_tx.tx_hash());

    // 等待交易被打包
    let receipt = pending_tx.await?;

    println!("-------------------------------------------");
    if let Some(receipt) = receipt {
        println!(" 交易成功！");
        println!(" 交易哈希 (Tx Hash): {:?}", receipt.transaction_hash);
        println!(" 区块高度: {:?}", receipt.block_number);
        println!(" 实际消耗 Gas: {:?}", receipt.gas_used);
        println!(" 浏览器查询: https://sepolia.arbiscan.io/tx/{:?}", receipt.transaction_hash);
    } else {
        println!(" 交易未被打包 (可能还在等待中)");
    }
    println!("-------------------------------------------");

    Ok(())
}