

use super::contract_data::{ContractData, ContractMethod};
use ethereum_types::U256;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct NetBuilder {
    contracts: HashMap<U256, ContractStorage>,
    net: DependencyNet,
}
struct ContractStorage {}

pub struct DependencyNet {
    transactions: Vec<Transaction>,
}
pub struct Transaction {
    dependencies: Vec<Rc<RefCell<Transaction>>>,
    count: usize,
}
impl Transaction {
    fn new() -> Self {
        Transaction {
            dependencies: Vec::new(),
            count: 0,
        }
    }
    fn requires(&mut self, number: usize) {
        self.count += number;
    }
    fn required_by(&mut self, other: Rc<RefCell<Transaction>>) {
        self.dependencies.push(other);
    }
    fn ended(&self){
        for transaction in &self.dependencies{

        }
    }
}
