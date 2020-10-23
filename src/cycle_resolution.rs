use crate::contract_utils::{top_level_data, DataType};
use crate::evm_types::StackValue;
pub trait CycleSolver {
    fn get_data(&mut self, location: &StackValue) -> DataType;
}

pub struct NocycleSolver();

impl CycleSolver for NocycleSolver {
    fn get_data(&mut self, location: &StackValue) -> DataType {
        top_level_data(location)
    }
}
