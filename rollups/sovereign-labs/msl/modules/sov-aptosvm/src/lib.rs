#[cfg(feature = "experimental")]
pub mod call;
#[cfg(feature = "experimental")]
pub mod evm;
#[cfg(feature = "experimental")]
pub mod genesis;
#[cfg(feature = "native")]
#[cfg(feature = "experimental")]
pub mod query;
#[cfg(feature = "experimental")]
#[cfg(test)]
mod tests;
<<<<<<< HEAD

#[cfg(feature = "experimental")]
pub use aptos_experimental::{AptosVm, AptosVmConfig};

#[cfg(feature = "experimental")]
extern crate dirs;

#[cfg(feature = "experimental")]
mod aptos_experimental {

    use anyhow::anyhow;

=======
#[cfg(feature = "experimental")]
pub use experimental::{AccountData, Evm, EvmConfig};

#[cfg(feature = "experimental")]
mod experimental {
    use revm::primitives::{KECCAK_EMPTY, U256};
>>>>>>> d0c9acb70a30c9f4e7b360459890efc3e9f1b236
    use sov_modules_api::Error;
    use sov_modules_macros::ModuleInfo;
    use sov_state::WorkingSet;

<<<<<<< HEAD
    use aptos_types::transaction::{Transaction};
    use aptos_db::AptosDB;
    use aptos_storage_interface::DbReaderWriter;
    use aptos_types::validator_signer::ValidatorSigner;

    use aptos_executor::block_executor::BlockExecutor;
    use aptos_executor::db_bootstrapper::{generate_waypoint, maybe_bootstrap};
    use aptos_executor_types::BlockExecutorTrait;
    use aptos_vm::AptosVM;
    // use anyhow::{Error};

    use borsh::{BorshDeserialize, BorshSerialize};

    use std::sync::Arc;

    #[derive(Clone)]
    pub struct AptosVmConfig {
        pub data: Vec<u8>,
=======
    use super::evm::db::EvmDb;
    use super::evm::transaction::BlockEnv;
    use super::evm::{DbAccount, EthAddress};
    use crate::evm::{Bytes32, EvmTransaction};

    #[derive(Clone)]
    pub struct AccountData {
        pub address: EthAddress,
        pub balance: Bytes32,
        pub code_hash: Bytes32,
        pub code: Vec<u8>,
        pub nonce: u64,
    }

    impl AccountData {
        pub fn empty_code() -> [u8; 32] {
            KECCAK_EMPTY.to_fixed_bytes()
        }

        pub fn balance(balance: u64) -> Bytes32 {
            U256::from(balance).to_le_bytes()
        }
    }

    #[derive(Clone)]
    pub struct EvmConfig {
        pub data: Vec<AccountData>,
>>>>>>> d0c9acb70a30c9f4e7b360459890efc3e9f1b236
    }

    #[allow(dead_code)]
    #[derive(ModuleInfo, Clone)]
<<<<<<< HEAD
    pub struct AptosVm<C: sov_modules_api::Context> {
=======
    pub struct Evm<C: sov_modules_api::Context> {
>>>>>>> d0c9acb70a30c9f4e7b360459890efc3e9f1b236
        #[address]
        pub(crate) address: C::Address,

        #[state]
<<<<<<< HEAD
        pub(crate) db_path: sov_state::StateValue<String>,

        // TODO: this may be redundant with address
        #[state]
        pub(crate) validator_signer: sov_state::StateValue<Vec<u8>>, // TODO: fix validator signer incompatability

        // This is string because we are using transaction.hash: https://github.com/movemntdev/aptos-core/blob/112ad6d8e229a19cfe471153b2fd48f1f22b9684/crates/indexer/src/models/transactions.rs#L31
        #[state]
        pub(crate) transactions: sov_state::StateMap<String, Vec<u8>>, // TODO: fix Transaction serialiation incompatability
    }

    impl<C: sov_modules_api::Context> sov_modules_api::Module for AptosVm<C> {
        type Context = C;

        type Config = AptosVmConfig;
=======
        pub(crate) accounts: sov_state::StateMap<EthAddress, DbAccount>,

        #[state]
        pub(crate) block_env: sov_state::StateValue<BlockEnv>,

        #[state]
        pub(crate) transactions: sov_state::StateMap<Bytes32, EvmTransaction>,
    }

    impl<C: sov_modules_api::Context> sov_modules_api::Module for Evm<C> {
        type Context = C;

        type Config = EvmConfig;
>>>>>>> d0c9acb70a30c9f4e7b360459890efc3e9f1b236

        type CallMessage = super::call::CallMessage;

        fn genesis(
            &self,
            config: &Self::Config,
            working_set: &mut WorkingSet<C::Storage>,
        ) -> Result<(), Error> {
<<<<<<< HEAD

            Ok(self.init_module(config, working_set)?)

=======
            Ok(self.init_module(config, working_set)?)
>>>>>>> d0c9acb70a30c9f4e7b360459890efc3e9f1b236
        }

        fn call(
            &self,
            msg: Self::CallMessage,
            context: &Self::Context,
            working_set: &mut WorkingSet<C::Storage>,
        ) -> Result<sov_modules_api::CallResponse, Error> {
<<<<<<< HEAD

            Ok(self.execute_call(msg.serialized_tx, context, working_set)?)

        }
    }

 

    impl<C: sov_modules_api::Context> AptosVm<C> {

        pub(crate) fn get_db(
            &self,
            working_set: &mut WorkingSet<C::Storage>,
        ) -> Result<
            DbReaderWriter, 
            Error
        > {

            let path = self.db_path.get(working_set).ok_or(
                anyhow::Error::msg("Database path is not set.")
            )?;
            // TODO: swap for non-test db
            // TODO: swap for celestia DA
            Ok(DbReaderWriter::new(AptosDB::new_for_sov(path.as_str())))

        }

        pub(crate) fn get_executor(
            &self,
            working_set: &mut WorkingSet<C::Storage>,
        ) -> Result<
            BlockExecutor<AptosVM>, 
            Error
        > {

            let db = self.get_db(working_set)?;
            Ok(BlockExecutor::new(db.clone()))

        }

    }

=======
            Ok(self.execute_call(msg.tx, context, working_set)?)
        }
    }

    impl<C: sov_modules_api::Context> Evm<C> {
        pub(crate) fn get_db<'a>(
            &self,
            working_set: &'a mut WorkingSet<C::Storage>,
        ) -> EvmDb<'a, C> {
            EvmDb::new(self.accounts.clone(), working_set)
        }
    }
>>>>>>> d0c9acb70a30c9f4e7b360459890efc3e9f1b236
}
