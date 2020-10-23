use crate::contract_data::{ContractData, ContractMethod};
//use crate::contract_utils::get_pubblic_method;
//use crate::evm_execution::EvmExecution;
use crate::cycle_resolution::NocycleSolver;
use crate::evm_function::{EvmFunction, FunctionRegistry};
use crate::evm_instructions;
use crate::evm_memory::{EvmMemory, EvmStack};
use crate::evm_types::{StackValue, StackValue::*};
use std::cell::RefCell;
use std::collections::vec_deque::VecDeque;
/*pub fn analyze_contract(code: &[u8]) -> ContractData {
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
                } else if let CodeSection(x) = ret {
                    // If the returned value is an effective section of code
                    runtime_code = Some(x);
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
    // Analyze runtime code
    if let Some(rcode) = runtime_code {
        executions.push_back(EvmExecution::new(&rcode[..], 0));
        while !executions.is_empty() {
            let mut exe = executions.pop_front().unwrap();
            exe.execute();
            if let Some(hash) = get_pubblic_method(&exe) {
                // If this execution belongs to a method
                // Retrive or create the method and append information
                let method = contract.get_method(hash);
                method.access_read(exe.storage_access_read);
                method.access_write(exe.storage_access_write);
                method.method_calls(exe.external_calls);
            }
            executions.append(&mut exe.execution_list);
        }
    }
    // Display the contract
    contract.display();
    contract
}*/

pub fn analyze_contract(code: &[u8]) -> ContractData {
    let mut data = ContractData::new();

    let mut registry = FunctionRegistry::new();
    let mut cycle_solver = NocycleSolver();
    let functions = list_functions(code);
    for f_loc in functions {
        println!("ANALYZING FUNCTION {}", f_loc);
        let mut evm_func = EvmFunction::new(f_loc, code);
        evm_func.execute(&mut cycle_solver);
        println!("Function calls: {:?}", evm_func.internal_calls);
        registry.analyzed.insert(f_loc, evm_func);
    }
    //get return value
    let start = &registry.analyzed[&0];
    let retv = resolve_node(start, &registry, Vec::new());
    println!("{:?}", retv);
    data
}

pub fn resolve_node(
    node: &EvmFunction,
    registry: &FunctionRegistry,
    parent_data: Vec<(&EvmStack, &EvmMemory)>,
) -> Option<StackValue> {
    for call in &node.internal_calls {
        let mut resolved = call.0.clone();
        for parent in parent_data.iter().rev() {
            resolved = resolved.replace_parent_call(parent.0, parent.1);
        }
        let address = resolved.resolve().unwrap();
        let new_node = &registry.analyzed[&address.as_usize()];
        let mut new_vector = parent_data.clone();
        new_vector.push((&call.1, &call.2));
        let returned = resolve_node(new_node, registry, new_vector);
        if let Some(x) = returned {
            return Some(x);
        }
    }
    None
}

/// List all function locations (defined by `JUMPDEST` and the starting position) inside the bytecode
pub fn list_functions(code: &[u8]) -> Vec<usize> {
    let mut list = vec![0];
    let mut pc = 0;
    while pc < code.len() {
        match code[pc] {
            0x5b => {
                list.push(pc);
                pc += 1;
            }
            0x60 => pc += 1 + 1,
            0x61 => pc += 2 + 1,
            0x62 => pc += 3 + 1,
            0x63 => pc += 4 + 1,
            0x64 => pc += 5 + 1,
            0x65 => pc += 6 + 1,
            0x66 => pc += 7 + 1,
            0x67 => pc += 8 + 1,
            0x68 => pc += 9 + 1,
            0x69 => pc += 10 + 1,
            0x6a => pc += 11 + 1,
            0x6b => pc += 12 + 1,
            0x6c => pc += 13 + 1,
            0x6d => pc += 14 + 1,
            0x6e => pc += 15 + 1,
            0x6f => pc += 16 + 1,
            0x70 => pc += 17 + 1,
            0x71 => pc += 18 + 1,
            0x72 => pc += 19 + 1,
            0x73 => pc += 20 + 1,
            0x74 => pc += 21 + 1,
            0x75 => pc += 22 + 1,
            0x76 => pc += 23 + 1,
            0x77 => pc += 24 + 1,
            0x78 => pc += 25 + 1,
            0x79 => pc += 26 + 1,
            0x7a => pc += 27 + 1,
            0x7b => pc += 28 + 1,
            0x7c => pc += 29 + 1,
            0x7d => pc += 30 + 1,
            0x7e => pc += 31 + 1,
            0x7f => pc += 32 + 1,
            _ => pc += 1,
        }
    }

    list
}
