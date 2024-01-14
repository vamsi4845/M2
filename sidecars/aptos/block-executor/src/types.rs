use aptos_types::transaction::Transaction;

#[derive(Debug, Clone)]
pub struct Block {
    ts : u64,
    id : String, 
    transactions : Vec<Transaction>,

}