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
    // let tmpdir = tempfile::tempdir().unwrap();
    // Get the home directory
    let home_path = home_dir().expect("Could not determine the home directory.");

    // Append the desired directory name
    let tmpdir = home_path.join(".sov-movevm-test");

    println!("tmpdir: {:?}", tmpdir.as_path());
    let mut working_set = WorkingSet::new(ProverStorage::with_path(tmpdir.as_path()).unwrap());

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
    

    /*movevm.remote_cache.get(
        &AccessPathWrapper::new(
            AccessPath::code_access_path(ModuleId::new(
                AccountAddress::ONE,
                Identifier::from_str("error").unwrap()
            ))
        ), working_set
    ).expect("Should be able to get value at path.");*/

    let (ordered_rws, _) = working_set.checkpoint().freeze();
    for rw in ordered_rws.ordered_writes.iter() {
        println!("rw: {:?}", rw);
    }

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
            ty_args: vec![TypeTagWrapper::new(TypeTag::U64)], 
            args: vec!["30".as_bytes().to_vec()]
        })
    };

    movevm.call(call_message, &sender_context, working_set)?;

    Ok(())

}