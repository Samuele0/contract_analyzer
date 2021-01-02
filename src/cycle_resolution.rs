use crate::contract_utils::{top_level_data, DataType};
use crate::evm_function::{EvmFunction, FunctionRegistry};
use crate::evm_types::StackValue;
pub trait CycleSolver {
    fn get_data(&self, location: &StackValue) -> DataType;
    fn should_go(
        &self,
        stack: &Vec<usize>,
        address: usize,
        prev_address: usize,
        registry: &FunctionRegistry,
        condition: &Option<StackValue>,
    ) -> bool;
}

pub struct NocycleSolver();
pub struct MaxIterations(usize);

impl CycleSolver for MaxIterations {
    fn get_data(&self, location: &StackValue) -> DataType {
        top_level_data(location)
    }
    fn should_go(
        &self,
        stack: &Vec<usize>,
        address: usize,
        _prev_address: usize,
        _registry: &FunctionRegistry,
        _condition: &Option<StackValue>,
    ) -> bool {
        /*let jumps = &registry.get_from_address(address).unwrap().internal_calls;
        let mut dynamic = false;
        for (jl, _, _, _) in jumps {
            match jl {
                StackValue::ActualValue(_) => {}
                _ => dynamic = true,
            }
        }
        if dynamic {
            !stack.iter().filter(|a| (**a) == address).count() > 20
        } else {
            !stack.contains(&address)
        }*/
        //println!("STACK {:?}",stack);
        let filtered: Vec<&usize> = stack.iter().filter(|a| (**a) == address).collect();
        //println!("FILTERED {:?}",filtered);
        //println!("JUMP {}",(filtered.len() > 50));
        !(filtered.len() > self.0)
        // !stack.contains(&address)
    }
}

impl CycleSolver for NocycleSolver {
    fn get_data(&self, location: &StackValue) -> DataType {
        top_level_data(location)
    }
    fn should_go(
        &self,
        stack: &Vec<usize>,
        address: usize,
        prev_address: usize,
        _registry: &FunctionRegistry,
        condition: &Option<StackValue>,
    ) -> bool {
        /*if stack.last().unwrap() == &address {
            false
        } else {
            let filtered: Vec<&usize> = stack.iter().filter(|a| (**a) == address).collect();
            !(filtered.len() > 20)
        }*/
        //println!("checking for cycle: {}=>{} in stack {:?}",prev_address, address, stack);
        //if let Some(c) = condition {
        
        let mut begun_cycle = false;
        let mut check = false;
        let mut counter = 0usize;
        let mut bi = 0usize;
        let mut buffers = vec![];
        // We are in a jumpi
        for jump in stack {
            if !begun_cycle {
                if *jump == address {
                    begun_cycle = true;
                    buffers.push(vec![]);
                }
            } else {
                if *jump == address {
                    bi += 1;
                    buffers.push(vec![]);
                } else {
                    buffers[bi].push(*jump)
                }
            }
        }
        buffers.pop();
        //println!("possible cycles: {:?}", buffers);

        let cycle = buffers
            .iter()
            .filter(|buffer| {
                let mut equal = true;
                for (exp, actual) in buffer.iter().rev().zip(stack.iter().rev()) {
                    equal = equal && exp == actual;
                }
                equal
            })
            .count()
            > 0;
        //}
        if cycle {
            return false;
        }
        let filtered: Vec<&usize> = stack.iter().filter(|a| (**a) == address).collect();
        //println!("stack_len: {}",stack.len());
        //return !(filtered.len() > 12);
        return !stack.contains(&address);
    }
}
