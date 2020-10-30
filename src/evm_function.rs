use crate::evm_memory::{EvmMemory, EvmStack};
use crate::evm_types::StackValue;
use std::collections::{HashMap, HashSet};

/// A Self contained block of instructions in the evm bytecode
///
/// We can immagine the bytecode as a set of disjointed functions, each one starting with `JUMPDEST` and terminating with either `RETURN`, `REVERT`, `STOP`, or an unconditional jump.
///
/// Each one of these functions presents itself as a 'black box', meaning that, while they do not provide a detailed insight of their inner workings, they will provide the following informations:
///  1. Storage access locations, expressed as a StackValue.
///
///  ### Exemple
///
#[derive(Debug)]
pub struct EvmFunction<'a> {
    /// Where this function starts; should be a `JUMPDEST` or the beginning of the code
    pub position: usize,
    /// Reference to the complete bytecode
    pub code: &'a [u8],

    /// The partial stack associated to this function
    pub stack: EvmStack,

    /// The partial memory state associated to this function
    pub memory: EvmMemory,

    /// Weather this Function has ended or not
    pub ended: bool,

    /// The program counter keeping trak of the current position in the code
    pub pc: usize,

    /// The memory area returned by this function
    pub return_value: Option<(StackValue, StackValue)>,

    /// The list of other internal (belonging to this contract) functions that might be called during the execution of this one
    ///
    /// The functions are referenced by their starting position, since we might not yet be able to resolve their actual location the positions are stored as StackValues.
    ///
    /// The stack and memory state at the moment of invocation are also recorded, since they might me accessed by the called function.
    ///
    /// The values memorized in the tuple are the following:
    /// 1. Jump address
    /// 2. Evm Stack at the moment of the call
    /// 3. Evm Memory at the moment of the call
    /// 4. Condition required to make the jump (None for unconditional jumps)
    pub internal_calls: Vec<(StackValue, EvmStack, EvmMemory, Option<StackValue>)>,

    /// The list of internal storage locations accessed by this function for reading
    pub storage_access_read: HashSet<StackValue>,

    /// The list of internal storage locations accessed by this function for writing
    pub storage_access_write: HashSet<StackValue>,

    /// The list of external (belonging to other contracts) functions invoked by this one
    pub external_calls: HashSet<(StackValue, StackValue)>,
}

impl<'a> EvmFunction<'a> {
    pub fn new(position: usize, code: &'a [u8]) -> Self {
        EvmFunction {
            position,
            code,
            stack: EvmStack::new(),
            memory: EvmMemory::new(),
            ended: false,
            pc: position,
            return_value: None,
            internal_calls: Vec::new(),
            external_calls: HashSet::new(),
            storage_access_read: HashSet::new(),
            storage_access_write: HashSet::new(),
        }
    }
}
pub struct FunctionRegistry<'a> {
    pub analyzed: HashMap<usize, EvmFunction<'a>>,
    pub in_analysis: Vec<usize>,
}

impl<'a> FunctionRegistry<'a> {
    pub fn new() -> Self {
        FunctionRegistry {
            analyzed: HashMap::new(),
            in_analysis: Vec::new(),
        }
    }

    pub fn get_from_address(&self, addr: usize) -> Option<&EvmFunction<'a>> {
        self.analyzed.get(&addr)
    }
}
