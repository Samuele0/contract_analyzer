use super::netbuilder::ContractStorage;
use crate::contract_data::ContractData;
use ethereum_types::U256;
use std::collections::HashMap;

pub struct RuntimeDelegationState {
    pub contract_data: HashMap<U256, ContractData>,
    pub contracts: HashMap<U256, ContractStorage>,
    pub contracthash: U256,
    pub methodhash: U256,
}
