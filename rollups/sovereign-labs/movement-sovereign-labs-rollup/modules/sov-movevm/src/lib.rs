pub mod call;
pub mod genesis;
pub mod query;
pub mod move_resolver;
#[cfg(test)]
mod tests;
pub use movevm::{AccountData, MoveVm, MoveVmConfig};

<<<<<<< HEAD:rollups/sovereign-labs/movement-sovereign-labs-rollup/modules/sov-movevm/src/lib.rs
mod movevm {
=======
// mvmt-patch; jack
#[cfg(feature = "experimental")]
pub mod db_interface;
// mvmt-patch; end

#[cfg(feature = "experimental")]
mod experimental {
>>>>>>> jack/add-db-interface:rollups/sovereign-labs/msl/modules/sov-movevm/src/lib.rs
    use revm::primitives::{KECCAK_EMPTY, U256};
    use sov_modules_api::Error;
    use sov_modules_macros::ModuleInfo;
    use sov_state::WorkingSet;

    #[derive(Clone)]
    pub struct AccountData {
   
    }

    impl AccountData {
        pub fn empty_code() -> [u8; 32] {
            KECCAK_EMPTY.to_fixed_bytes()
        }

        pub fn balance(balance: u64) -> u64 {
            balance
        }
    }

    #[derive(Clone)]
    pub struct MoveVmConfig {
        pub data: Vec<AccountData>,
    }

    #[allow(dead_code)]
    #[derive(ModuleInfo, Clone)]
    pub struct MoveVm<C: sov_modules_api::Context> {
        #[address]
        pub(crate) address: C::Address,

    }

    impl<C: sov_modules_api::Context> sov_modules_api::Module for MoveVm<C> {
        type Context = C;

        type Config = EvmConfig;

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

}
