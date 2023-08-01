use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};
use move_core_types::language_storage::{ModuleId, TypeTag};
use move_core_types::identifier::IdentStr;
use aptos_types::account_address::AccountAddress;

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub struct IdentStrWrapper(String);

impl IdentStrWrapper {

    pub fn inner (&self) -> &str {
        &self.0
    }
    
}


#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(Debug, PartialEq, Clone)]
pub struct ModuleIdWrapper(ModuleId);

impl BorshSerialize for ModuleIdWrapper {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&serde_json::to_vec(&self.0)?)?;        
        Ok(())
    }
  }
  
impl BorshDeserialize for ModuleIdWrapper {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        Ok(Self(serde_json::from_slice(buf)?))
    }
    fn deserialize_reader<R>(_: &mut R) -> Result<Self, std::io::Error> where R: std::io::Read { todo!() }
}

impl Into<ModuleId> for ModuleIdWrapper {
    fn into(self) -> ModuleId {
        self.0
    }
}

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(Debug, PartialEq, Clone)]
pub struct TypeTagWrapper(TypeTag);

impl BorshSerialize for TypeTagWrapper {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&serde_json::to_vec(&self.0)?)?;        
        Ok(())
    }
  }
  
impl BorshDeserialize for TypeTagWrapper {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        Ok(Self(serde_json::from_slice(buf)?))
    }
    fn deserialize_reader<R>(_: &mut R) -> Result<Self, std::io::Error> where R: std::io::Read { todo!() }
}

impl Into<TypeTag> for TypeTagWrapper {
    fn into(self) -> TypeTag {
        self.0
    }
}

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(Debug, PartialEq, Clone)]
pub struct AccountAddressWrapper(AccountAddress);

impl BorshSerialize for AccountAddressWrapper {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&serde_json::to_vec(&self.0)?)?;        
        Ok(())
    }
}

impl Into<AccountAddress> for AccountAddressWrapper {
    fn into(self) -> AccountAddress {
        self.0
    }
}

impl BorshDeserialize for AccountAddressWrapper {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        Ok(Self(serde_json::from_slice(buf)?))
    }
    fn deserialize_reader<R>(_: &mut R) -> Result<Self, std::io::Error> where R: std::io::Read { todo!() }
}

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub struct CallScript {
    pub script : Vec<u8>,
    pub ty_args: Vec<TypeTagWrapper>,
    pub args: Vec<Vec<u8>>,
}


#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub struct CallModuleFunc {
    pub module_id: ModuleIdWrapper,
    pub function_name: IdentStrWrapper,
    pub ty_args: Vec<TypeTagWrapper>,
    pub args: Vec<Vec<u8>>,
}

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub struct PublishModules {
    pub modules: Vec<Vec<u8>>,
    pub account_address: AccountAddressWrapper
}

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub enum Transaction {

    CallScript(CallScript),

    CallModuleFunc(CallModuleFunc),

    PublishModules(PublishModules)

}