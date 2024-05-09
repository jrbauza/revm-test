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