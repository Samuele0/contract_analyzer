use super::transaction::{
    ChainStateProvider, MethodType, RunningFunction, Transaction, TransactionDataProvider,
};
use crate::contract_data::{ContractData, ContractMethod};
use ethereum_types::U256;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct NetBuilder {
    contracts: HashMap<U256, ContractStorage>,
    contract_data: HashMap<U256, ContractData>,
    counter: usize,
    /// How many dependencies can we assume before delegating to runtime
    pub threshold: usize,
}
pub struct ContractStorage {
    pub storage_write: HashMap<U256, Vec<Arc<Mutex<Transaction>>>>,
    pub storage_read: HashMap<U256, Vec<Arc<Mutex<Transaction>>>>,
}
impl ContractStorage {
    fn new() -> Self {
        ContractStorage {
            storage_write: HashMap::new(),
            storage_read: HashMap::new(),
        }
    }
}

// TODO: add dependencies to constructors

impl NetBuilder {
    pub fn new() -> Self {
        NetBuilder {
            contracts: HashMap::new(),
            contract_data: HashMap::new(),
            counter: 0,
            threshold: 10,
        }
    }
    pub fn register_contract(&mut self, address: U256, contract: ContractData) {
        self.contracts.insert(address, ContractStorage::new());
        self.contract_data.insert(address, contract);
    }

    pub fn new_transaction(
        &mut self,
        transaction_data: &dyn TransactionDataProvider,
        run: RunningFunction,
    ) {
        // Retrive Transaction Informations
        let contract = transaction_data.get_target_contract();
        let method = transaction_data.get_target_method();
        let contract_data = &self.contract_data[&contract];
        let method_data = match method {
            MethodType::Method(x) => &contract_data.methods[&x],
            MethodType::Constructor => &contract_data.constructor,
        };

        // Create the transaction
        let transaction = Arc::from(Mutex::from(Transaction::new(self.counter, run)));
        self.counter += 1;
        let mut methods_to_analyze = vec![(contract, method_data)];
        let mut methods_analyzed = vec![]; // Keep a list of analyzed methods to avoid cycles
        if let MethodType::Method(x) = method {
            methods_analyzed.push((contract, x));
        }
        while !methods_to_analyze.is_empty() {
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
                let method_opt = call.1.resolve();
                if let Some(method) = method_opt {
                    if let Some(c) = contract_addr {
                        // If we can resolve the contract hash
                        let new_method = &self.contract_data[&c].methods[&method];
                        methods_to_analyze.push((c, new_method));
                    } else {
                        let mut compatible = Vec::new();
                        // Otherwise add dependency to all
                        // Look for contracts with compatible methods
                        for c in &self.contract_data {
                            for m in &c.1.methods {
                                if *m.0 == method {
                                    // If they have the same signature
                                    compatible.push((*c.0, m.1))
                                }
                            }
                        }
                        if compatible.len() < self.threshold {
                            methods_to_analyze.extend(compatible);
                        } else {
                        }
                    }
                }
            }
        }
    }

    fn analyze_method(
        method_data: &ContractMethod,
        contract: &mut ContractStorage,
        transaction: &Arc<Mutex<Transaction>>,
    ) {
        for access in &method_data.storage_read {
            // TODO: Replace resolve
            let memory_address = access.value().resolve().unwrap();
            let current = contract.storage_write.get(&memory_address);
            if let Some(list) = current {
                // If there are transactions writing to this locations
                for trans in list {
                    // Add these transactions as dependencies
                    trans.lock().unwrap().required_by(transaction.clone());
                }
            }
            let map = &mut contract.storage_read;
            // Add yourself to the reading list
            let read_location = map.entry(memory_address).or_insert_with(Vec::new);
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
                    let id1 = trans.lock().unwrap().id;
                    if id1 != transaction.lock().unwrap().id {
                        println!(
                            "Adding dependency: ({})=>({})",
                            id1,
                            transaction.lock().unwrap().id
                        );
                        // Add these transactions as dependencies
                        trans.lock().unwrap().required_by(transaction.clone());
                    }
                }
            }
            // Add dependency to writing transactions
            let current = contract.storage_read.get(&memory_address);
            if let Some(list) = current {
                // If there are transactions writing to this locations
                for trans in list {
                    let id1 = trans.lock().unwrap().id;
                    if id1 != transaction.lock().unwrap().id {
                        println!(
                            "Adding dependency: ({})=>({})",
                            id1,
                            transaction.lock().unwrap().id
                        );

                        // Add these transactions as dependencies
                        trans.lock().unwrap().required_by(transaction.clone());
                    }
                }
            }

            // Add yourself to the reading list
            let map = &mut contract.storage_read;
            let read_location = map.entry(memory_address).or_insert_with(Vec::new);
            read_location.push(transaction.clone())
        }
    }
}
