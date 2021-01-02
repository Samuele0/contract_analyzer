use crate::contract_logger::{ContractLogger,NoLogger};
use crate::evm_function::{EvmFunction, FunctionRegistry};
use scoped_threadpool::Pool;
use std::sync::{Arc, Mutex};
pub type FunctionAnalyzer<T: ContractLogger> =
    for<'a> fn(code: &'a [u8], functions: &[usize], logger: &mut T) -> FunctionRegistry<'a>;

pub fn single_threded_function_analyzer<'a>(
    code: &'a [u8],
    functions: &[usize],
    logger: &mut impl ContractLogger,
) -> FunctionRegistry<'a> {
    let mut registry = FunctionRegistry::new();
    for f_loc in functions {
        logger.log_new_function(*f_loc);
        let mut evm_func = EvmFunction::new(*f_loc, code);
        evm_func.execute(logger);
        logger.finalize_function(&evm_func);
        registry.analyzed.insert(*f_loc, evm_func);
        
    }
    registry
}

pub fn multi_threded_function_analyzer<'a>(
    code: &'a [u8],
    functions: &[usize],
    _logger: &mut impl ContractLogger,
) -> FunctionRegistry<'a> {
    let mut registry = FunctionRegistry::new();
    let mutexp = Arc::from(Mutex::from(&mut registry));
    let ocode: Arc<&[u8]> = Arc::from(code);
    let n_workers = 4;
    let mut pool = Pool::new(n_workers);
    pool.scoped(|scope| {
        for f_loc in functions {
            let cloned = *f_loc;
            let owned_code = ocode.clone();
            let mutex = mutexp.clone();
            scope.execute(move || {
                let code = owned_code;
                let mut evm_func = EvmFunction::new(cloned, &code[..]);
                evm_func.execute(&mut NoLogger());
                let lock = mutex.lock();
                lock.unwrap().analyzed.insert(cloned, evm_func.clone());
            });
        }
    });
    return registry;
}
