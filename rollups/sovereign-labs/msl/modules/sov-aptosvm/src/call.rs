use anyhow::Result;
use revm::primitives::CfgEnv;
use sov_modules_api::CallResponse;
use sov_state::WorkingSet;
use serde_json;
use aptos_crypto::hash::CryptoHash;

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
use aptos_types::transaction::Transaction::UserTransaction;
use aptos_types::validator_signer::ValidatorSigner;
use aptos_vm::AptosVM;
use aptos_vm_genesis::{GENESIS_KEYPAIR, test_genesis_change_set_and_validators};
use aptos_types::transaction::{Transaction};

use borsh::{BorshDeserialize, BorshSerialize};


#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub struct CallMessage {
    pub serialized_tx: Vec<u8>,
}

impl<C: sov_modules_api::Context> AptosVm<C> {
    pub(crate) fn execute_call(
        &self,
        serialized_tx: Vec<u8>,
        _context: &C,
        working_set: &mut WorkingSet<C::Storage>,
    ) -> Result<CallResponse> {

        let tx : Transaction = serde_json::from_slice(&serialized_tx)?;
        let hash = tx.hash(); // diem crypto hasher
        let str_hash = String::from_utf8(hash.to_vec())?;
        // store the transaction
        self.transactions.set(&str_hash, &serialized_tx, working_set);

        // execute the transaction in Aptos
        let executor = self.get_executor(working_set)?;
        let parent_block_id = executor.committed_block_id();
        let block_id = HashValue::random();
        let result = executor
            .execute_block((block_id, vec![tx]).into(), parent_block_id, None)?;

        // TODO: may want to use a lower level of execution abstraction
        // TODO: see https://github.com/movemntdev/aptos-core/blob/main/aptos-move/block-executor/src/executor.rs#L73
        // TODO: for an entrpoint that does not require a block.

        println!("Result {:?}", result);
        Ok(CallResponse::default())
    }
}
