
use tokio::sync::{mpsc::Sender, RwLock};
use aptos_api::accept_type::AcceptType;
use aptos_api::response::{AptosResponseContent, BasicResponse};
use aptos_api::transactions::{
    SubmitTransactionPost, SubmitTransactionResponse, SubmitTransactionsBatchPost,
    SubmitTransactionsBatchResponse,
};
use aptos_api::{
    get_api_service, 
    Context,
};
use aptos_api_types::{
    Address, EncodeSubmissionRequest, IdentifierWrapper, MoveStructTag, RawTableItemRequest,
    StateKeyWrapper, TableItemRequest, ViewRequest, U64,
};
use aptos_config::config::NodeConfig;
use aptos_crypto::HashValue;
use aptos_db::AptosDB;
use aptos_executor::block_executor::BlockExecutor;
use aptos_executor::db_bootstrapper::{generate_waypoint, maybe_bootstrap};
use aptos_executor_types::BlockExecutorTrait;
use aptos_mempool::core_mempool::{CoreMempool, TimelineState};
use aptos_mempool::{MempoolClientRequest, MempoolClientSender, SubmissionStatus};
use aptos_sdk::rest_client::aptos_api_types::MAX_RECURSIVE_TYPES_ALLOWED;
use aptos_sdk::transaction_builder::TransactionFactory;
use aptos_sdk::types::{AccountKey, LocalAccount};
use aptos_state_view::account_with_state_view::AsAccountWithStateView;
use aptos_storage_interface::state_view::DbStateViewAtVersion;
use aptos_storage_interface::DbReaderWriter;
use aptos_types::account_address::AccountAddress;
use aptos_types::account_config::aptos_test_root_address;
use aptos_types::account_view::AccountView;
use aptos_types::block_executor::partitioner::{ExecutableBlock, ExecutableTransactions};
use aptos_types::block_info::BlockInfo;
use aptos_types::block_metadata::BlockMetadata;
use aptos_types::chain_id::ChainId;
use aptos_types::ledger_info::{generate_ledger_info_with_sig, LedgerInfo};
use aptos_types::mempool_status::{MempoolStatus, MempoolStatusCode};
use aptos_types::transaction::Transaction::UserTransaction;
use aptos_types::transaction::{SignedTransaction, Transaction, WriteSetPayload};
use aptos_types::validator_signer::ValidatorSigner;
use aptos_vm::AptosVM;
use aptos_vm_genesis::{test_genesis_change_set_and_validators, GENESIS_KEYPAIR};
use types::Block;

const APTOS_DB_DIR: &str = ".aptosdb-block-executor-sidecar";

#[async_trait::async_trait]
pub trait ExecutorOperations {

    async fn init(&self, config: NodeConfig) -> Result<(), anyhow::Error>;

    async fn execute_block(&self, block: Block) -> Result<(), anyhow::Error>;

    async fn get_api_service(&self) -> Result<u32, anyhow::Error>;

}

#[derive(Debug, Clone)]
pub enum ExecutorState {
    Init(Init),
    Ready(Ready),
    Unhealthy(Unhealthy),
}

impl ExecutorState {
    pub fn new() -> Self {
        Executor::Init(Init)
    }
}

#[derive(Debug, Clone)]
pub struct Init;

impl Init {

    pub async fn init(&self, config: NodeConfig) -> Result<ExecutorState, anyhow::Error> {
    
        // initialize the est Genesis
        let (genesis, validators) = test_genesis_change_set_and_validators(Some(1));
        let signer = ValidatorSigner::new(
            validators[0].data.owner_address,
            validators[0].consensus_key.clone(),
        );
        let genesis_txn = Transaction::GenesisTransaction(WriteSetPayload::Direct(genesis));

        // configure db
        let p = format!(
            "{}/{}",
            dirs::home_dir()?,
            APTOS_DB_DIR
        );
        fs::create_dir_all(p.as_str()).unwrap();
        let (aptos_db, db_reader_writer) = DbReaderWriter::wrap(AptosDB::new_for_test(p.as_str()));
        let db = Arc::new(RwLock::new(db_reader_writer.clone()));

        // generate the waypoint
        match generate_waypoint::<AptosVM>(&db_reader_writer, &genesis_txn) {
            Ok(waypoint) => {
                maybe_bootstrap::<AptosVM>(&db_reader_writer, &genesis_txn, waypoint).unwrap();
            },
            _ => {},
        }
     
        // initialize the executor
        let block_executor = BlockExecutor::new(db_reader_writer.clone());
        let executor = Arc::new(RwLock::new(executor));


        // initialize the mempool
        let (mempool_client_sender, mut mempool_client_receiver) =
            futures_mpsc::channel::<MempoolClientRequest>(10);
        let sender = MempoolClientSender::from(mempool_client_sender);

         // set up the api context
         let context = Context::new(
            ChainId::test(),
            db_reader_writer.reader.clone(),
            sender,
            node_config.clone(),
        );

        // get the api service
        let service = get_raw_api_service(Arc::new(context.clone()));

        // spawn the mempool handler
        let mempool_handler = tokio::task::spawn(async move {
            while let Some(request) = mempool_client_receiver.next().await {
                match request {
                    MempoolClientRequest::SubmitTransaction(_t, callback) => {
                        // accept all the transaction
                        let ms = MempoolStatus::new(MempoolStatusCode::Accepted);
                        let status: SubmissionStatus = (ms, None);
                        callback.send(Ok(status)).unwrap();
                    },
                    MempoolClientRequest::GetTransactionByHash(_, _) => {},
                }
            }
        });

    }

}

#[derive(Debug, Clone)]
pub struct Ready {

    pub api_service: u32,

    pub api_context: Context,

    pub core_mempool: Arc<RwLock<CoreMempool>>,

    pub db: Option<Arc<RwLock<DbReaderWriter>>>,

    pub signer: Option<ValidatorSigner>,

    pub executor: Arc<RwLock<BlockExecutor<AptosVM>>>,

}

impl Ready {

    pub async fn execute_block(&self, block: Block) -> Result<(), anyhow::Error> {
        
        // todo: transform this into an into
        // get block and timestamp
        let ts = block.ts;
        let executable_transactions = ExecutableTransactions::Unsharded(
            block.transactions.clone()
        );
        let executable_block = ExecutableBlock::new(
            block.id.clone(),
            executable_transactions,
        );

        // execute the block
        let executor = self.executor.write().await;
        let output = executor.execute_block(executable_block)?;

        // get next epoch
        let db = self.db;
        let latest_ledger_info = get_latest_ledger_info()?;
        let next_epoch = latest_ledger_info.ledger_info().next_block_epoch();

        // commit the ledger info
        let ledger_info = LedgerInfo::new(
            BlockInfo::new(
                next_epoch,
                0,
                block.id.clone(),
                output.root_hash(),
                output.version(),
                ts,
                output.epoch_state().clone(),
            ),
            HashValue::zero(),
        );
        let li = generate_ledger_info_with_sig(
            &[self.signer.as_ref().unwrap().clone()],
            ledger_info,
        );
        executor.commit_blocks(vec![block_id], li.clone()).unwrap();

        // commit the transactions
        {
            let mut mempool = self.core_mempool.write().await;
                for transaction in block.transactions {
                    match transaction {
                        Transaction::UserTransaction(txn) => {
                            let sender = txn.sender();
                            let sequence_number = txn.sequence_number();
                            mempool.commit_transaction(&AccountAddress::from(sender), sequence_number);
                        },
                        _ => {},
                    
                }
            }

        }

    }


}

#[derive(Debug, Clone)]
pub struct Unhealthy;