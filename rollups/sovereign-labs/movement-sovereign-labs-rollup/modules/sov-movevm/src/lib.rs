pub mod call;
pub mod genesis;
pub mod query;
#[cfg(test)]
mod tests;
pub use movevm::{AccountData, MoveVm, MoveVmConfig};

mod movevm {
    use revm::primitives::{KECCAK_EMPTY, U256};
    use sov_modules_api::Error;
    use sov_modules_macros::ModuleInfo;
    use sov_state::WorkingSet;
    use working_set_move_resolver::{MvmStoreView};
    use working_set_change_set_publisher::{ChangeSetPublisher};
    use move_core_types::{
        resolver::{ModuleResolver, ResourceResolver, MoveResolver},
    };
    use move_vm_runtime::session::{Session};
    use move_vm_runtime::move_vm::{MoveVM};
    use sov_movevm_types::identifiers::AccessPathWrapper;
    use move_vm_types::gas::{UnmeteredGasMeter};
    use move_stdlib::natives::GasParameters;
    use move_core_types::account_address::AccountAddress;

    #[derive(Clone)]
    pub struct AccountData {
   
    }

    impl AccountData {
 
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

        #[state]
        pub(crate) remote_cache : sov_state::StateMap<AccessPathWrapper, Vec<u8>>

    }

    impl<C: sov_modules_api::Context> sov_modules_api::Module for MoveVm<C> {
        type Context = C;

        type Config = MoveVmConfig;

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

    impl<C: sov_modules_api::Context> MoveVm<C> {

        pub(crate) fn get_mvm_store_view<'a>(
            &self,
            working_set: &'a mut WorkingSet<C::Storage>,
        ) -> MvmStoreView<'a, C>{

           MvmStoreView::new(self.remote_cache.clone(), working_set)

        }

        pub(crate) fn get_change_set_publisher<'a>(
            &self,
            working_set: &'a mut WorkingSet<C::Storage>,
        ) -> ChangeSetPublisher<'a, C>{

           ChangeSetPublisher::new(self.remote_cache.clone(), working_set)

        }

        pub(crate) fn get_vm(
            &self,
            working_set: &mut WorkingSet<C::Storage>, // TODO: not needed for now, may want to change this soon however to include some kind of running count
        ) -> Result<MoveVM, Error> {

           let natives = move_stdlib::natives::all_natives(
                AccountAddress::ONE,
                GasParameters::zeros()
           );

           Ok(MoveVM::new(natives).expect("Unable to create MoveVM"))

        }

    }



}
