use ethereum_types::U256;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

pub struct Transaction {
    dependencies: Vec<Arc<RefCell<Transaction>>>,
    count: Mutex<usize>,
    id: usize,
}

/// Bridge between library and client
pub trait TransactionDataProvider {
    fn get_target_contract(&self) -> U256;
    fn get_target_method(&self) -> MethodType;
}
pub enum MethodType {
    Constructor,
    Method(U256),
}

impl Transaction {
    pub fn new(id: usize) -> Self {
        Transaction {
            dependencies: Vec::new(),
            count: Mutex::from(0),
            id,
        }
    }
    fn requires(&mut self, number: usize) {
        *self.count.lock().unwrap() += number;
    }
    pub fn required_by(&mut self, other: Arc<RefCell<Transaction>>) {
        let id = other.borrow().id;

        // Do not insert duplicate transictions
        for transaction in &self.dependencies {
            if transaction.borrow().id == id {
                return;
            }
        }
        // Increase the dependency counter of the other transaction
        other.borrow_mut().requires(1);

        // Register the transaction in the list of dependencies
        self.dependencies.push(other);
    }
    pub fn ended(&self) {
        for transaction in &self.dependencies {
            let transaction = &transaction.borrow_mut();
            let mutex= transaction.count.lock();
            let mut data = mutex.unwrap();
            *data -= 1;
            if *data == 0 {
                // Run transaction;
            }
        }
    }
}
