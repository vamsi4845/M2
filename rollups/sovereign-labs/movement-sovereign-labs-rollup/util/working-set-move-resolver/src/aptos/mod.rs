use aptos_vm::{MoveResolverExt};
use crate::{MvmStoreView};

#[cfg(feature = "nono")]
impl<'a, C: sov_modules_api::Context> MoveResolverExt for MvmStoreView<'a, C> {


}
  