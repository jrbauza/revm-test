use anyhow::{Ok, anyhow};
use ethers_core::types::BlockId;
use ethers_providers::{Http, Provider};
use revm::{
    db::{CacheDB, EthersDB},
    primitives::{address, Address, TransactTo, U256, ExecutionResult, Output},
    Database, Evm,
};
use std::sync::Arc;
use alloy_sol_types::{sol, SolCall, SolValue};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut db = get_db();
    let usdc_eth_pair = address!("88e6a0c2ddd26feeb64f039a2c41296fcb3f5640");
    let alien_worlds = address!("888888848B652B3E3a0f34c96E00EEC0F3a23F72");
    let usdc = address!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");

    let account = address!("0000000000000000000000000000000000000000");//address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    //let _ = get_balance(&mut db, account).await;
    let token = ERC20Token::new(alien_worlds);

    let pool = UniswapV2TokenPool::new(usdc_eth_pair);

    let token_0 = pool.token_0(&mut db)?;

    println!("Account balance: {:#?}", token.balance_of(account, &mut db)?.to_string());
    println!("Token 0: {:#?}", token_0.to_string());
    Ok(())

}


fn get_db() -> CacheDB<EthersDB<Provider<Http>>> {
    let client = Arc::new(Provider::<Http>::try_from(
        "http://127.0.0.1:8545",
    ).expect("Could not connect to the client"));
    CacheDB::new(EthersDB::new(client.clone(), None/*Some(BlockId::from(19791181))*/).unwrap())
}

async fn get_balance(db: &mut CacheDB<EthersDB<Provider<Http>>>, account: Address) -> anyhow::Result<()> {
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
}


pub struct UniswapV2TokenPool {
    address: Address
}

impl UniswapV2TokenPool {
    pub fn new(address: Address) -> Self {
        Self {
            address
        }
    }
    
    pub fn token_0(&self, db: &mut CacheDB<EthersDB<Provider<Http>>>) -> anyhow::Result<Address> {
        sol! {
            function token0() external view returns (address);
        }

        let encoded = token0Call {}.abi_encode();

        let mut evm = Evm::builder()
            .with_db(db)
            .modify_tx_env(|tx| {
                tx.caller = address!("0000000000000000000000000000000000000000");
                tx.transact_to = TransactTo::Call(self.address);
                tx.data = encoded.into();
                tx.value = U256::from(0);
            })
            .build();
        let ref_tx = evm.transact_commit().unwrap();
        let token_0: Address = match ref_tx {
            ExecutionResult::Success {
                output: Output::Call(value),
                ..
            } => <Address>::abi_decode(&value, false)?,
            result => return Err(anyhow!("'token0' execution failed: {result:?}")),
        };
        Ok(token_0)
    }

    pub fn token_1(&self, db: &mut CacheDB<EthersDB<Provider<Http>>>) -> anyhow::Result<Address> {
        sol! {
            function token1() external view returns (address);
        }

        let encoded = token1Call {}.abi_encode();

        let mut evm = Evm::builder()
            .with_db(db)
            .modify_tx_env(|tx| {
                tx.caller = address!("0000000000000000000000000000000000000000");
                tx.transact_to = TransactTo::Call(self.address);
                tx.data = encoded.into();
                tx.value = U256::from(0);
            })
            .build();
        let ref_tx = evm.transact_commit().unwrap();
        let token_1: Address = match ref_tx {
            ExecutionResult::Success {
                output: Output::Call(value),
                ..
            } => <Address>::abi_decode(&value, false)?,
            result => return Err(anyhow!("'token1' execution failed: {result:?}")),
        };
        Ok(token_1)
    }
}

pub struct ERC20Token {
    address : Address
}

impl ERC20Token {
    pub fn new(address: Address) -> Self {
        Self {
            address
        }
    }

    pub fn balance_of(&self, account: Address, db: &mut CacheDB<EthersDB<Provider<Http>>>) -> anyhow::Result<U256> {
        sol! {
            function balanceOf(address account) external view returns (uint);
        }

        let encoded = balanceOfCall { account }.abi_encode();

        let mut evm = Evm::builder()
            .with_db(db)
            .modify_tx_env(|tx| {
                tx.caller = account;
                tx.transact_to = TransactTo::Call(self.address);
                tx.data = encoded.into();
                tx.value = U256::from(0);
            })
            .build();

        let ref_tx = evm.transact_commit().unwrap();
        let balance: U256 = match ref_tx {
            ExecutionResult::Success {
                output: Output::Call(value),
                ..
            } => <U256>::abi_decode(&value, false)?,
            result => return Err(anyhow!("'balanceOf' execution failed: {result:?}")),
        };

        Ok(balance)
    }
}