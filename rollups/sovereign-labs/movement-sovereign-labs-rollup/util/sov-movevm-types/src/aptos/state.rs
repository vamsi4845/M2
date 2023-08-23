use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};
use aptos_types::state_store::state_value::StateValue;


#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
pub struct StateValueWrapper(StateValue);


impl BorshSerialize for StateValueWrapper {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&serde_json::to_vec(&self.0)?)?;        
        Ok(())
    }
  }
  
impl BorshDeserialize for StateValueWrapper {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        Ok(Self(serde_json::from_slice(buf)?))
    }
    fn deserialize_reader<R>(_: &mut R) -> Result<Self, std::io::Error> where R: std::io::Read { todo!() }
}

impl StateValueWrapper {
    pub fn new (state_key : StateValue) -> Self {
        Self(state_key)
    }
}

impl Into<StateValue> for StateValueWrapper {
    fn into(self) -> StateValue {
        self.0
    }
}