use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};
use move_core_types::language_storage::{ModuleId, TypeTag};
use move_core_types::identifier::IdentStr;
use aptos_types::account_address::AccountAddress;
use crate::identifiers::{
    AccountAddressWrapper,
    IdentStrWrapper,
    ModuleIdWrapper,
    TypeTagWrapper
};

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