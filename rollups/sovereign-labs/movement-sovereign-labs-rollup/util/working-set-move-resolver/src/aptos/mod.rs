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

    match self.remote_cache.get(&StateKeyWrapper::new(
      StateKey::access_path(ap)
    ), &mut working_set) {
      Some(val) => Ok((Some(serde_json::to_vec(&val)?), 0)),
      None => Ok((None, 0))
    }

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
    match self.remote_cache.get(&StateKeyWrapper::new(
      StateKey::access_path(ap)
    ), &mut working_set) {
      Some(val) => Ok(Some(serde_json::to_vec(&val)?)),
      None => Ok(None)
    }
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

  fn resolve_table_entry(
          &self,
          handle: &move_table_extension::TableHandle,
          key: &[u8],
      ) -> Result<Option<Vec<u8>>, anyhow::Error> {
          let mut working_set = self.working_set.borrow_mut();
          let account_address = handle.0;
          let ap = AccessPath::new(account_address, key.to_vec());
          let state_key_wrapper = StateKeyWrapper::new(
            StateKey::access_path(ap)
          );
          let state_value_wrapper = self.remote_cache.get(&state_key_wrapper, &mut working_set);
          match state_value_wrapper {
              Some(state_value_wrapper) => {
                  let state_value : StateValue = state_value_wrapper.into();
                  Ok(Some(state_value.into_bytes()))
              },
              None => Ok(None)
          }
  }
    
}

impl<'a, C: sov_modules_api::Context> StateStorageUsageResolver for WorkingSetAptosMoveResolverExt<'a, C> {
    

  fn get_state_storage_usage(&self) -> anyhow::Result<aptos_types::state_store::state_storage_usage::StateStorageUsage> {
      
    Ok(aptos_types::state_store::state_storage_usage::StateStorageUsage::Untracked)

  }

}

impl<'a, C: sov_modules_api::Context> ConfigStorage for WorkingSetAptosMoveResolverExt<'a, C> {

  fn fetch_config(&self, access_path: AccessPath) -> Option<Vec<u8>> {
      None
  }
    
}

impl<'a, C: sov_modules_api::Context> MoveResolverExt for WorkingSetAptosMoveResolverExt<'a, C> {

  fn get_resource_group_data(
          &self,
          address: &AccountAddress,
          struct_tag: &StructTag,
      ) -> move_binary_format::errors::VMResult<Option<Vec<u8>>> {
      unimplemented!("get_resource_group_data is not yet supported.")
  }

  fn get_standard_resource(
          &self,
          address: &AccountAddress,
          struct_tag: &StructTag,
      ) -> move_binary_format::errors::VMResult<Option<Vec<u8>>> {
        unimplemented!("get_standard_resource is not yet supported.")
  }

  fn is_resource_group(&self, struct_tag: &StructTag) -> bool {
      false
  }

  fn release_resource_group_cache(
          &self,
      ) -> std::collections::BTreeMap<AccountAddress, std::collections::BTreeMap<StructTag, std::collections::BTreeMap<StructTag, Vec<u8>>>> {
      unimplemented!("release_resource_group_cache is not yet supported.")
  }
    
}