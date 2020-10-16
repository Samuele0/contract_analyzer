use crate::evm_execution::EvmExecution;
use crate::evm_types::StackValue;
use crate::evm_types::StackValue::*;
use ethereum_types::U256;

///Type of data that can be present in the contract storage
#[derive(Debug, Hash, Eq, Clone, PartialEq)]
pub enum DataType {
    Field(StackValue),
    Struct(StackValue),
    Vector(StackValue),
    Mapping(StackValue),
    Unknown(StackValue),
}
impl DataType {
    /// Retrive the StackValue representing the storage address of the data
    pub fn value(&self) -> StackValue {
        match self {
            DataType::Field(x) => x.clone(),
            DataType::Struct(x) => x.clone(),
            DataType::Vector(x) => x.clone(),
            DataType::Mapping(x) => x.clone(),
            DataType::Unknown(x) => x.clone(),
        }
    }
}

/// Return the root public method which this execution belongs to;
pub fn get_pubblic_method(execution: &EvmExecution) -> Option<U256> {
    for guard in &execution.guards {
        if let EQ(a, b) = guard {
            // Method guards allways start with eq
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
            let ret = match right {
                DataType::Unknown(x) => DataType::Unknown(Add(a.clone(), Box::from(x))),
                DataType::Field(y) => DataType::Struct(y),
                DataType::Struct(y) => DataType::Struct(y),
                DataType::Mapping(y) => DataType::Mapping(y),
                DataType::Vector(y) => DataType::Vector(y),
            };
            return ret;
        }

        Sha3(v) => {
            if v.len() == 1 {
                // Array
                return DataType::Vector(v[0].1.clone());
            }
            if v.len() == 2 {
                // Mapping
                if v[0].0 > v[1].0 {
                    return DataType::Mapping(expr.clone());
                } else {
                    return DataType::Mapping(expr.clone());
                }
            } else {
                return DataType::Unknown(expr.clone());
            }
        }

        _ => return DataType::Unknown(expr.clone()),
    }
}
