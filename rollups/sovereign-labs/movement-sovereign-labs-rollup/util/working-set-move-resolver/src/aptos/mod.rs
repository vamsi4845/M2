use std::cell::RefCell;

use sov_state::WorkingSet;
// move crates
use move_core_types::{
  account_address::AccountAddress,
  language_storage::{ModuleId, StructTag},
  resolver::{ModuleResolver, ResourceResolver, MoveResolver},
  metadata::{Metadata}
};
use move_binary_format::{ file_format::CompiledModule };
use aptos_types::{ access_path::AccessPath };

use anyhow::Error;
use borsh::{BorshSerialize, BorshDeserialize};
use sov_movevm_types::identifiers::AccessPathWrapper;
use aptos_vm::move_vm_ext::MoveResolverExt;
use sov_movevm_types::aptos::state::StateValueWrapper;
use sov_movevm_types::aptos::identifiers::StateKeyWrapper;
use aptos_types::state_store::state_key::StateKey;
use aptos_types::state_store::state_value::StateValue;
use move_table_extension::TableResolver;
use aptos_framework::natives::state_storage::StateStorageUsageResolver;
use aptos_types::on_chain_config::ConfigStorage;
use aptos_state_view::TStateView;

pub struct WorkingSetAptosMoveResolverExt<'a, C: sov_modules_api::Context> {
    pub remote_cache: sov_state::StateMap<StateKeyWrapper, StateValueWrapper>,
    pub working_set: RefCell<&'a mut WorkingSet<C::Storage>>,
}

impl<'a, C: sov_modules_api::Context> WorkingSetAptosMoveResolverExt<'a, C> {
  
  pub fn new(
      remote_cache: sov_state::StateMap<StateKeyWrapper, StateValueWrapper>,
      working_set: &'a mut WorkingSet<C::Storage>,
  ) -> Self {
      Self {
          remote_cache,
          working_set: RefCell::new(working_set),
      }
  }



}

impl<'a, C: sov_modules_api::Context> ResourceResolver for WorkingSetAptosMoveResolverExt<'a, C> {
  
  fn get_resource_with_metadata(
    &self,
    address: &AccountAddress,
    struct_tag: &StructTag,
    _metadata: &[Metadata],
  ) -> Result<(Option<Vec<u8>>, usize), Error> {
    let ap = AccessPath::resource_access_path(*address, struct_tag.clone())
    .expect("Invalid access path.");
    
    let mut working_set = self.working_set.borrow_mut();

    let res = self.remote_cache.get(
      &StateKeyWrapper::new(
          StateKey::access_path(ap)
      ), &mut working_set);

    Ok((res, 0))


  }
}

impl<'a, C: sov_modules_api::Context> ModuleResolver for WorkingSetAptosMoveResolverExt<'a, C> {
  fn get_module_metadata(&self, module_id: &ModuleId) -> Vec<Metadata> {
    let module_bytes = match self.get_module(module_id) {
      Ok(Some(bytes)) => bytes,
      _ => return vec![],
    };
    let module = match CompiledModule::deserialize(&module_bytes) {
        Ok(module) => module,
        _ => return vec![],
    };
    module.metadata
  }

  fn get_module(&self, module_id: &ModuleId) -> Result<Option<Vec<u8>>, Error> {
    let ap = AccessPath::from(module_id);
    let mut working_set = self.working_set.borrow_mut();
    Ok(self.remote_cache.get(&AccessPathWrapper::new(ap), &mut working_set))
  }
}

impl<'a, C: sov_modules_api::Context> TStateView for WorkingSetAptosMoveResolverExt<'a, C> {
 
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


impl<'a, C: sov_modules_api::Context> TableResolver for WorkingSetAptosMoveResolverExt<'a, C> {
    
}

impl<'a, C: sov_modules_api::Context> StateStorageUsageResolver for WorkingSetAptosMoveResolverExt<'a, C> {
    
}

impl<'a, C: sov_modules_api::Context> ConfigStorage for WorkingSetAptosMoveResolverExt<'a, C> {
    
}

impl<'a, C: sov_modules_api::Context> MoveResolverExt for WorkingSetAptosMoveResolverExt<'a, C> {
    
}