use anyhow::{Result, Ok};
use sov_state::WorkingSet;
use crate::MoveVm;
use sov_modules_api::Error;
use move_compiler::{compiled_unit::AnnotatedCompiledUnit, Compiler};
use move_vm_types::gas::{UnmeteredGasMeter};
use move_core_types::account_address::AccountAddress;

impl<C: sov_modules_api::Context> MoveVm<C> {

    pub fn publish_stdlib(
        &self,
        working_set: &mut WorkingSet<C::Storage>,
    ) -> Result<()> {
        
        let mut src_files = move_stdlib::move_stdlib_files();
        let (_files, compiled_units) = Compiler::from_files(
            src_files,
            vec![],
            move_stdlib::move_stdlib_named_addresses(),
        )
        .build_and_report()
        .expect("Error compiling...");

        let compiled_modules = compiled_units.into_iter().map(|unit| match unit {
            AnnotatedCompiledUnit::Module(annot_module) => annot_module.named_module.module,
            AnnotatedCompiledUnit::Script(_) => panic!("Unexpected Script in stdlib"),
        });

      
        let vm = self.get_vm(working_set)?;
        let resolver = self.get_mvm_store_view(working_set);
        let mut session = vm.new_session(&resolver);
        for module in compiled_modules {
            let mut mod_blob = vec![];
            module
                .serialize(&mut mod_blob)
                .expect("Module serialization error");
            session
                .publish_module(
                    mod_blob, 
                    AccountAddress::ONE,
                    &mut UnmeteredGasMeter
                )
                .expect("Module must load");
        }

        let (change_set, events) = session.finish()?;
        self.get_change_set_publisher(working_set).publish(change_set)?;

        Ok(())
    
    }
    

    pub(crate) fn init_module(
        &self,
        config: &<Self as sov_modules_api::Module>::Config,
        working_set: &mut WorkingSet<C::Storage>,
    ) -> Result<()> {

        self.publish_stdlib(working_set)?;
        
        Ok(())
    }
}
