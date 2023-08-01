use anyhow::Result;
use sov_state::WorkingSet;
use crate::MoveVm;

impl<C: sov_modules_api::Context> MoveVm<C> {
    pub(crate) fn init_module(
        &self,
        config: &<Self as sov_modules_api::Module>::Config,
        working_set: &mut WorkingSet<C::Storage>,
    ) -> Result<()> {
        
        Ok(())
    }
}
