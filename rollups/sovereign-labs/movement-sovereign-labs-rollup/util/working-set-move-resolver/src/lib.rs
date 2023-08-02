// pub mod access_path; // TODO: implement custom access path

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

/// This is an implmenetantion of remote cache which will be back of TransactionDataCache
/// review? I used RefCell but not sure if it can be secure.
pub struct MvmStoreView<'a, C: sov_modules_api::Context> {
    pub remote_cache: sov_state::StateMap<AccessPathWrapper, Vec<u8>>,
    pub working_set: RefCell<&'a mut WorkingSet<C::Storage>>,
}

impl<'a, C: sov_modules_api::Context> MvmStoreView<'a, C> {
  pub fn new(
      remote_cache: sov_state::StateMap<AccessPathWrapper, Vec<u8>>,
      working_set: &'a mut WorkingSet<C::Storage>,
  ) -> Self {
      Self {
          remote_cache,
          working_set: RefCell::new(working_set),
      }
  }
}

impl<'a, C: sov_modules_api::Context> ResourceResolver for MvmStoreView<'a, C> {
  fn get_resource_with_metadata(
    &self,
    address: &AccountAddress,
    struct_tag: &StructTag,
    _metadata: &[Metadata],
  ) -> Result<(Option<Vec<u8>>, usize), Error> {
    let ap = AccessPath::resource_access_path(*address, struct_tag.clone()).ok();
    if let Some(path) = ap {
      let mut working_set = self.working_set.borrow_mut();
      Ok((self.remote_cache.get(&AccessPathWrapper::new(path), &mut working_set), 0))
    } else {
      Ok((None, 0))
    }
  }
}

impl<'a, C: sov_modules_api::Context> ModuleResolver for MvmStoreView<'a, C> {
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

/*impl<'a, C: sov_modules_api::Context> MoveResolver for MvmStoreView<'a, C> {

}*/