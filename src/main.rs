mod erc20_token;
mod uniswap_v2_token_pool;
mod integer_decimal;

use erc20_token::ERC20Token;
use uniswap_v2_token_pool::UniswapV2TokenPool;

use anyhow::Ok;
//use ethers_core::types::BlockId;
use ethers_providers::{Http, Provider};
use revm::{
    db::{CacheDB, EthersDB},
    primitives::address
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut db = get_db();
    //let usdc_eth_pair_v3 = address!("88e6a0c2ddd26feeb64f039a2c41296fcb3f5640");

    //let usdc = address!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");
    //let weth = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");

    let account = address!("0000000000000000000000000000000000000000");//address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    //let _ = get_balance(&mut db, account).await;
    let alien_worlds = address!("888888848B652B3E3a0f34c96E00EEC0F3a23F72");
    let token = ERC20Token::new(alien_worlds);
    println!("Account: {:#?} balance: {:#?}",account, token.balance_of(account, &mut db)?.to_string());

    let uniswap_v2_usdc_eth_pair_on_mainnet = address!("b4e16d0168e52d35cacd2c6185b44281ec28c9dc");
    let uniswap_v2_wbtc_eth_pair_on_mainnet = address!("CBCdF9626bC03E24f779434178A73a0B4bad62eD");
    let uniswap_v2_usdc_wbtc_pair_on_mainnet = address!("99ac8cA7087fA4A2A1FB6357269965A2014ABc35");


    
    let uniswap_v2_usdc_eth_on_mainnet_pool = UniswapV2TokenPool::new(uniswap_v2_usdc_eth_pair_on_mainnet);
    let uniswap_v2_wbtc_eth_on_mainnet_pool = UniswapV2TokenPool::new(uniswap_v2_wbtc_eth_pair_on_mainnet);
    let uniswap_v2_usdc_wbtc_on_mainnet_pool = UniswapV2TokenPool::new(uniswap_v2_usdc_wbtc_pair_on_mainnet);

    let token0 = ERC20Token::new(uniswap_v2_usdc_eth_on_mainnet_pool.token_0(&mut db).unwrap());
    let token1 = ERC20Token::new(uniswap_v2_usdc_eth_on_mainnet_pool.token_1(&mut db).unwrap());

    println!("Token 0 Symbol: {:#?}, Reserves: {:#?}", token0.symbol(&mut db)?.to_string(), uniswap_v2_usdc_eth_on_mainnet_pool.token_0_amount(&mut db).to_string());
    println!("Token 1 Symbol: {:#?}, Reserves: {:#?}", token1.symbol(&mut db)?.to_string(), uniswap_v2_usdc_eth_on_mainnet_pool.token_1_amount(&mut db).to_string());

    println!("Token 0 / Token 1: {:#?}", uniswap_v2_usdc_eth_on_mainnet_pool.ratio(false, &mut db));
    println!("Token 1 / Token 0: {:#?}", uniswap_v2_usdc_eth_on_mainnet_pool.ratio(true, &mut db));
    println!("Reserves: {:#?} | {:#?} | {:#?}", uniswap_v2_usdc_eth_on_mainnet_pool.get_reserves(&mut db)?.0.to_string(), uniswap_v2_usdc_eth_on_mainnet_pool.get_reserves(&mut db)?.1.to_string(), uniswap_v2_usdc_eth_on_mainnet_pool.get_reserves(&mut db)?.2.to_string());
    Ok(())
}


fn get_db() -> CacheDB<EthersDB<Provider<Http>>> {
    let client = Arc::new(Provider::<Http>::try_from(
        "http://127.0.0.1:8545",
    ).expect("Could not connect to the client"));
    CacheDB::new(EthersDB::new(client.clone(), None/*Some(BlockId::from(19791181))*/).unwrap())
}

/*async fn get_balance(db: &mut CacheDB<EthersDB<Provider<Http>>>, account: Address) -> anyhow::Result<()> {
    let mut evm = Evm::builder()
        .with_db(db)
        .build();

    let account_info = evm.db_mut().basic(account)?;

    println!("Account address: {:#?}", account.to_string());
    //println!("Account nonce: {:#?}", account_info.unwrap().nonce.to_string());
    //println!("Account code hash: {:#?}", account_info.unwrap().code_hash.to_string());
    //println!("Account code: {:#?}", account_info.unwrap().code);
    //println!("Account storage: {:#?}", account_info.unwrap());
    println!("Account balance: {:#?}", account_info.unwrap().balance.to_string());
    Ok(())
}*/