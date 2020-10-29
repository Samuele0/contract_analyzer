use crate::evm_memory::{EvmMemory, EvmStack};
use ethereum_types::U256;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StackValue {
    ActualValue(U256),
    Calldata,
    Unknown,
    Add(Box<StackValue>, Box<StackValue>),
    Mul(Box<StackValue>, Box<StackValue>),
    Sub(Box<StackValue>, Box<StackValue>),
    Div(Box<StackValue>, Box<StackValue>),
    SDiv(Box<StackValue>, Box<StackValue>),
    Mod(Box<StackValue>, Box<StackValue>),
    SMod(Box<StackValue>, Box<StackValue>),
    AddMod(Box<StackValue>, Box<StackValue>, Box<StackValue>),
    MulMod(Box<StackValue>, Box<StackValue>, Box<StackValue>),
    Exp(Box<StackValue>, Box<StackValue>),
    SignExtend(Box<StackValue>, Box<StackValue>),
    LT(Box<StackValue>, Box<StackValue>),
    GT(Box<StackValue>, Box<StackValue>),
    SLT(Box<StackValue>, Box<StackValue>),
    SGT(Box<StackValue>, Box<StackValue>),
    EQ(Box<StackValue>, Box<StackValue>),
    IsZero(Box<StackValue>),
    And(Box<StackValue>, Box<StackValue>),
    Or(Box<StackValue>, Box<StackValue>),
    Xor(Box<StackValue>, Box<StackValue>),
    Not(Box<StackValue>),
    Byte(Box<StackValue>, Box<StackValue>),
    ShL(Box<StackValue>, Box<StackValue>),
    Shr(Box<StackValue>, Box<StackValue>),
    Sar(Box<StackValue>, Box<StackValue>),
    Sha3(Vec<(usize, StackValue)>),
    Address,
    Balance(Box<StackValue>),
    Origin,
    Caller,
    CallValue,
    CallDataLoad(Box<StackValue>),
    CallDataSize,
    CalldataCopy(Box<StackValue>, Box<StackValue>),
    CodeSize,
    CodeCopy(Box<StackValue>, Box<StackValue>),
    GasPrice,
    ExtCodeSize(Box<StackValue>),
    ExtCodeCopy(Box<StackValue>, Box<StackValue>, Box<StackValue>),
    ReturnDataSize,
    ReturnDataCopy(Box<StackValue>, Box<StackValue>),
    ExtCodeHash(Box<StackValue>),
    Blockhash(Box<StackValue>),
    CoinBase,
    TimeStamp,
    Number,
    Difficulty,
    GasLimit,
    SLoad(Box<StackValue>),
    PC,
    MSize,
    Gas,
    Create(Box<StackValue>, Box<StackValue>, Box<StackValue>),
    Call(
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
    ),
    CallCode(
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
    ),
    DelegateCall(
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
    ),
    Create2(
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
    ),
    StaticCall(
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
        Box<StackValue>,
    ),
    CodeSection(Vec<u8>),

    /// Placeholder for stack value caming from function callee
    StackPaceHolder(usize),
    MemoryPlaceHolder(Box<StackValue>, Box<StackValue>),
}

impl StackValue {
    pub fn resolve(&self) -> Option<U256> {
        match self {
            StackValue::ActualValue(x) => Some(*x),
            StackValue::Add(a, b) => {
                if let Some(x) = a.resolve() {
                    if let Some(y) = b.resolve() {
                        return Some(x + y);
                    }
                }
                None
            }
            StackValue::CodeSection(x) => Some(U256::from(&x[..])),
            StackValue::ShL(a, b) => {
                if let Some(x) = b.resolve() {
                    if let Some(y) = a.resolve() {
                        return Some(x << y);
                    }
                }
                None
            }
            StackValue::Shr(a, b) => {
                if let Some(x) = b.resolve() {
                    if let Some(y) = a.resolve() {
                        return Some(x >> y);
                    }
                }
                None
            }
            StackValue::And(a, b) => {
                if let Some(x) = b.resolve() {
                    if let Some(y) = a.resolve() {
                        return Some(x & y);
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Replace PlaceHolders with actual values
    pub fn replace_parent_call(
        &self,
        extended_stack: &EvmStack,
        extended_memory: &EvmMemory,
    ) -> StackValue {
        match self {
            StackValue::StackPaceHolder(a) => extended_stack.clone_pos(*a),
            StackValue::MemoryPlaceHolder(a, b) => {
                if let Some(v) = extended_memory.retrive(*a.clone(), *b.clone()) {
                    v
                } else {
                    self.clone()
                }
            }
            StackValue::Add(a, b) => StackValue::Add(
                Box::from(a.replace_parent_call(extended_stack, extended_memory)),
                Box::from(b.replace_parent_call(extended_stack, extended_memory)),
            ),
            StackValue::Mul(a, b) => StackValue::Mul(
                Box::from(a.replace_parent_call(extended_stack, extended_memory)),
                Box::from(b.replace_parent_call(extended_stack, extended_memory)),
            ),
            StackValue::Sha3(v) => {
                let mut v2 = Vec::new();
                for item in v {
                    v2.push((
                        item.0,
                        item.1.replace_parent_call(extended_stack, extended_memory),
                    ));
                }
                StackValue::Sha3(v2)
            }
            _ => self.clone(),
        }
    }
}
