use revm::primitives::{KECCAK_EMPTY, U256};
use sov_modules_api::default_context::DefaultContext;
use sov_modules_api::default_signature::private_key::DefaultPrivateKey;
use sov_modules_api::{Context, Module, PublicKey, Spec};
use sov_state::{ProverStorage, WorkingSet};
type C = DefaultContext;