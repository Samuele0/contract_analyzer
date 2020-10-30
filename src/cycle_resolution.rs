use crate::contract_utils::{top_level_data, DataType};
use crate::evm_types::StackValue;
pub trait CycleSolver {
    fn get_data(&self, location: &StackValue) -> DataType;
    fn should_go(&self, stack: &Vec<usize>, address: usize) -> bool;
}

pub struct NocycleSolver();

impl CycleSolver for NocycleSolver {
    fn get_data(&self, location: &StackValue) -> DataType {
        top_level_data(location)
    }
    fn should_go(&self, stack: &Vec<usize>, address: usize) -> bool {
        !stack.contains(&address)
    }
}
