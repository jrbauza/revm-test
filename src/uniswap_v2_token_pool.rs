use ethers_providers::{Http, Provider};
use revm::{
    db::{CacheDB, EthersDB},
    primitives::{address, Address, TransactTo, U256, ExecutionResult, Output},
    Evm,
};
use alloy_sol_types::{sol, SolCall, SolValue};
use anyhow::{Ok, anyhow};

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
}