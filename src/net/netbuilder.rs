use super::runtime_delegation::RuntimeDelegationState;
use super::transaction::{MethodType, RunningFunction, Transaction, TransactionDataProvider};
use crate::contract_data::{ContractData, ContractMethod};
use ethereum_types::U256;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
pub struct NetBuilder {
    contracts: HashMap<U256, ContractStorage>,
    contract_data: HashMap<U256, ContractData>,
    runtime_dependent: Vec<(U256, U256, Arc<Mutex<Transaction>>)>,
    zero_deps: Vec<Arc<Mutex<Transaction>>>,
    counter: usize,
    /// How many dependencies can we assume before delegating to runtime
    pub threshold: usize,
}
#[derive(Clone)]
pub struct ContractStorage {
    pub contructor_transition: Option<Arc<Mutex<Transaction>>>,
    pub storage_write: HashMap<U256, Vec<Arc<Mutex<Transaction>>>>,
    pub storage_read: HashMap<U256, Vec<Arc<Mutex<Transaction>>>>,
}
impl ContractStorage {
    fn new() -> Self {
        ContractStorage {
            storage_write: HashMap::new(),
            storage_read: HashMap::new(),
            contructor_transition: None,
        }
    }
}

// TODO: add dependencies to constructors

impl NetBuilder {
    pub fn new() -> Self {
        NetBuilder {
            contracts: HashMap::new(),
            contract_data: HashMap::new(),
            runtime_dependent: Vec::new(),
            zero_deps: Vec::new(),
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
        /*println!(
            "Analyzing transaction {} :: {:?}",
            transaction_data.get_target_contract(),
            transaction_data.get_target_method()
        );*/
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
        for (_, _, trans) in &mut self.runtime_dependent {
            trans.lock().unwrap().required_by(transaction.clone());
        }
        self.counter += 1;

        let mut methods_to_analyze = vec![(contract, method_data)];
        let mut methods_analyzed = vec![]; // Keep a list of analyzed methods to avoid cycles
        let mut constructor_analyzed = vec![]; // Keep a list of analyzed contracts to avoid cycles on contructors
        if let MethodType::Method(x) = method {
            methods_analyzed.push((contract, x));
        } else {
            self.contracts
                .get_mut(&contract)
                .unwrap()
                .contructor_transition = Some(transaction.clone());
            constructor_analyzed.push(contract);
        }
        while !methods_to_analyze.is_empty() {
            let method_data = methods_to_analyze.pop().unwrap();
           //println!("Analyzing method {}", method_data.0);
            let contract_d = self.contracts.get_mut(&method_data.0).unwrap();
            if !constructor_analyzed.contains(&method_data.0) {
               //println!("Adding constructor dependency");
                contract_d
                    .contructor_transition
                    .as_ref()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .required_by(transaction.clone());
            }
            // Resolve dependencies for method access
            Self::analyze_method(method_data.1, contract_d, &transaction);

            // Resolve external Calls
            for call in &method_data.1.method_call {
                let contract_addr = call.0.resolve();
                let method_opt = call.1.resolve();
                if let Some(method_2) = method_opt {
                    if let Some(c) = contract_addr {
                        // If we can resolve the contract hash
                        let new_method = &self.contract_data[&c].methods[&method_2];
                        methods_to_analyze.push((c, new_method));
                    } else {
                        let mut compatible = Vec::new();
                        // Otherwise add dependency to all
                        // Look for contracts with compatible methods
                        for c in &self.contract_data {
                            for m in &c.1.methods {
                                if *m.0 == method_2 {
                                    // If they have the same signature
                                    compatible.push((*c.0, m.1))
                                }
                            }
                        }
                        if compatible.len() < self.threshold {
                            methods_to_analyze.extend(compatible);
                        } else {
                            self.runtime_dependent.push((
                                contract,
                                match method {
                                    MethodType::Method(x) => x,
                                    _ => U256::from(0),
                                },
                                transaction.clone(),
                            ));
                        }
                    }
                } else {
                    self.runtime_dependent.push((
                        contract,
                        match method {
                            MethodType::Method(x) => x,
                            _ => U256::from(0),
                        },
                        transaction.clone(),
                    ));
                }
            }
        }
        if *transaction.lock().unwrap().count.lock().unwrap() == 0 {
            self.zero_deps.push(transaction.clone())
        }
    }

    fn analyze_method(
        method_data: &ContractMethod,
        contract: &mut ContractStorage,
        transaction: &Arc<Mutex<Transaction>>,
    ) {
       //println!("Analyzing method read accesses");
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
       //println!("Analyzing method write accesses");
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
                        /* println!(
                            "Adding dependency: ({})=>({})",
                            id1,
                            transaction.lock().unwrap().id
                        ); */
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
                        /* println!(
                            "Adding dependency: ({})=>({})",
                            id1,
                            transaction.lock().unwrap().id
                        ); */

                        // Add these transactions as dependencies
                        trans.lock().unwrap().required_by(transaction.clone());
                    }
                }
            }

            // Add yourself to the reading list
            let map = &mut contract.storage_write;
            let read_location = map.entry(memory_address).or_insert_with(Vec::new);
            read_location.push(transaction.clone())
        }
    }
    pub fn finalize(mut self) -> Vec<Arc<Mutex<Transaction>>> {
        for (c, m, trans) in &mut self.runtime_dependent {
            trans.lock().unwrap().runtime = Some(RuntimeDelegationState {
                contract_data: self.contract_data.clone(),
                contracts: self.contracts.clone(),
                contracthash: *c,
                methodhash: *m,
            });
        }
        self.zero_deps
    }
}
