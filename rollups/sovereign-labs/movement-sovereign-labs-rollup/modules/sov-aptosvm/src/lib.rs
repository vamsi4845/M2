pub mod call;
pub mod genesis;
#[cfg(feature = "native")]
pub mod query;
#[cfg(test)]
mod tests;

pub use aptos::{AptosVm, AptosVmConfig};

extern crate dirs;

mod aptos {

    use std::str::FromStr;

    use anyhow::anyhow;

    use sov_modules_api::Error;
    use sov_modules_macros::ModuleInfo;
    use sov_state::WorkingSet;

    use aptos_types::transaction::{Transaction};
    use aptos_types::waypoint::{Waypoint};
    use aptos_db::AptosDB;
    use aptos_storage_interface::DbReaderWriter;
    use aptos_types::validator_signer::ValidatorSigner;

    use aptos_executor::block_executor::BlockExecutor;
    use aptos_executor::db_bootstrapper::{generate_waypoint, maybe_bootstrap};
    use aptos_executor_types::BlockExecutorTrait;
    use aptos_vm::AptosVM;
    use aptos_crypto::{HashValue};
    use working_set_aptos_state_view::WorkingSetAptosStateView;
    // use anyhow::{Error};

    use borsh::{BorshDeserialize, BorshSerialize};

    use std::sync::Arc;

    #[derive(Clone)]
    pub struct AptosVmConfig {
        pub data: Vec<u8>,
    }

    #[allow(dead_code)]
    #[derive(ModuleInfo, Clone)]
    pub struct AptosVm<C: sov_modules_api::Context> {
        #[address]
        pub(crate) address: C::Address,

        // #[cfg(feature = "aptos-consensus")]
        #[state]
        pub(crate) db_path: sov_state::StateValue<String>,

        // TODO: this may be redundant with address
        // #[cfg(feature = "aptos-consensus")]
        #[state]
        pub(crate) validator_signer: sov_state::StateValue<Vec<u8>>, // TODO: fix validator signer incompatability

        // This is string because we are using transaction.hash: https://github.com/movemntdev/aptos-core/blob/112ad6d8e229a19cfe471153b2fd48f1f22b9684/crates/indexer/src/models/transactions.rs#L31
        // #[cfg(feature = "aptos-consensus")]
        #[state]
        pub(crate) transactions: sov_state::StateMap<String, Vec<u8>>, // TODO: fix Transaction serialiation incompatability

        // #[cfg(feature = "aptos-consensus")]
        #[state]
        pub(crate) genesis_hash: sov_state::StateValue<Vec<u8>>, // TODO: fix genesis serialiation incompatability

        // #[cfg(feature = "aptos-consensus")]
        #[state]
        pub(crate) waypoint: sov_state::StateValue<String>, // TODO: fix waypoint serialiation incompatability

        // #[cfg(feature = "aptos-consensus")]
        #[state]
        pub(crate) known_version : sov_state::StateValue<u64>,

    }

    impl<C: sov_modules_api::Context> sov_modules_api::Module for AptosVm<C> {
        type Context = C;

        type Config = AptosVmConfig;

        type CallMessage = super::call::CallMessage;

        fn genesis(
            &self,
            config: &Self::Config,
            working_set: &mut WorkingSet<C::Storage>,
        ) -> Result<(), Error> {

            Ok(self.init_module(config, working_set)?)

        }

        fn call(
            &self,
            msg: Self::CallMessage,
            context: &Self::Context,
            working_set: &mut WorkingSet<C::Storage>,
        ) -> Result<sov_modules_api::CallResponse, Error> {

            Ok(self.execute_call(msg.serialized_txs, context, working_set)?)

        }
    }

 

    impl<C: sov_modules_api::Context> AptosVm<C> {

        pub (crate) fn get_validator_signer(
            &self,
            working_set: &mut WorkingSet<C::Storage>,
        ) -> Result<
            ValidatorSigner, 
            Error
        > {
            
            let serialized_validator_signer = self.validator_signer.get(working_set).ok_or(
                anyhow::Error::msg("Validator signer is not set.")
            )?;

            // TODO: seems redundant, but error types are different
            Ok(serde_json::from_slice::<ValidatorSigner>(&serialized_validator_signer).expect(
                "Failed to deserialize validator signer"
            ))

        }

        pub (crate) fn get_genesis_hash(
            &self,
            working_set: &mut WorkingSet<C::Storage>,
        ) -> Result<
            HashValue, 
            Error
        > {
            
            let serialized_genesis_hash = self.genesis_hash.get(working_set).ok_or(
                anyhow::Error::msg("Serialized genesis hash is not set.")
            )?;

            // TODO: seems redundant, but error types are different
            Ok(HashValue::from_slice(serialized_genesis_hash).expect(
                "Failed to deserialize genesis hash"
            ))

        }

        pub (crate) fn get_waypoint(
            &self,
            working_set: &mut WorkingSet<C::Storage>,
        ) -> Result<
            Waypoint, 
            Error
        > {
            
            let serialized_waypoint = self.waypoint.get(working_set).ok_or(
                anyhow::Error::msg("Serialized waypoint hash is not set.")
            )?;

            println!("serialized_waypoint: {:?}", serialized_waypoint);

            // TODO: seems redundant, but error types are different
            Ok(Waypoint::from_str(serialized_waypoint.as_str()).expect(
                "Failed to deserialize waypoint"
            ))

        }

        pub (crate) fn get_known_version(
            &self,
            working_set: &mut WorkingSet<C::Storage>,
        ) -> Result<
            u64, 
            Error
        > {
            
            let known_version = self.known_version.get(working_set).ok_or(
                anyhow::Error::msg("Serialized waypoint hash is not set.")
            )?;

            Ok(known_version)

        }

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

        pub(crate) fn get_aptos_vm(
            &self,
            working_set: &mut WorkingSet<C::Storage>
        ) -> Result<
            AptosVM, 
            Error
        > {

            AptosVM::new();
            unimplemented!("Get aptos vm is not implemented!")

        }

    }

}
