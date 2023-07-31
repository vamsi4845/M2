// pub mod access_path; // TODO: implement custom access path

use std::cell::RefCell;

use sov_state::WorkingSet;
// move crates
use move_core_types::{
  account_address::AccountAddress,
  language_storage::{ModuleId, StructTag},
  resolver::{ModuleResolver, ResourceResolver},
  metadata::{Metadata}
};
use move_binary_format::{ file_format::CompiledModule };
use aptos_types::{ access_path::AccessPath };

use anyhow::Error;
use borsh::{BorshSerialize, BorshDeserialize};

/// Wrapper for AccessPath
/// AccessPath doesn't derive BorshSerialize and BorshDeserialize. 
/// It drives serde::Serialize and serde::Deserialize
/// StateMap requires that its Key should derive borsh
pub struct AccessPathWrapper(AccessPath);

impl BorshSerialize for AccessPathWrapper {
  fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
      writer.write_all(&serde_json::to_vec(&self.0)?)?;        
      Ok(())
  }
}

impl BorshDeserialize for AccessPathWrapper {
  fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
    Ok(Self(serde_json::from_slice(buf)?))
  }
  fn deserialize_reader<R>(_: &mut R) -> Result<Self, std::io::Error> where R: std::io::Read { todo!() }
}


/// This is an implmenetantion of remote cache which will be back of TransactionDataCache
/// review? I used RefCell but not sure if it can be secure.
pub(crate) struct MvmStoreView<'a, C: sov_modules_api::Context> {
    pub(crate) remote_cache: sov_state::StateMap<AccessPathWrapper, Vec<u8>>,
    pub(crate) working_set: RefCell<&'a mut WorkingSet<C::Storage>>,
}

impl<'a, C: sov_modules_api::Context> MvmStoreView<'a, C> {
  pub(crate) fn new(
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
      Ok((self.remote_cache.get(&AccessPathWrapper(path), &mut working_set), 0))
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
    Ok(self.remote_cache.get(&AccessPathWrapper(ap), &mut working_set))
  }
}