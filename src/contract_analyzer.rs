use crate::contract_data::{ContractData, ContractMethod};
use crate::contract_utils::get_pubblic_method;
//use crate::evm_execution::EvmExecution;
use crate::cycle_resolution::CycleSolver;
use crate::cycle_resolution::NocycleSolver;
use crate::evm_function::{EvmFunction, FunctionRegistry};
use crate::evm_memory::{EvmMemory, EvmStack};
use crate::evm_types::{StackValue, StackValue::*};
use ethereum_types::U256;

use std::collections::HashMap;
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

pub fn analyze_contract(code: &[u8]) -> Option<ContractData> {
    let mut registry = FunctionRegistry::new();
    let mut cycle_solver = NocycleSolver();
    let functions = list_functions(code);
    for f_loc in functions {
        //println!("ANALYZING FUNCTION {}", f_loc);
        let mut evm_func = EvmFunction::new(f_loc, code);
        evm_func.execute(&mut cycle_solver);
        //println!("Function calls: {:?}", evm_func.internal_calls);
        registry.analyzed.insert(f_loc, evm_func);
    }
    // Get storage access
    let start = &registry.analyzed[&0];
    let mut constructor = ContractMethod::new();
    let mut storage = HashMap::new();

    resolve_function_storage(
        start,
        &registry,
        Vec::new(),
        &mut constructor,
        false,
        &cycle_solver,
        &mut storage,
        vec![0],
    );

    //get return value
    let retv = resolve_return_node(start, &registry, Vec::new());
    //println!("{:?}", retv);
    if let Some(CodeSection(v)) = retv {
        registry = FunctionRegistry::new();
        let code = &v[..];
        let functions = list_functions(code);
        for f_loc in functions {
            //println!("ANALYZING FUNCTION {}", f_loc);
            let mut evm_func = EvmFunction::new(f_loc, code);
            evm_func.execute(&mut cycle_solver);
            //println!("Function details: {:?}", evm_func);

            registry.analyzed.insert(f_loc, evm_func);
        }
        let start = &registry.analyzed[&0];
        let mut temporary = ContractMethod::new();
        resolve_function_storage(
            start,
            &registry,
            Vec::new(),
            &mut temporary,
            false,
            &cycle_solver,
            &mut storage,
            vec![0],
        );
        return Some(ContractData::construct(constructor, storage));
    }
    None
}

pub fn resolve_function_storage(
    node: &EvmFunction,
    registry: &FunctionRegistry,
    parent_data: Vec<(&EvmStack, &EvmMemory)>,
    contract_method: &mut ContractMethod,
    top_level_found: bool,
    cycle_solver: &dyn CycleSolver,
    storage: &mut HashMap<U256, ContractMethod>,
    call_stack: Vec<usize>,
) {
    //println!("RESOLVING NODE {} FOR STORAGE ACCESS", node.position);
    //println!("TOP LEVEL METHOD FOUND?: {}", top_level_found);
    // Resolve read access
    for read_access in &node.storage_access_read {
        //println!("Resolving read access: {:?}", read_access);
        let mut resolved = read_access.clone();
        for parent in parent_data.iter().rev() {
            resolved = resolved.replace_parent_call(parent.0, parent.1);
        }
        //println!("Resolved value: {:?}", resolved);
        contract_method.push_read_location(cycle_solver.get_data(&resolved));
    }
    // Resolve write access
    for read_access in &node.storage_access_write {
        //println!("Resolving write access: {:?}", read_access);
        let mut resolved = read_access.clone();
        for parent in parent_data.iter().rev() {
            resolved = resolved.replace_parent_call(parent.0, parent.1);
        }
        //println!("Resolved value: {:?}", resolved);
        contract_method.push_write_location(cycle_solver.get_data(&resolved));
    }
    // Resolve external calls
    //println!("EXTERNALS: {:?}", node.external_calls);
    for read_access in &node.external_calls {
        //println!("Resolving external call: {:?}", read_access);
        let mut resolved_address = read_access.0.clone();
        for parent in parent_data.iter().rev() {
            resolved_address = resolved_address.replace_parent_call(parent.0, parent.1);
        }
        let mut resolved_method = read_access.1.clone();
        for parent in parent_data.iter().rev() {
            resolved_method = resolved_method.replace_parent_call(parent.0, parent.1);
        }
        let resolved = (resolved_address, resolved_method);
        //println!("Resolved value: {:?}", resolved);
        contract_method.push_external_call(resolved);
    }

    for call in &node.internal_calls {
        //println!("\t Analyzing call  to {:?}", call.0);
        let mut resolved = call.0.clone();
        for parent in parent_data.iter().rev() {
            resolved = resolved.replace_parent_call(parent.0, parent.1);
        }
        //println!("\t Resolved address: {:?}", resolved);
        let address = resolved.resolve().unwrap();
        if !cycle_solver.should_go(&call_stack, address.as_usize()) {
            continue;
        }
        let new_node = &registry.analyzed[&address.as_usize()];
        let mut new_vector = parent_data.clone();
        new_vector.push((&call.1, &call.2));
        let mut newstack = call_stack.clone();
        newstack.push(address.as_usize());
        // Check if we have found a top level method
        if !top_level_found {
            if let Some(c) = &call.3 {
                if let Some(addr) = get_pubblic_method(c) {
                    let mut method = ContractMethod::new();
                    method.access_read(contract_method.storage_read.clone());
                    method.access_write(contract_method.storage_write.clone());
                    method.method_calls(contract_method.method_call.clone());
                    resolve_function_storage(
                        new_node,
                        registry,
                        new_vector,
                        &mut method,
                        true,
                        cycle_solver,
                        storage,
                        newstack,
                    );
                    storage.insert(addr, method);
                    continue;
                }
            }
        }
        resolve_function_storage(
            new_node,
            registry,
            new_vector,
            contract_method,
            top_level_found,
            cycle_solver,
            storage,
            newstack,
        );
    }
}

pub fn resolve_return_node(
    node: &EvmFunction,
    registry: &FunctionRegistry,
    parent_data: Vec<(&EvmStack, &EvmMemory)>,
) -> Option<StackValue> {
    //println!("RESOLVING NODE FOR RETURN: {}", node.position);
    if let Some(r) = &node.return_value {
        //println!("Found Return value: {:?}", r);
        let mut replaced_start = r.1.clone();
        let mut replaced_length = r.0.clone();
        for parent in parent_data.iter().rev() {
            replaced_start = replaced_start.replace_parent_call(parent.0, parent.1);
            replaced_length = replaced_length.replace_parent_call(parent.0, parent.1);
        }

        if let Some(x) = node
            .memory
            .retrive(replaced_start.clone(), replaced_length.clone())
        {
            return Some(x);
        }
        for parent in parent_data.iter().rev() {
            let mem_loc = parent
                .1
                .retrive(replaced_start.clone(), replaced_length.clone());
            if let Some(x) = mem_loc {
                return Some(x);
            }
        }
    }
    for call in &node.internal_calls {
        //println!("\t Analyzing call  to {:?}", call.0);
        let mut resolved = call.0.clone();
        for parent in parent_data.iter().rev() {
            resolved = resolved.replace_parent_call(parent.0, parent.1);
        }
        //println!("\t Resolved address: {:?}", resolved);
        let address = resolved.resolve().unwrap();
        let new_node = &registry.analyzed[&address.as_usize()];
        let mut new_vector = parent_data.clone();
        new_vector.push((&call.1, &call.2));
        let returned = resolve_return_node(new_node, registry, new_vector);
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
