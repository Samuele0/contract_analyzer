use super::transaction::{MethodType, Transaction, TransactionDataProvider};
use crate::contract_data::{ContractData, ContractMethod};
use ethereum_types::U256;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
pub struct NetBuilder {
    contracts: HashMap<U256, ContractStorage>,
    contract_data: HashMap<U256, ContractData>,
    counter: usize,
}
struct ContractStorage {
    storage_write: HashMap<U256, Vec<Arc<RefCell<Transaction>>>>,
    storage_read: HashMap<U256, Vec<Arc<RefCell<Transaction>>>>,
}
impl ContractStorage {
    fn new() -> Self {
        ContractStorage {
            storage_write: HashMap::new(),
            storage_read: HashMap::new(),
        }
    }
}

impl NetBuilder {
    pub fn new() -> Self {
        NetBuilder {
            contracts: HashMap::new(),
            contract_data: HashMap::new(),
            counter: 0,
        }
    }
    pub fn register_contract(&mut self, address: U256, contract: ContractData) {
        self.contracts.insert(address, ContractStorage::new());
        self.contract_data.insert(address, contract);
    }

    pub fn new_transaction(&mut self, transaction_data: &dyn TransactionDataProvider) {
        // Retrive Transaction Informations
        let contract = transaction_data.get_target_contract();
        let method = transaction_data.get_target_method();
        let contract_data = &self.contract_data[&contract];
        let method_data = match method {
            MethodType::Method(x) => &contract_data.methods[&x],
            MethodType::Constructor => &contract_data.constructor,
        };

        // Create the transaction
        let transaction = Arc::from(RefCell::from(Transaction::new(self.counter)));
        self.counter += 1;
        let mut methods_to_analyze = vec![(contract, method_data)];
        while methods_to_analyze.len() > 0 {
            let method_data = methods_to_analyze.pop().unwrap();

            // Resolve dependencies for method access
            Self::analyze_method(
                method_data.1,
                self.contracts.get_mut(&method_data.0).unwrap(),
                &transaction,
            );
            
            // Resolve external Calls
            for call in &method_data.1.method_call {
                let contract_addr = call.0.resolve();
                let method = call.1.resolve().unwrap();
                if let Some(c) = contract_addr {
                    // If we can resolve the contract hash
                    let new_method = &self.contract_data[&c].methods[&method];
                    methods_to_analyze.push((c, new_method));
                } else {
                    // Otherwise add dependency to all
                    // Look for contracts with compatible methods
                    for c in &self.contract_data {
                        for m in &c.1.methods {
                            if *m.0 == method {
                                // If they have the same signature
                                methods_to_analyze.push((*c.0, m.1))
                            }
                        }
                    }
                }
            }
        }
    }

    fn analyze_method(
        method_data: &ContractMethod,
        contract: &mut ContractStorage,
        transaction: &Arc<RefCell<Transaction>>,
    ) {
        for access in &method_data.storage_read {
            // TODO: Replace resolve
            let memory_address = access.value().resolve().unwrap();
            let current = contract.storage_write.get(&memory_address);
            if let Some(list) = current {
                // If there are transactions writing to this locations
                for trans in list {
                    // Add these transactions as dependencies
                    trans.borrow_mut().required_by(transaction.clone());
                }
            }
            let map = &mut contract.storage_read;
            // Add yourself to the reading list
            let read_location = map.entry(memory_address).or_insert(Vec::new());
            read_location.push(transaction.clone())
        }

        // Resolve dependencies for write access
        for access in &method_data.storage_write {
            // TODO: Replace resolve
            let memory_address = access.value().resolve().unwrap();
            // Add dependencies to reading transactions
            let current = contract.storage_write.get(&memory_address);
            if let Some(list) = current {
                // If there are transactions writing to this locations
                for trans in list {
                    // Add these transactions as dependencies
                    trans.borrow_mut().required_by(transaction.clone());
                }
            }
            // Add dependency to writing transactions
            let current = contract.storage_read.get(&memory_address);
            if let Some(list) = current {
                // If there are transactions writing to this locations
                for trans in list {
                    // Add these transactions as dependencies
                    trans.borrow_mut().required_by(transaction.clone());
                }
            }

            // Add yourself to the reading list
            let map = &mut contract.storage_read;
            let read_location = map.entry(memory_address).or_insert(Vec::new());
            read_location.push(transaction.clone())
        }
    }
}
