pub mod netbuilder;
pub mod transaction;
#[cfg(test)]
mod tests {
    use super::netbuilder::NetBuilder;
    use super::transaction::{MethodType, TransactionDataProvider};
    use crate::contract_data::{ContractData, ContractMethod};
    use crate::contract_utils::DataType;
    use crate::evm_types::StackValue;
    use ethereum_types::U256;
    use std::collections::{HashMap, HashSet};
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

    #[test]
    fn test_simple_net_building() {
        let mut builder = NetBuilder::new();
        let c1 = contract1();
        builder.register_contract(U256::from(120), c1);
        let transaction = MockTransaction {
            target: U256::from(120),
            method: MethodType::Method(U256::from(34)),
        };
        builder.new_transaction(&transaction);
        let transaction2 = MockTransaction {
            target: U256::from(120),
            method: MethodType::Method(U256::from(34)),
        };
        builder.new_transaction(&transaction2);
    }

    fn contract1() -> ContractData {
        let constructor = ContractMethod::new();
        let mut method1 = ContractMethod::new();
        let mut read = HashSet::<DataType>::new();
        read.insert(DataType::Field(StackValue::ActualValue(U256::from(0))));
        method1.access_read(read.clone());
        method1.access_write(read);
        let mut methods = HashMap::<U256, ContractMethod>::new();
        methods.insert(U256::from(34), method1);
        ContractData {
            constructor,
            methods,
        }
    }
}
