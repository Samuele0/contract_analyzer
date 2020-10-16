use crate::contract_data::{ContractData, ContractMethod};
use crate::contract_utils::get_pubblic_method;
use crate::evm_execution::EvmExecution;
use crate::evm_types::StackValue::*;
use std::collections::vec_deque::VecDeque;

pub fn analyze_contract(code: Vec<u8>) -> ContractData {
    let mut executions: VecDeque<EvmExecution<'_>> = VecDeque::new();
    executions.push_back(EvmExecution::new(&code[..], 0));
    let mut runtime_code: Option<Vec<u8>> = None;
    let mut contract = ContractData::new();

    while !executions.is_empty() {
        let mut exe = executions.pop_front().unwrap();
        exe.execute();
        executions.append(&mut exe.execution_list);
        // Check if the runtime code has been returned
        if let Some((x, y)) = exe.return_value {
            // If the execution has returned something
            let memvalue = exe.memory.retrive(y, x);
            if let Some(ret) = memvalue {
                // If the returned value point to a valid memory value
                if let CodeCopy(from, length) = ret {
                    // If the returned value is a non solvable CodeCopy
                    runtime_code = Some(Vec::from(
                        &code[from.resolve().unwrap().as_usize()
                            ..from.resolve().unwrap().as_usize()
                                + length.resolve().unwrap().as_usize()],
                    ));
                    // Add information about the constructor
                    let mut method = ContractMethod::new();
                    method.access_read(exe.storage_access_read);
                    method.access_write(exe.storage_access_write);
                    contract.set_constructor(method);
                } else {
                    if let CodeSection(x) = ret {
                        // If the returned value is an effective section of code
                        runtime_code = Some(Vec::from(x));
                        let mut method = ContractMethod::new();
                        // Add information about the constructor
                        method.access_read(exe.storage_access_read);
                        method.access_write(exe.storage_access_write);
                        method.method_calls(exe.external_calls);
                        contract.set_constructor(method);
                    }
                }
            }
        }
    }
    // Analyze runtime code
    if let Some(rcode) = runtime_code {
        executions.push_back(EvmExecution::new(&rcode[..], 0));
        while !executions.is_empty() {
            let mut exe = executions.pop_front().unwrap();
            exe.execute();
            if let Some(hash) = get_pubblic_method(&exe) {
                // If this execution belongs to a method
                // Retrive or create the method and append information
                let  method = contract.get_method(hash);
                method.access_read(exe.storage_access_read);
                method.access_write(exe.storage_access_write);
                method.method_calls(exe.external_calls);
            }
            executions.append(&mut exe.execution_list);
        }
    }
    // Display the contract
    contract.display();
    return contract;
}
