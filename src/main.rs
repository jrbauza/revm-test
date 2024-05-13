mod erc20_token;
mod uniswap_v2_token_pool;

use erc20_token::ERC20Token;
use uniswap_v2_token_pool::UniswapV2TokenPool;

use anyhow::Ok;
//use ethers_core::types::BlockId;
use ethers_providers::{Http, Provider};
use revm::{
    db::{CacheDB, EthersDB},
    primitives::{address, Address},
    Database, Evm,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut db = get_db();
    //let usdc_eth_pair_v3 = address!("88e6a0c2ddd26feeb64f039a2c41296fcb3f5640");
    let usdc_eth_pair = address!("b4e16d0168e52d35cacd2c6185b44281ec28c9dc");
    let alien_worlds = address!("888888848B652B3E3a0f34c96E00EEC0F3a23F72");
    //let usdc = address!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");
    //let weth = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");

    let account = address!("0000000000000000000000000000000000000000");//address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    //let _ = get_balance(&mut db, account).await;
    let token = ERC20Token::new(alien_worlds);


    let pool = UniswapV2TokenPool::new(usdc_eth_pair);

    let token0 = ERC20Token::new(pool.token_0(&mut db)?);
    let token1 = ERC20Token::new(pool.token_1(&mut db)?);

    println!("Account: {:#?} balance: {:#?}",account, token.balance_of(account, &mut db)?.to_string());
    let token0_reserves = pool.get_reserves(&mut db)?.0;
    let token1_reserves = pool.get_reserves(&mut db)?.1;

    let token0_amount: erc20_token::IntegerDecimal = token0.integer_decimal(token0_reserves, &mut db)?;
    let token1_amount: erc20_token::IntegerDecimal = token1.integer_decimal(token1_reserves, &mut db)?;

    println!("Token 0 Symbol: {:#?}, Reserves: {:#?},{:#?}", token0.symbol(&mut db)?.to_string(), token0_amount.int_part.to_string(), token0_amount.decimal_part.to_string());
    println!("Token 1 Symbol: {:#?}, Reserves: {:#?},{:#?}", token1.symbol(&mut db)?.to_string(), token1_amount.int_part.to_string(), token1_amount.decimal_part.to_string());

    println!("Reserves: {:#?} | {:#?} | {:#?}", pool.get_reserves(&mut db)?.0.to_string(), pool.get_reserves(&mut db)?.1.to_string(), pool.get_reserves(&mut db)?.2.to_string());
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