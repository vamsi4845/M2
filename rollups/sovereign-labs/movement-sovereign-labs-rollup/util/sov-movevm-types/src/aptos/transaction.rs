use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};
use aptos_types::transaction::{Transaction};


pub struct TransactionWrapper(Transaction);

impl BorshSerialize for TransactionWrapper {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&serde_json::to_vec(&self.0)?)?;        
        Ok(())
    }
}

impl Into<Transaction> for TransactionWrapper {
    fn into(self) -> Transaction {
        self.0
    }
}

impl BorshDeserialize for TransactionWrapper {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        Ok(Self(serde_json::from_slice(buf)?))
    }
    fn deserialize_reader<R>(_: &mut R) -> Result<Self, std::io::Error> where R: std::io::Read { todo!() }
}
