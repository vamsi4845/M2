use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};
use aptos_types::state_store::state_key::StateKey;


#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
pub struct StateKeyWrapper(StateKey);


impl BorshSerialize for StateKeyWrapper {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&serde_json::to_vec(&self.0)?)?;        
        Ok(())
    }
  }
  
impl BorshDeserialize for StateKeyWrapper {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        Ok(Self(serde_json::from_slice(buf)?))
    }
    fn deserialize_reader<R>(_: &mut R) -> Result<Self, std::io::Error> where R: std::io::Read { todo!() }
}

impl StateKeyWrapper {
    pub fn new (state_key : StateKey) -> Self {
        Self(state_key)
    }
}

impl Into<StateKey> for StateKeyWrapper {
    fn into(self) -> StateKey {
        self.0
    }
}