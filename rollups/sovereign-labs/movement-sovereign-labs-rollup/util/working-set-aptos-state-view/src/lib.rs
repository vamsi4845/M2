use aptos_state_view::{StateView, TStateView};
use aptos_types::state_store::state_key::{StateKey};


use working_set_move_resolver::MvmStoreView;

pub struct WorkingSetAptosStateView<'a, C: sov_modules_api::Context> {
    // pub remote_cache: sov_state::StateMap<AccessPathWrapper, Vec<u8>>,
    // pub working_set: RefCell<&'a mut WorkingSet<C::Storage>>,
    pub resolver: MvmStoreView<'a, C>,
}



impl<'a, C: sov_modules_api::Context> WorkingSetAptosStateView<'a, C> {
  pub fn new(
      resolver : MvmStoreView<'a, C>
  ) -> Self {
      resolver
  }
}

impl<'a, C: sov_modules_api::Context> TStateView<StateView<Key = StateKey>> for WorkingSetAptosStateView<'a, C> {
 
    fn id(&self) -> aptos_state_view::StateViewId {
        unimplemented!("This may need to change from the Miscellaneous ID.")
    }

    // This can remain the default.
    // fn get_state_value_bytes(&self, state_key: &Self::Key) -> anyhow::Result<Option<Vec<u8>>> {
    //    unimplemented!("Not sure if we can do this directly yet.")
    //}

    fn get_state_value(&self, state_key: &Self::Key) -> anyhow::Result<Option<aptos_types::state_store::state_value::StateValue>> {
       unimplemented!("Not sure if we can do this directly yet.");
    }

    fn get_usage(&self) -> anyhow::Result<aptos_types::state_store::state_storage_usage::StateStorageUsage> {
        unimplemented!("Not sure if we can do this directly yet.")
    }

    fn is_genesis(&self) -> bool {
        unimplemented!("Not sure if we can do this directly yet.")
    }

    fn as_in_memory_state_view(&self) -> aptos_state_view::in_memory_state_view::InMemoryStateView {
        unreachable!("This is not yet support in the aptos upstream, we will not support it here.")
    }


}