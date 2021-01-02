use super::netbuilder::ContractStorage;
use super::runtime_delegation::RuntimeDelegationState;
use crate::contract_data::{ContractData, ContractMethod};
use ethereum_types::U256;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
pub type RunningFunction = Box<dyn Fn() -> () + Send + Sync>;
pub trait ChainStateProvider {}

pub struct Transaction {
    dependencies: Vec<Arc<Mutex<Transaction>>>,
    pub count: Mutex<usize>,
    pub id: usize,
    runner: RunningFunction,
    pub runtime: Option<RuntimeDelegationState>,
}

/// Bridge between library and client
pub trait TransactionDataProvider {
    fn get_target_contract(&self) -> U256;
    fn get_target_method(&self) -> MethodType;
}
#[derive(Clone,Debug)]
pub enum MethodType {
    Constructor,
    Method(U256),
}

impl Transaction {
    pub fn new(id: usize, runner: RunningFunction) -> Self {
        Transaction {
            dependencies: Vec::new(),
            count: Mutex::from(0),
            id,
            runner,
            runtime: None,
        }
    }
    fn requires(&mut self, number: usize) {
        *self.count.lock().unwrap() += number;
    }
    pub fn required_by(&mut self, other: Arc<Mutex<Transaction>>) {
        let id = other.lock().unwrap().id;

        // Do not insert duplicate transictions
        for transaction in &self.dependencies {
            if transaction.lock().unwrap().id == id {
               //println!("Already a dependency");
                return;
            }
        }
       //println!("Added");
        // Increase the dependency counter of the other transaction
        other.lock().unwrap().requires(1);

        // Register the transaction in the list of dependencies
        self.dependencies.push(other);
    }
    pub fn ended(&self) -> Vec<Arc<Mutex<Transaction>>> {
        let mut rec = Vec::new();
        for transaction in &self.dependencies {
            let transaction_inner = &mut transaction.lock().unwrap();
            let mutex = transaction_inner.count.lock();
            let mut data = mutex.unwrap();
            *data -= 1;
            let mut run = false;
            if *data == 0 {
                run = true;
            }
            std::mem::drop(data);
            if run {
                rec.push(transaction.clone())
            }
        }
        rec
    }
    pub fn resolve_runtime(
        &mut self,
        _state_provider: Box<dyn ChainStateProvider>,
    ) -> Vec<Arc<Mutex<Transaction>>> {
        let mut freed = Vec::new();
        if let Some(rs) = &mut self.runtime {
            let contract = rs.contracthash;
            let method = rs.methodhash;
            let mut methods_to_prune = Vec::new();

            methods_to_prune.push((
                contract,
                rs.contract_data
                    .get(&contract)
                    .unwrap()
                    .methods
                    .get(&method)
                    .unwrap(),
            ));
            while !methods_to_prune.is_empty() {
                let (contract, method) = methods_to_prune.pop().unwrap();
                prune_method(&mut rs.contracts.get_mut(&contract).unwrap(), method);
                let mut holder = Vec::new();
                for call in &method.method_call {
                    let contract = call.0.resolve().unwrap();
                    let method = call.1.resolve().unwrap();
                    holder.push((
                        contract,
                        rs.contract_data
                            .get(&contract)
                            .unwrap()
                            .methods
                            .get(&method)
                            .unwrap(),
                    ));
                }
                methods_to_prune.extend(holder);
            }
            // Remove remaining links
            for (_, storage) in &rs.contracts {
                //Read
                for (_, list) in &storage.storage_read {
                    for transaction in list {
                        if transaction.lock().unwrap().id > self.id {
                            let tr_lock = transaction.lock().unwrap();
                            let mut count = tr_lock.count.lock().unwrap();
                            *count -= 1;
                            if *count == 0 {
                                freed.push(transaction.clone())
                            }
                            if let Some(i) = self
                                .dependencies
                                .iter()
                                .position(|t| Arc::ptr_eq(t, transaction))
                            {
                                self.dependencies.remove(i);
                            }
                        }
                    }
                }
            }
        }
        freed
    }
    pub fn run(&mut self) {
        (self.runner)()
    }
}
fn prune_method(storage: &mut ContractStorage, method: &ContractMethod) {
    for access in &method.storage_read {
        // TODO: Replace resolve
        let memory_address = access.value().resolve().unwrap();
        storage.storage_write.remove(&memory_address);
    }

    // Resolve dependencies for write access
    for access in &method.storage_write {
        // TODO: Replace resolve
        let memory_address = access.value().resolve().unwrap();
        // Add dependencies to reading transactions
        storage.storage_read.remove(&memory_address);
        // Add dependency to writing transactions
        storage.storage_read.remove(&memory_address);
    }
}
