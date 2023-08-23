use anyhow::Result;
use aptos_sdk::rest_client::aptos;
use revm::primitives::CfgEnv;
use sov_modules_api::CallResponse;
use sov_state::WorkingSet;
use serde_json;
use aptos_crypto::hash::CryptoHash;
use chrono::{Utc};
use aptos_consensus_types::block::Block;

use crate::AptosVm;

use aptos_crypto::{HashValue, ValidCryptoMaterialStringExt};
use aptos_crypto::ed25519::Ed25519PublicKey;
use aptos_db::AptosDB;
use aptos_executor::block_executor::BlockExecutor;
use aptos_executor::db_bootstrapper::{generate_waypoint, maybe_bootstrap};
use aptos_executor_types::BlockExecutorTrait;
use aptos_sdk::rest_client::aptos_api_types::MAX_RECURSIVE_TYPES_ALLOWED;
use aptos_sdk::transaction_builder::TransactionFactory;
use aptos_sdk::types::{AccountKey, LocalAccount};
use aptos_state_view::account_with_state_view::AsAccountWithStateView;
use aptos_storage_interface::DbReaderWriter;
use aptos_storage_interface::state_view::DbStateViewAtVersion;
use aptos_types::account_address::AccountAddress;
use aptos_types::account_config::aptos_test_root_address;
use aptos_types::account_view::AccountView;
use aptos_types::block_info::BlockInfo;
use aptos_types::block_metadata::BlockMetadata;
use aptos_types::chain_id::ChainId;
use aptos_types::ledger_info::{generate_ledger_info_with_sig, LedgerInfo};
use aptos_types::mempool_status::{MempoolStatus, MempoolStatusCode};
use aptos_types::validator_signer::ValidatorSigner;
use aptos_vm::AptosVM;
use aptos_vm_genesis::{GENESIS_KEYPAIR, test_genesis_change_set_and_validators};
use aptos_types::transaction::{Transaction};
use aptos_types::trusted_state::{TrustedState, TrustedStateChange};

use borsh::{BorshDeserialize, BorshSerialize};
use sov_movevm_types::aptos::transaction::{TransactionWrapper};
use aptos_vm::adapter_common::VMAdapter;



#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub struct CallMessage {
    pub tx : TransactionWrapper,
}

impl<C: sov_modules_api::Context> AptosVm<C> {



    pub(crate) fn execute_call_with_naked_vm(
        &self,
        tx: TransactionWrapper,
        _context: &C,
        working_set: &mut WorkingSet<C::Storage>,
    ) -> Result<CallResponse> {

        let vm = self.get_aptos_vm(working_set)?;
        vm.execute_single_transaction(tx, _context, working_set)?;
        Ok(CallResponse::default())

    }

    pub(crate) fn execute_call(
        &self,
        tx : TransactionWrapper, 
        _context: &C,
        working_set: &mut WorkingSet<C::Storage>,
    ) -> Result<CallResponse> {

      
        self.execute_call_with_naked_vm(
            tx, 
            _context, 
            working_set
        )

    }


}
