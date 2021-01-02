//use crate::evm_execution::EvmExecution;

use crate::evm_types::StackValue;
use crate::evm_types::StackValue::*;
use ethereum_types::U256;
///Type of data that can be present in the contract storage
#[derive(Debug, Hash, Eq, Clone, PartialEq)]
pub enum DataType {
    Field(StackValue),
    Struct(StackValue),
    Vector(StackValue),
    Mapping(StackValue, StackValue),
    Unknown(StackValue),
}
impl DataType {
    /// Retrive the StackValue representing the storage address of the data
    pub fn value(&self) -> StackValue {
        match self {
            DataType::Field(x) => x.clone(),
            DataType::Struct(x) => x.clone(),
            DataType::Vector(x) => x.clone(),
            DataType::Mapping(x, _) => x.clone(),
            DataType::Unknown(x) => x.clone(),
        }
    }
}

/// Return the root public method which this execution belongs to;
pub fn get_pubblic_method(
    guard: &StackValue,
    stack: &Vec<(&crate::evm_memory::EvmStack, &crate::evm_memory::EvmMemory)>,
) -> Option<U256> {
    if let EQ(a, b) = guard {
        // Method guards allways start with eq
        //println!("looking for calldata in {:?}", guard);
        //println!("{:?}", stack.last().unwrap().0);
        if let ActualValue(x) = **a {
            let hash = x;
            if look_for_calldata(&*b) {
                return Some(hash);
            }
        }
        if let ActualValue(x) = **b {
            let hash = x;
            if look_for_calldata(&*a) {
                return Some(hash);
            }
        }
    }
    None
}

/// looks for a CallDataLoad(0) inside a StackValue tree
pub fn look_for_calldata(val: &StackValue) -> bool {
    match val {
        CallDataLoad(x) => {
            if let ActualValue(v) = **x {
                v == U256::from(0)
            } else {
                false
            }
        }
        And(a, b) => look_for_calldata(&**a) || look_for_calldata(&**b),
        Xor(a, b) => look_for_calldata(&**a) || look_for_calldata(&**b),
        Or(a, b) => look_for_calldata(&**a) || look_for_calldata(&**b),
        Add(a, b) => look_for_calldata(&**a) || look_for_calldata(&**b),
        Sub(a, b) => look_for_calldata(&**a) || look_for_calldata(&**b),
        Mul(a, b) => look_for_calldata(&**a) || look_for_calldata(&**b),
        Div(a, b) => look_for_calldata(&**a) || look_for_calldata(&**b),
        Shr(a, b) => look_for_calldata(&**a) || look_for_calldata(&**b),
        ShL(a, b) => look_for_calldata(&**a) || look_for_calldata(&**b),
        GT(a, b) => look_for_calldata(&**a) || look_for_calldata(&**b),
        _ => false,
    }
}

/// Retrives the "top level" data structure this storage location belongs to
pub fn top_level_data(expr: &StackValue) -> DataType {
    match expr {
        ActualValue(_) => DataType::Field(expr.clone()),
        Add(a, b) => {
            let right = top_level_data(&**b);
            if let DataType::Vector(x) = top_level_data(a) {
                return DataType::Vector(x);
            }
            match right {
                DataType::Unknown(x) => DataType::Unknown(Add(a.clone(), Box::from(x))),
                DataType::Field(y) => DataType::Struct(y),
                DataType::Struct(y) => DataType::Struct(y),
                DataType::Mapping(x, y) => DataType::Mapping(x, y),
                DataType::Vector(y) => DataType::Vector(y),
            }
        }

        Sha3(v) => {
            if v.len() == 1 {
                // Array
                return DataType::Vector(v[0].1.clone());
            }
            if v.len() == 2 {
                // Mapping
                if v[0].0 > v[1].0 {
                    let resolved = top_level_data(&v[0].1);
                    DataType::Mapping(resolved.value(), expr.clone())
                } else {
                    let resolved = top_level_data(&v[1].1);

                    DataType::Mapping(resolved.value(), expr.clone())
                }
            } else {
                DataType::Unknown(expr.clone())
            }
        }

        _ => DataType::Unknown(expr.clone()),
    }
}
/*/// tries to recursivly resolve unknown jump locations
pub fn resolve_parent_calls(
    stack: &EvmStack,
    memory: &EvmMemory,
    resolve_list: Vec<(EvmStack, EvmMemory, StackValue)>,
    registry: &mut FunctionRegistry,
) -> (Vec<usize>, Vec<StackValue>, Vec<usize>) {
    let mut resolved_vector = Vec::<usize>::new();
    let mut unresolved_vector = Vec::<StackValue>::new();
    let mut unanalized_vector = Vec::<usize>::new();
    for call in &resolve_list {
        let to_resolve = call.2;
        let replaced = to_resolve.replace_parent_call(stack, memory);
        let resolved = replaced.resolve();
        if let Some(addr) = resolved {
            let f = registry.get_from_address(addr.as_usize());
            if let Some(func) = f {
                resolved_vector.push(addr.as_usize());
                let ret = resolve_parent_calls(
                    &call.0,
                    &call.1,
                    func.internal_unresolved_calls,
                    registry,
                );
                resolved_vector.extend(ret.0);
                for unresolved in ret.1 {
                    let replaced2 = unresolved.replace_parent_call(stack, memory);
                    if let Some(x) = replaced.resolve() {
                        let f2 = registry.get_from_address(x.as_usize());
                        if let Some(func2) = f2 {
                            let ret = resolve_parent_calls(
                                stack,
                                memory,
                                func2.internal_unresolved_calls,
                                registry,
                            );
                            resolved_vector.extend(ret.0);
                            unresolved_vector.extend(ret.1);
                            unanalized_vector.extend(ret.2);
                        } else {
                            unanalized_vector.push(x.as_usize())
                        }
                    } else {
                        unresolved_vector.push(replaced2);
                    }
                }
            } else {
                unanalized_vector.push(addr.as_usize());
            }
        } else {
            unresolved_vector.push(replaced);
        }
    }
    return (resolved_vector, unresolved_vector, unanalized_vector);
}
*/
