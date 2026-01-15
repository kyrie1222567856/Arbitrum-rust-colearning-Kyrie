use ethers::prelude::*;
use std::sync::Arc;
use std::convert::TryFrom;

// 1. 定义合约接口 (ABI)
abigen!(
    IMyContract,
    r#"[
        function name() external view returns (string)
        function symbol() external view returns (string)
        function decimals() external view returns (uint8)
    ]"#
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 2. 连接 Arbitrum Sepolia 测试网
    let provider = Provider::<Http>::try_from(
        "https://sepolia-rollup.arbitrum.io/rpc"
    )?;
    // 用 Arc 包裹 provider，因为合约实例需要共享所有权
    let client = Arc::new(provider);

    // 3. 设置目标合约地址，Chainlink (LINK) 的地址：
    let contract_address = "0xb1D4538B4571d411F07960EF2838Ce337FE1E80E".parse::<Address>()?;

    // 4. 创建合约实例
    let contract = IMyContract::new(contract_address, client);

    println!("正在连接合约: {:?}", contract_address);
    println!("正在读取链上数据...");
    println!("-------------------------------------------");

    // 5. 调用合约方法 (只读操作，不需要 Gas)，.call().await 是发起查询的标准写法
    
    let name = contract.name().call().await?;
    let symbol = contract.symbol().call().await?;
    let decimals = contract.decimals().call().await?;

    // 6. 输出结果
    println!(" 合约交互成功！");
    println!(" 合约名称 (Name):   {}", name);
    println!(" 代币符号 (Symbol): {}", symbol);
    println!(" 精度 (Decimals):   {}", decimals);
    println!("-------------------------------------------");

    Ok(())
}