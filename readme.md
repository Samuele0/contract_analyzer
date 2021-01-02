# Contract Analyzer

This library provides the tools to concurrently execute Ethereum transactions. In particular this library provides three functionalities:

-   Static Analysis of the contracts bytecode.
-   Building a dependency net between transactions.
-   Executing the said dependency net.

## Usage

The library provides two traits that need to be implemented:

1. `TransactionDataProvider`: implement this trait by either adding an `impl` block to the existing transactions or by creating a new wrapper struct.
1. `RunningFunction`: this type is an alias for a function that takes no parameter, this function will automatically be called to execute a transaction. It is recommended to implement it using closures.

Afterward you can use the provided struct `NetBuilder` to create the dependency net.

When a new contract is found its bytecode should be analyzed using the provided `analyze_contract_default` function.

Finally the function `execute_net` can be used to run the built dependency net.

## Example

```Rust
use contract_analyzer::*; // Import everything for simplicity
use crate::...::Transaction; // Import the internal data structure
use ethereum_types::U256; // Import the numerical representation library

impl TransactionDataProvider for Transaction{
    fn get_target_contract(&self) -> U256{
        ...
    }
    fn get_target_method(&self) -> MethodType{
        ...
    }
}

pub fn analyze_block(transactions: &[Transaction]){
    let net_builder = NetBuilder::new();

    for transaction in transactions{
        if let ContractCreation(bytes) = transaction{
            net_builder.register_contract(
                transaction.address,
                analyze_contract_default(&bytes),
            );
        }
        net_builder.new_transaction(&transaction);
    }

    execute_net(net_builder.finalize(),transactions.len());
}
```
