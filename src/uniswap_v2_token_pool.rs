use ethers_providers::{Http, Provider};
use revm::{
    db::{CacheDB, EthersDB},
    primitives::{address, Address, TransactTo, U256, ExecutionResult, Output},
    Evm,
};
use alloy_sol_types::{sol, SolCall, SolValue};
use anyhow::{Ok, anyhow};

use crate::integer_decimal::IntegerDecimal;
use crate::erc20_token::ERC20Token;


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
                tx.caller = address!("0000000000000000000000000000000000000001");
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

    pub fn get_reserves(&self, db: &mut CacheDB<EthersDB<Provider<Http>>>) -> anyhow::Result<(U256, U256, U256)> {
        sol! {
            function getReserves() external view returns (uint112, uint112, uint32);
        }

        let encoded = getReservesCall {}.abi_encode();

        let mut evm = Evm::builder()
            .with_db(db)
            .modify_tx_env(|tx| {
                tx.caller = address!("0000000000000000000000000000000000000001");
                tx.transact_to = TransactTo::Call(self.address);
                tx.data = encoded.into();
                tx.value = U256::from(0);
            })
            .build();
        let ref_tx = evm.transact_commit().unwrap();
        let (reserve0, reserve1, timestamp) = match ref_tx {
            ExecutionResult::Success {
                output: Output::Call(value),
                ..
            } => <(U256, U256, U256)>::abi_decode(&value, false)?,
            result => return Err(anyhow!("'getReserves' execution failed: {result:?}")),
        };
        Ok((reserve0, reserve1, timestamp))
    }

    pub fn token_0_amount(&self, db: &mut CacheDB<EthersDB<Provider<Http>>>) -> IntegerDecimal {
        let reserves = self.get_reserves(db).unwrap();
        IntegerDecimal::new(reserves.0, ERC20Token::new(self.token_0(db).unwrap()).decimals(db).unwrap())
    }

    pub fn token_1_amount(&self, db: &mut CacheDB<EthersDB<Provider<Http>>>) -> IntegerDecimal {
        let reserves = self.get_reserves(db).unwrap();
        IntegerDecimal::new(reserves.1, ERC20Token::new(self.token_1(db).unwrap()).decimals(db).unwrap())
    }

    pub fn ratio(&self, inverse:bool, db: &mut CacheDB<EthersDB<Provider<Http>>>) -> f64 {
        let token0_amount = self.token_0_amount(db);
        let token1_amount = self.token_1_amount(db);

        if inverse {
            token1_amount.divide(&token0_amount)
        } else {
            token0_amount.divide(&token1_amount)
        }
    }
}