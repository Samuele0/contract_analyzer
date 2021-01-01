use super::netbuilder::NetBuilder;
use super::transaction::RunningFunction;
use super::transaction::{MethodType, TransactionDataProvider};
use crate::contract_data::{ContractData, ContractMethod};
use crate::contract_utils::DataType;
use crate::evm_types::StackValue;
use ethereum_types::U256;
use std::collections::HashSet;
struct MockTransaction {
    target: U256,
    method: MethodType,
}
impl TransactionDataProvider for MockTransaction {
    fn get_target_contract(&self) -> U256 {
        self.target
    }
    fn get_target_method(&self) -> MethodType {
        self.method.clone()
    }
}

macro_rules! contract_data {
    ($($a:expr=>{read: $($rl:expr),*;write: $($wl:expr),*;calls: $($cl:expr),*;});*) => {{
        let mut contract = ContractData::new();
        contract.constructor = ContractMethod::new();
        $(
            let method:&mut ContractMethod;
            if $a==0{
                method= &mut contract.constructor;
            }
            else{
                method= contract.get_method(U256::from($a));
            }
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
macro_rules! u56 {
    ($a:expr) => {
        U256::from($a)
    };
}
macro_rules! transaction {
    ($a:expr,$b:expr) => {{
        let method: MethodType;
        if $b == 0 {
            method = MethodType::Constructor;
        } else {
            method = MethodType::Method(U256::from($b));
        }
        MockTransaction {
            target: U256::from($a),
            method,
        }
    }};
}

#[test]
fn full_test() {
    let mut builder = NetBuilder::new();
    let rf = || {};

    builder.register_contract(
        u56!(10),
        contract_data! {
             0 => {
                 read: 0 ,1;
                 write: 0,1;
                 calls: ;
             };
             0x43 =>{
                 read: 0;
                 write: 0;
                 calls: ;
             };
             0x345 =>{
                 read: 1;
                 write: 1;
                 calls: ;
             };
             0x2347 =>{
                read: 0,1;
                write: 0,1;
                calls: ;
             }

        },
    );
    builder.new_transaction(&transaction!(10, 0), Box::from(rf.clone()));
    builder.new_transaction(&transaction!(10, 0x43), Box::from(rf.clone()));
    builder.new_transaction(&transaction!(10, 0x2347), Box::from(rf.clone()));
    builder.new_transaction(&transaction!(10, 0x345), Box::from(rf.clone()));
    builder.register_contract(
        u56!(15),
        contract_data! {
             0 => {
                 read: 2 ,1,4;
                 write: 2,1, 3;
                 calls: ;
             };
             0x57 =>{
                 read: 0;
                 write: 0,1;
                 calls: ;
             };
             0x96 =>{
                 read: 1,0;
                 write: 1;
                 calls: 0x2347;
             };
             0x2147 =>{
                read: 0,1;
                write: 0,1;
                calls: ;
             }

        },
    );
    builder.new_transaction(&transaction!(15, 0), Box::from(rf.clone()));
    builder.new_transaction(&transaction!(15, 0x57), Box::from(rf.clone()));
    builder.new_transaction(&transaction!(10, 0x345), Box::from(rf.clone()));
    builder.new_transaction(&transaction!(15, 0x96), Box::from(rf.clone()));
}
