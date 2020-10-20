use crate::contract_data::{ContractData, ContractMethod};
use crate::contract_utils::DataType;
use crate::evm_types::StackValue;
use ethereum_types::U256;
use std::collections::HashSet;

macro_rules! contract_data {
    ($($a:expr=>{read: $($rl:expr),*;write: $($wl:expr),*;calls: $($cl:expr),*;});*) => {{
        let mut contract = ContractData::new();
        contract.constructor = ContractMethod::new();
        $(
            let method= contract.get_method(U256::from($a));
            let mut readaccess= HashSet::<DataType>::new();
            $(
                readaccess.insert(DataType::Field(StackValue::ActualValue(U256::from($rl))));
            )*
            method.access_read(readaccess);
            let mut writeaccess= HashSet::<DataType>::new();
            $(
                writeaccess.insert(DataType::Field(StackValue::ActualValue(U256::from($wl))));
            )*
            method.access_write(writeaccess);
            let mut calls= HashSet::<(StackValue,StackValue)>::new();
            $(
                calls.insert((StackValue::SLoad(Box::from(StackValue::ActualValue(U256::from(0)))),StackValue::ActualValue(U256::from($cl))));
            )*
            method.method_calls(calls);
        )*
        contract
    }};
}

#[test]
fn full_test() {
    let contr1: ContractData = contract_data! {
         0 => {
             read: 0 ,12;
             write: 0,1,4;
             calls: ;
         };
         0x43 =>{
             read: 1,4;
             write: 12;
             calls: 435;
         }
    };
    contr1.display();
}
