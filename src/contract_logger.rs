use crate::contract_data::ContractData;
use crate::evm_function::EvmFunction;
use crate::evm_memory::EvmStack;
pub trait ContractLogger {
    fn log_instruction(&mut self, instruction: &str, pc: usize, stack: &EvmStack);
    fn log_new_function(&mut self, start: usize);
    fn finalize_function(&mut self, function: &EvmFunction);
    fn finalize(&mut self);
    fn log_contract_data(&mut self, contract_data: ContractData);
}

pub struct NoLogger();

impl ContractLogger for NoLogger {
    fn log_instruction(&mut self, _instruction: &str, _pc: usize, _stack: &EvmStack) {}
    fn log_new_function(&mut self, _start: usize) {}
    fn finalize_function(&mut self, _function: &EvmFunction) {}
    fn finalize(&mut self) {}
    fn log_contract_data(&mut self, _contract_data: ContractData) {}
}
