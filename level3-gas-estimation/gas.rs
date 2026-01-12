use ethers::prelude::*;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 连接到Arbitrum Sepolia测试网
    let provider = Provider::<Http>::try_from(
        "https://sepolia-rollup.arbitrum.io/rpc"
    )?;

    println!("正在获取 Arbitrum 网络实时 Gas 信息...");

    // 2. 动态获取网络当前的Gas Price（异步操作，从链上实时拉取数据）
    let gas_price = provider.get_gas_price().await?;

    // 3. 定义基础转账的Gas Limit（普通 ETH 转账通常固定消耗 21,000 Gas）
    let gas_limit = U256::from(21000);

    // 4. 计算预估Gas费公式：Gas Fee = Gas Price(单价)*Gas Limit(消耗量)
    let estimated_fee_wei = gas_price * gas_limit;

    // 5.格式化输出
    let gas_price_gwei = ethers::utils::format_units(gas_price, "gwei")?;
    let fee_eth = ethers::utils::format_units(estimated_fee_wei, "ether")?;
    let fee_gwei = ethers::utils::format_units(estimated_fee_wei, "gwei")?;

    println!("------------------------------------------------");
    println!("实时Gas Price: {} Gwei", gas_price_gwei);
    println!("基础转账Gas Limit: {}", gas_limit);
    println!("------------------------------------------------");
    println!("预估转账Gas 费 (计算逻辑: Price * Limit):");
    println!("   = {} ETH", fee_eth);
    println!("   = {} Gwei", fee_gwei);
    println!("------------------------------------------------");

    Ok(())
}