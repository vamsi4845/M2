use sov_modules_api::default_context::DefaultContext;
use sov_modules_api::default_signature::private_key::DefaultPrivateKey;
use sov_modules_api::{Context, Module, PublicKey, Spec};
use sov_state::{ProverStorage, WorkingSet};
use crate::call::CallMessage;
use crate::{MoveVm, MoveVmConfig};
use sov_modules_api::Error;
type C = DefaultContext;
use sov_movevm_types::transaction::{Transaction, CallModuleFunc,};
use sov_movevm_types::identifiers::{ModuleIdWrapper, TypeTagWrapper, IdentStrWrapper, AccessPathWrapper};
use move_core_types::language_storage::{ModuleId, TypeTag};
use move_core_types::identifier::{IdentStr, Identifier};
use move_core_types::account_address::AccountAddress;
use std::str::FromStr;
use aptos_types::access_path::AccessPath;
use dirs::home_dir;
use std::fs;

#[test]
fn initialize_movevm_test()->Result<(), Error>{

    // working set
    let tmpdir = tempfile::tempdir().unwrap();

    println!("tmpdir: {:?}", tmpdir.path());
    let mut working_set = WorkingSet::new(ProverStorage::with_path(tmpdir.path()).unwrap());

    // context
    let priv_key = DefaultPrivateKey::generate();
    let sender = priv_key.pub_key();
    let sender_addr = sender.to_address::<<C as Spec>::Address>();
    let sender_context = C::new(sender_addr);

    // movevm 
    let movevm = MoveVm::<C>::default();
    movevm.init_module(&MoveVmConfig {
        data : vec![]
    }, &mut working_set)?;
    

    let path = AccessPath::code_access_path(ModuleId::new(
        AccountAddress::ONE,
        Identifier::from_str("error").unwrap()
    ));
    println!("Getting path: {:?}", path);
    movevm.remote_cache.get(
        &AccessPathWrapper::new(path), &mut working_set
    ).expect("Should be able to get value at path.");

    Ok(())

}

#[test]
fn movevm_test()->Result<(), Error>{

    // working set
    let tmpdir = tempfile::tempdir().unwrap();
    let working_set = &mut WorkingSet::new(ProverStorage::with_path(tmpdir.path()).unwrap());

    // context
    let priv_key = DefaultPrivateKey::generate();
    let sender = priv_key.pub_key();
    let sender_addr = sender.to_address::<<C as Spec>::Address>();
    let sender_context = C::new(sender_addr);

    // movevm 
    let movevm = MoveVm::<C>::default();
    movevm.init_module(&MoveVmConfig {
        data : vec![]
    }, working_set)?;

    // call native func
    let call_message = CallMessage {
        tx : Transaction::CallModuleFunc(CallModuleFunc { 
            module_id: ModuleIdWrapper::new(ModuleId::new(
                AccountAddress::ONE,
                Identifier::from_str("error").unwrap()
            )), 
            function_name: IdentStrWrapper::new(&IdentStr::new("aborted").unwrap()), 
            ty_args: vec![], 
            args: vec![
                30_u64.to_le_bytes().to_vec()
            ]
        })
    };

    movevm.call(call_message, &sender_context, working_set)?;

    Ok(())

}