use crate::contract_utils::DataType;
use crate::evm_types::StackValue;
use ethereum_types::U256;
use std::collections::{HashMap, HashSet};

pub struct ContractData {
    pub constructor: ContractMethod,
    pub methods: HashMap<U256, ContractMethod>,
}
pub struct ContractMethod {
    pub storage_read: HashSet<DataType>,
    pub storage_write: HashSet<DataType>,
    pub method_call: HashSet<(StackValue, StackValue)>,
}

impl ContractData {
    pub fn new() -> Self {
        ContractData {
            constructor: ContractMethod::new(),
            methods: HashMap::new(),
        }
    }
    ///
    /// Returns or creates the method with corrisponding hash
    ///
    pub fn get_method(&mut self, hash: U256) -> &mut ContractMethod {
        if self.methods.contains_key(&hash) {
            return self.methods.get_mut(&hash).unwrap();
        }
        self.methods.insert(hash, ContractMethod::new());
        return self.methods.get_mut(&hash).unwrap();
    }

    pub fn set_constructor(&mut self, method: ContractMethod) {
        self.constructor = method;
    }
    ///
    /// Writes Information about the contract on console
    ///
    pub fn display(&self) {
        println!("\x1b[0;31m[===CONTRACT DATA===]\n");
        println!("\x1b[0;33m[CONSTRUCTOR]\n\t\x1b[0;32mREAD:\x1b[0m{:?}\n\t\x1b[0;32mWRITE:\x1b[0m{:?}\n\n\t\x1b[0;32mCALLS:\x1b[0m{:?}\n",self.constructor.storage_read,self.constructor.storage_write,self.constructor.method_call);
        for method in &self.methods {
            println!("\x1b[0;33mFUNCTION {:x}\n\t\x1b[0;32mREAD:\x1b[0m{:?}\n\t\x1b[0;32mWRITE:\x1b[0m{:?}\n\n\t\x1b[0;32mCALLS:\x1b[0m{:?}\n",method.0,method.1.storage_read,method.1.storage_write,method.1.method_call);
        }
    }
}

impl ContractMethod {
    pub fn new() -> Self {
        ContractMethod {
            storage_read: HashSet::new(),
            storage_write: HashSet::new(),
            method_call: HashSet::new(),
        }
    }
    ///
    /// Adds storage locations that will be written during the execution of the method
    ///
    pub fn access_write(&mut self, access: HashSet<DataType>) {
        self.storage_write.extend(access);
    }
    ///
    /// Adds storage locations that will be read during the execution of the method
    ///
    pub fn access_read(&mut self, access: HashSet<DataType>) {
        self.storage_read.extend(access);
    }
    ///
    /// Adds external method calls that will be executed in this method
    ///
    pub fn method_calls(&mut self, access: HashSet<(StackValue, StackValue)>) {
        self.method_call.extend(access);
    }
}
