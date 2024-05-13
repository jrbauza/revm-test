
use ethers_providers::{Http, Provider};

use revm::{
    db::{CacheDB, EthersDB},
    primitives::{address, Address, TransactTo, U256, ExecutionResult, Output},
    Evm,
};
use alloy_sol_types::{sol, SolCall, SolValue};
use anyhow::{Ok, anyhow};

pub struct ERC20Token {
    address : Address
}

pub struct IntegerDecimal {
    pub int_part: U256,
    pub decimal_part: U256
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
                tx.caller = address!("0000000000000000000000000000000000000001");
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

    pub fn decimals(&self, db: &mut CacheDB<EthersDB<Provider<Http>>>) -> anyhow::Result<U256> {
        sol! {
            function decimals() external view returns (uint);
        }

        let encoded = decimalsCall {}.abi_encode();

        let mut evm = Evm::builder()
            .with_db(db)
            .modify_tx_env(|tx: &mut revm::primitives::TxEnv| {
                tx.caller = address!("0000000000000000000000000000000000000001");
                tx.transact_to = TransactTo::Call(self.address);
                tx.data = encoded.into();
                tx.value = U256::from(0);
            })
            .build();

        let ref_tx = evm.transact_commit().unwrap();
        let decimals: U256 = match ref_tx {
            ExecutionResult::Success {
                output: Output::Call(value),
                ..
            } => <U256>::abi_decode(&value, false)?,
            result => return Err(anyhow!("'decimals' execution failed: {result:?}")),
        };
        Ok(decimals)
    }

    pub fn integer_decimal(&self, amount: U256, db: &mut CacheDB<EthersDB<Provider<Http>>>) -> anyhow::Result<IntegerDecimal> {
        let decimals = self.decimals(db).unwrap();
        let divisor = U256::from(10).pow(decimals);
        let decimal_part = amount.checked_rem(divisor).unwrap();
        let int_part = amount.checked_sub(decimal_part).unwrap().checked_div(divisor).unwrap();
        Ok(IntegerDecimal {
            int_part,
            decimal_part
        })
    }

    pub fn integer_balance_of(&self, account: Address, db: &mut CacheDB<EthersDB<Provider<Http>>>) -> anyhow::Result<U256> {
        let balance = self.balance_of(account, db).unwrap();
        let int_balance = self.integer_decimal(balance, db).unwrap();
        Ok(int_balance.int_part)
    }

    pub fn symbol(&self, db: &mut CacheDB<EthersDB<Provider<Http>>>) -> anyhow::Result<String> {
        sol! {
            function symbol() external view returns (string);
        }

        let encoded = symbolCall {}.abi_encode();

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
        let symbol: String = match ref_tx {
            ExecutionResult::Success {
                output: Output::Call(value),
                ..
            } => <String>::abi_decode(&value, false)?,
            result => return Err(anyhow!("'symbol' execution failed: {result:?}")),
        };
        Ok(symbol)
    }
}