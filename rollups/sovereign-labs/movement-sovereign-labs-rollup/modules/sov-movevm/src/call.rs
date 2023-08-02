use anyhow::Result;
use move_core_types::identifier::IdentStr;
use revm::primitives::CfgEnv;
use sov_modules_api::CallResponse;
use sov_state::WorkingSet;
use sov_movevm_types::transaction::{Transaction, CallScript, CallModuleFunc, PublishModules};
use crate::MoveVm;
use move_vm_runtime::session::Session;
use move_vm_types::gas::{UnmeteredGasMeter};
use std::borrow::Borrow;
use std::rc::Rc;
use std::borrow::Cow;
use working_set_change_set_publisher::{ChangeSetPublisher};
use sov_modules_api::Error;


#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub struct CallMessage {
    pub tx: Transaction,
}

impl<C: sov_modules_api::Context> MoveVm<C> {

    pub(crate) fn call_script(
        &self,
        call_script: CallScript,
        _context: &C,
        working_set: &mut WorkingSet<C::Storage>,
    ) -> Result<CallResponse> {
        
        let vm = self.get_vm(working_set)?;
        let resolver = self.get_mvm_store_view(working_set);
        let mut session = vm.new_session(&resolver);
        session.execute_script(
            call_script.script,
            call_script.ty_args.into_iter().map(|ty_arg| ty_arg.into()).collect(),
            call_script.args,
            &mut UnmeteredGasMeter{ },
        ).expect("Failed to execute script");

        let (change_set, events) = session.finish()?;
        self.get_change_set_publisher(working_set).publish(change_set)?;

        Ok(CallResponse { })
        

    }

    pub(crate) fn call_module_func(
        &self,
        call_module_func: CallModuleFunc,
        _context: &C,
        working_set: &mut WorkingSet<C::Storage>,
    ) -> Result<CallResponse> {
        
        let vm = self.get_vm(working_set)?;
        let resolver = self.get_mvm_store_view(working_set);
        let mut session = vm.new_session(&resolver);
        session.execute_entry_function(
            &call_module_func.module_id.into(),
            &IdentStr::new(call_module_func.function_name.inner()).expect(
                "Failed to convert function name to identifier"
            ),
            call_module_func.ty_args.into_iter().map(|ty_arg| ty_arg.into()).collect(),
            call_module_func.args,
            &mut UnmeteredGasMeter{},
        ).expect("Failed to execute entry function");

        let (change_set, events) = session.finish()?;
        self.get_change_set_publisher(working_set).publish(change_set)?;

        Ok(CallResponse { })

    }

    pub(crate) fn publish_modules(
        &self,
        publish_modules : PublishModules,
        _context: &C,
        working_set: &mut WorkingSet<C::Storage>,
    ) -> Result<CallResponse> {
        
        let vm = self.get_vm(working_set)?;
        let resolver = self.get_mvm_store_view(working_set);
        let mut session = vm.new_session(&resolver);
        session.publish_module_bundle(
            publish_modules.modules.into(),
            publish_modules.account_address.into(),
            &mut UnmeteredGasMeter { },
        ).expect("Failed to publish module bundle");

        let (change_set, events) = session.finish()?;
        self.get_change_set_publisher(working_set).publish(change_set)?;

        Ok(CallResponse { })

    }

    pub(crate) fn execute_call(
        &self,
        tx: Transaction,
        _context: &C,
        working_set: &mut WorkingSet<C::Storage>,
    ) -> Result<CallResponse> {
        
        match tx {

            Transaction::CallScript(call_script)=>{

                self.call_script(call_script, _context, working_set)

            },

            Transaction::CallModuleFunc(call_module_func)=>{

                self.call_module_func(call_module_func, _context, working_set)

            },

            Transaction::PublishModules(publish_modules)=>{

                self.publish_modules(publish_modules, _context, working_set)

            }

        }
        

    }
}
