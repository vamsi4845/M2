use aptos_state_view::{StateView, TStateView};
use aptos_types::state_store::state_key::{StateKey};
use sov_movevm_types::aptos::identifiers::StateKeyWrapper;
use sov_movevm_types::aptos::state::StateValueWrapper;
use sov_state::WorkingSet;
use std::cell::RefCell;

pub struct WorkingSetAptosStateView<'a, C: sov_modules_api::Context> {
    pub remote_cache: sov_state::StateMap<StateKeyWrapper, StateValueWrapper>,
    pub working_set: RefCell<&'a mut WorkingSet<C::Storage>>,
    // pub resolver: MvmStoreView<'a, C>,
}



impl<'a, C: sov_modules_api::Context> WorkingSetAptosStateView<'a, C> {
  pub fn new(
    remote_cache: sov_state::StateMap<StateKeyWrapper, StateValueWrapper>,
    working_set: RefCell<&'a mut WorkingSet<C::Storage>>,
  ) -> Self {
      Self {
        remote_cache,
        working_set
      }
  }
}

impl<'a, C: sov_modules_api::Context> TStateView for WorkingSetAptosStateView<'a, C> {
 
    type Key = StateKey;

    //
    // fn id(&self) -> aptos_state_view::StateViewId {
       //  unimplemented!("This may need to change from the Miscellaneous ID.")
    // }

    // This can remain the default.
    // fn get_state_value_bytes(&self, state_key: &Self::Key) -> anyhow::Result<Option<Vec<u8>>> {
    //    unimplemented!("Not sure if we can do this directly yet.")
    //}

    fn get_state_value(&self, state_key: &Self::Key) -> anyhow::Result<Option<aptos_types::state_store::state_value::StateValue>> {
        let mut working_set = self.working_set.borrow_mut();
        let state_key_wrapper = StateKeyWrapper::new(state_key.clone());
        let state_value_wrapper = self.remote_cache.get(&state_key_wrapper, &mut working_set);
        match state_value_wrapper {
            Some(state_value_wrapper) => {
                let state_value = state_value_wrapper.into();
                Ok(Some(state_value))
            },
            None => Ok(None)
        }
    }

    fn get_usage(&self) -> anyhow::Result<aptos_types::state_store::state_storage_usage::StateStorageUsage> {
        Ok(aptos_types::state_store::state_storage_usage::StateStorageUsage::Untracked)
    }

    fn is_genesis(&self) -> bool {
       false // for now this can just be false
    }

    fn as_in_memory_state_view(&self) -> aptos_state_view::in_memory_state_view::InMemoryStateView {
        unreachable!("This is not yet support in the aptos upstream, we will not support it here.")
    }


}