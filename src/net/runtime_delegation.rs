use super::netbuilder::ContractStorage;
use crate::contract_data::ContractData;
use crate::evm_types::StackValue;
use ethereum_types::U256;
use std::collections::HashMap;
use std::sync::Arc;

pub struct RuntimeDelegationState {
    pub contract_data: HashMap<U256, ContractData>,
    pub contracts: HashMap<U256, ContractStorage>,
    pub contracthash: StackValue,
    pub methodhash: StackValue,
}
