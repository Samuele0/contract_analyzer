use crate::contract_logger::ContractLogger;
use crate::cycle_resolution::CycleSolver;
use crate::evm_function::EvmFunction;
use crate::evm_types::{StackValue, StackValue::*};
use ethereum_types::U256;
impl<'a> EvmFunction<'a> {
    pub fn execute(&mut self, logger: &mut impl ContractLogger) {
        while !self.ended {
            let opcode = self.code[self.pc];
            match opcode {
                0x0 => self.stop(logger),
                0x1 => self.add(logger),
                0x2 => self.mul(logger),
                0x3 => self.sub(logger),
                0x4 => self.div(logger),
                0x5 => self.sdiv(logger),
                0x6 => self.mod_(logger),
                0x7 => self.smod(logger),
                0x8 => self.add_mod(logger),
                0x9 => self.mul_mod(logger),
                0xa => self.exp(logger),
                0xb => self.sign_extend(logger),
                0x10 => self.lt(logger),
                0x11 => self.gt(logger),
                0x12 => self.slt(logger),
                0x13 => self.sgt(logger),
                0x14 => self.eq(logger),
                0x15 => self.iszero(logger),
                0x16 => self.and(logger),
                0x17 => self.or(logger),
                0x18 => self.xor(logger),
                0x19 => self.not(logger),
                0x1a => self.byte(logger),
                0x1b => self.shl(logger),
                0x1c => self.shr(logger),
                0x1d => self.sar(logger),
                0x20 => self.sha3(logger),
                0x30 => self.address(logger),
                0x31 => self.balance(logger),
                0x32 => self.origin(logger),
                0x33 => self.caller(logger),
                0x34 => self.callvalue(logger),
                0x35 => self.calldata_load(logger),
                0x36 => self.calldata_size(logger),
                0x37 => self.calldata_copy(logger),
                0x38 => self.code_size(logger),
                0x39 => self.codecopy(logger),
                0x3a => self.gasprice(logger),
                0x3b => self.ext_codesize(logger),
                0x3c => self.ext_codecopy(logger),
                0x3d => self.return_data_size(logger),
                0x3e => self.return_data_copy(logger),
                0x3f => self.ext_codehash(logger),
                0x40 => self.blockhash(logger),
                0x41 => self.coinbase(logger),
                0x42 => self.timestamp(logger),
                0x43 => self.number(logger),
                0x44 => self.difficulty(logger),
                0x45 => self.gaslimit(logger),
                0x50 => self.pop(logger),
                0x51 => self.mload(logger),
                0x52 => self.mstore(logger),
                0x53 => self.mstore8(logger),
                0x54 => self.sload(logger),
                0x55 => self.sstore(logger),
                0x56 => self.jump(logger),
                0x57 => self.jumpi(logger),
                0x58 => self.pc(logger),
                0x59 => self.msize(logger),
                0x5a => self.gas(logger),
                0x5b => self.jumpdest(logger),
                0x60 => self.push(1, logger),
                0x61 => self.push(2, logger),
                0x62 => self.push(3, logger),
                0x63 => self.push(4, logger),
                0x64 => self.push(5, logger),
                0x65 => self.push(6, logger),
                0x66 => self.push(7, logger),
                0x67 => self.push(8, logger),
                0x68 => self.push(9, logger),
                0x69 => self.push(10, logger),
                0x6a => self.push(11, logger),
                0x6b => self.push(12, logger),
                0x6c => self.push(13, logger),
                0x6d => self.push(14, logger),
                0x6e => self.push(15, logger),
                0x6f => self.push(16, logger),
                0x70 => self.push(17, logger),
                0x71 => self.push(18, logger),
                0x72 => self.push(19, logger),
                0x73 => self.push(20, logger),
                0x74 => self.push(21, logger),
                0x75 => self.push(22, logger),
                0x76 => self.push(23, logger),
                0x77 => self.push(24, logger),
                0x78 => self.push(25, logger),
                0x79 => self.push(26, logger),
                0x7a => self.push(27, logger),
                0x7b => self.push(28, logger),
                0x7c => self.push(29, logger),
                0x7d => self.push(30, logger),
                0x7e => self.push(31, logger),
                0x7f => self.push(32, logger),
                0x80 => self.dup(1, logger),
                0x81 => self.dup(2, logger),
                0x82 => self.dup(3, logger),
                0x83 => self.dup(4, logger),
                0x84 => self.dup(5, logger),
                0x85 => self.dup(6, logger),
                0x86 => self.dup(7, logger),
                0x87 => self.dup(8, logger),
                0x88 => self.dup(9, logger),
                0x89 => self.dup(10, logger),
                0x8a => self.dup(11, logger),
                0x8b => self.dup(12, logger),
                0x8c => self.dup(13, logger),
                0x8d => self.dup(14, logger),
                0x8e => self.dup(15, logger),
                0x8f => self.dup(16, logger),
                0x90 => self.swap(1, logger),
                0x91 => self.swap(2, logger),
                0x92 => self.swap(3, logger),
                0x93 => self.swap(4, logger),
                0x94 => self.swap(5, logger),
                0x95 => self.swap(6, logger),
                0x96 => self.swap(7, logger),
                0x97 => self.swap(8, logger),
                0x98 => self.swap(9, logger),
                0x99 => self.swap(10, logger),
                0x9a => self.swap(11, logger),
                0x9b => self.swap(12, logger),
                0x9c => self.swap(13, logger),
                0x9d => self.swap(14, logger),
                0x9e => self.swap(15, logger),
                0x9f => self.swap(16, logger),
                0xa0 => self.log(0, logger),
                0xa1 => self.log(1, logger),
                0xa2 => self.log(2, logger),
                0xa3 => self.log(3, logger),
                0xa4 => self.log(4, logger),
                0xf0 => self.create(logger),
                0xf1 => self.call(logger),
                0xf2 => self.call(logger),
                0xf3 => self.return_(logger),
                0xf4 => self.delegate_call(logger),
                0xf5 => self.create2(logger),
                0xfa => self.static_call(logger),
                0xfd => self.revert(logger),
                0xff => self.selfdestruct(logger),
                _ => self.ended = true,
            }
            self.pc += 1;
        }
    }
    pub fn log_operation(&self, logger: &mut impl ContractLogger, name: &str) {
        //println!("{}\t\x1b[0;34m{}\t{}\x1b[0m", self.pc, name, self.stack);

        logger.log_instruction(name, self.pc, &self.stack);
    }

    /* Instructions */
    pub fn stop(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "STOP");
        self.ended = true;
    }

    pub fn add(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "ADD");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Add(Box::from(op1), Box::from(op2)));
    }
    pub fn mul(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "MUL");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Mul(Box::from(op1), Box::from(op2)));
    }
    pub fn sub(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "SUB");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Sub(Box::from(op1), Box::from(op2)));
    }
    pub fn div(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "DIV");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        /* if let Some(a) = op1.resolve() {
            if let Some(b) = op2.resolve() {
                self.stack.push(ActualValue(a / b));
                return;
            }
        }*/
        self.stack.push(Div(Box::from(op1), Box::from(op2)));
    }
    pub fn sdiv(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "SDIV");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(SDiv(Box::from(op1), Box::from(op2)));
    }
    pub fn mod_(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "MOD");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Mod(Box::from(op1), Box::from(op2)));
    }
    pub fn smod(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "SMOD");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(SMod(Box::from(op1), Box::from(op2)));
    }
    pub fn add_mod(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "ADDMOD");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        let op3 = self.stack.pop();
        self.stack
            .push(AddMod(Box::from(op1), Box::from(op2), Box::from(op3)));
    }
    pub fn mul_mod(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "MULMOD");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        let op3 = self.stack.pop();
        self.stack
            .push(MulMod(Box::from(op1), Box::from(op2), Box::from(op3)));
    }
    pub fn exp(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "EXP");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Exp(Box::from(op1), Box::from(op2)));
    }
    pub fn sign_extend(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "SIGNEXTEND");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(SignExtend(Box::from(op1), Box::from(op2)));
    }
    pub fn lt(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "LT");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(LT(Box::from(op1), Box::from(op2)));
    }
    pub fn gt(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "GT");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(GT(Box::from(op1), Box::from(op2)));
    }
    pub fn slt(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "SLT");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(SLT(Box::from(op1), Box::from(op2)));
    }
    pub fn sgt(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "SGT");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(SGT(Box::from(op1), Box::from(op2)));
    }
    pub fn eq(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "EQ");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(EQ(Box::from(op1), Box::from(op2)));
    }
    pub fn iszero(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "ISZERO");
        let op = self.stack.pop();
        self.stack.push(IsZero(Box::from(op)));
    }
    pub fn and(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "AND");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(And(Box::from(op1), Box::from(op2)));
    }
    pub fn or(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "OR");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Or(Box::from(op1), Box::from(op2)));
    }
    pub fn xor(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "XOR");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Xor(Box::from(op1), Box::from(op2)));
    }
    pub fn not(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "NOT");
        let op1 = self.stack.pop();
        self.stack.push(Not(Box::from(op1)));
    }
    pub fn byte(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "BYTE");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Byte(Box::from(op1), Box::from(op2)));
    }
    pub fn shl(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "SHL");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(ShL(Box::from(op1), Box::from(op2)));
    }
    pub fn shr(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "SHR");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Shr(Box::from(op1), Box::from(op2)));
    }
    pub fn sar(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "SAR");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Sar(Box::from(op1), Box::from(op2)));
    }
    pub fn sha3(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "SHA3");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        /*let value= self.memory.retrive_actual(op1, op2);
        let mut hasher= Keccak256::new();
        hasher.update(&value[..]);
        self.stack.push(ActualValue(U256::from(&hasher.finalize()[..])));*/
        let value = self
            .memory
            .retrive_array(op1.resolve().unwrap(), op2.resolve().unwrap());
        self.stack.push(Sha3(value));
    }
    pub fn address(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "ADDRESS");
        self.stack.push(Address);
    }
    pub fn balance(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "BALANCE");
        let address = self.stack.pop();
        self.stack.push(Balance(Box::from(address)));
    }
    pub fn origin(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "ORIGIN");
        self.stack.push(Origin);
    }
    pub fn caller(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "CALLER");
        self.stack.push(Caller);
    }
    pub fn callvalue(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "CALLVALUE");
        self.stack.push(CallValue);
    }
    pub fn calldata_load(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "CALLDATALOAD");
        let op1 = self.stack.pop();
        self.stack.push(CallDataLoad(Box::from(op1)));
    }
    pub fn calldata_size(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "CALLDATASIZE");
        self.stack.push(CallDataSize);
    }
    pub fn calldata_copy(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "CALLDATACOPY");
        let length = self.stack.pop();
        let offset = self.stack.pop();
        let dest_offset = self.stack.pop();
        let length_clone = length.clone();
        let value = CalldataCopy(Box::from(length), Box::from(offset));
        self.memory.store(dest_offset, value, length_clone);
    }
    pub fn code_size(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "CODESIZE");
        self.stack.push(CodeSize);
    }
    pub fn codecopy(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "CODECOPY");
        let dest_offset = self.stack.pop();
        let code_offset = self.stack.pop();
        let length = self.stack.pop();
        let length2 = length.clone();
        let value: StackValue;
        if let Some(a) = code_offset.resolve() {
            if let Some(b) = length.resolve() {
                if a.as_usize() + b.as_usize() <= self.code.len() {
                    value = CodeSection(Vec::from(
                        &self.code[a.as_usize()..a.as_usize() + b.as_usize()],
                    ));
                    self.memory.store(dest_offset, value, length2);
                    return;
                }
            }
        }
        value = CodeCopy(Box::from(code_offset), Box::from(length));
        self.memory.store(dest_offset, value, length2);
    }
    pub fn gasprice(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "GASPRICE");
        self.stack.push(GasPrice);
    }
    pub fn ext_codesize(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "EXTCODESIZE");
        let op1 = self.stack.pop();
        self.stack.push(ExtCodeSize(Box::from(op1)));
    }
    pub fn ext_codecopy(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "EXTCODECOPY");
        let length = self.stack.pop();
        let code_offset = self.stack.pop();
        let dest_offset = self.stack.pop();
        let addr = self.stack.pop();
        let value = ExtCodeCopy(
            Box::from(addr),
            Box::from(code_offset),
            Box::from(length.clone()),
        );
        self.memory.store(dest_offset, value, length);
    }
    pub fn return_data_size(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "RETURNDATASIZE");
        self.stack.push(ReturnDataSize);
    }
    pub fn return_data_copy(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "RETURNDATACOPY");
        let length = self.stack.pop();
        let code_offset = self.stack.pop();
        let dest_offset = self.stack.pop();
        let value = ReturnDataCopy(Box::from(code_offset), Box::from(length.clone()));
        self.memory.store(dest_offset, value, length);
    }
    pub fn ext_codehash(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "EXTCODEHASH");
        let op1 = self.stack.pop();
        self.stack.push(ExtCodeHash(Box::from(op1)));
    }
    pub fn blockhash(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "BLOCKHASH");
        let op1 = self.stack.pop();
        self.stack.push(Blockhash(Box::from(op1)));
    }
    pub fn coinbase(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "COINBASE");
        self.stack.push(CoinBase);
    }
    pub fn timestamp(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "TIMESTAMP");
        self.stack.push(TimeStamp);
    }
    pub fn number(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "NUMBER");
        self.stack.push(Number);
    }
    pub fn difficulty(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "DIFFICULTY");
        self.stack.push(Difficulty);
    }
    pub fn gaslimit(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "GASLIMIT");
        self.stack.push(GasLimit);
    }
    pub fn pop(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "POP");
        self.stack.pop();
    }
    pub fn mload(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "MLOAD");
        let offset = self.stack.pop();
        let sv = self
            .memory
            .retrive(offset.clone(), ActualValue(U256::from(32)));
        if let Some(av) = sv {
            self.stack.push(av);
        } else {
            self.stack.push(MemoryPlaceHolder(
                Box::from(offset),
                Box::from(ActualValue(U256::from(32))),
            ))
        }
    }
    pub fn mstore(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "MSTORE");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.memory.store(op1, op2, ActualValue(U256::from(32)));
    }
    pub fn mstore8(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "MSTORE8");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.memory.store(op1, op2, ActualValue(U256::from(1)));
    }
    pub fn sload(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "SLOAD");
        let op1 = self.stack.pop();
        self.stack.push(SLoad(Box::from(op1.clone())));
        self.storage_access_read.insert(op1);
    }
    pub fn sstore(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "SSTORE");
        let storage = self.stack.pop();
        self.stack.pop();
        self.storage_access_write.insert(storage);
    }
    pub fn jump(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "JUMP");
        /*let nextpc = self.stack.pop().resolve().unwrap().as_usize();
            if !self.maybecycle.contains(&nextpc) && !self.function_stack.contains(&nextpc) {
                self.pc = nextpc;
                self.function_stack.push(nextpc);
                self.jumped = true;
        }*/
        let jmp_address = self.stack.pop();
        self.ended = true;
        self.internal_calls
            .push((jmp_address, self.stack.clone(), self.memory.clone(), None))
    }
    pub fn jumpi(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "JUMPI");
        /*let address = self.stack.pop().resolve().unwrap().as_usize();
        if self.maybecycle.contains(&address) || self.function_stack.contains(&address) {
            return; // Do not follow cycles
        }
        let condition = self.stack.pop();
        let mut other = self.clone();
        other.pc = address;
        other.function_stack.push(address);
        other.jumped = true;
        other.guards.push(condition);
        self.execution_list.push_back(other);*/
        let jmp_address = self.stack.pop();
        let condition = self.stack.pop();
        self.internal_calls.push((
            jmp_address,
            self.stack.clone(),
            self.memory.clone(),
            Some(condition),
        ));
    }
    pub fn pc(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "PC");
        self.stack.push(ActualValue(U256::from(self.pc)));
    }
    pub fn msize(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "MSIZE");
        self.stack.push(MSize);
    }
    pub fn gas(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "GAS");
        self.stack.push(Gas);
    }
    pub fn jumpdest(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "JUMPDEST");
        /*if self.jumped {
            self.jumped = false;
            return;
        }
        self.maybecycle.insert(self.pc);*/
    }
    pub fn push(&mut self, length: usize, logger: &mut impl ContractLogger) {
        let value = &self.code[self.pc + 1..(self.pc + length + 1)];
        self.log_operation(logger, &format!("PUSH({:x})", U256::from(value))[..]);
        self.pc += length;
        self.stack.push(ActualValue(U256::from(value)));
    }
    pub fn dup(&mut self, n: usize, logger: &mut impl ContractLogger) {
        self.log_operation(logger, &format!("DUP {}", n)[..]);
        let value = self.stack.clone_pos(n);
        self.stack.push(value);
    }
    pub fn swap(&mut self, n: usize, logger: &mut impl ContractLogger) {
        self.log_operation(logger, &format!("SWAP {}", n)[..]);
        self.stack.swap(n);
    }
    pub fn log(&mut self, n: usize, logger: &mut impl ContractLogger) {
        self.log_operation(logger, &format!("LOG {}", n)[..]);
        self.stack.pop();
        self.stack.pop();
        for _ in 0..n {
            self.stack.pop();
        }
    }
    pub fn create(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "CREATE");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        let op3 = self.stack.pop();
        self.stack
            .push(Create(Box::from(op1), Box::from(op2), Box::from(op3)));
    }
    pub fn call(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "CALL");
        let op1 = self.stack.pop();
        let address = self.stack.pop();
        let op3 = self.stack.pop();
        let offset = self.stack.pop();
        let mem_length = self.stack.pop();
        let op6 = self.stack.pop();
        let op7 = self.stack.pop();
        let mem_value = self.memory.retrive(offset.clone(), mem_length.clone());
        //println!("INSERTING INTO CALL");
        self.external_calls
            .insert((address.clone(), mem_value.unwrap()));
        //println!("{:?}", self.external_calls);
        self.stack.push(Call(
            Box::from(op1),
            Box::from(address),
            Box::from(op3),
            Box::from(offset),
            Box::from(mem_length),
            Box::from(op6),
            Box::from(op7),
        ))
    }
    pub fn return_(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "RETURN");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.return_value = Some((op2, op1));
        self.ended = true;
    }
    pub fn delegate_call(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "DELEGATECALL");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        let op3 = self.stack.pop();
        let op4 = self.stack.pop();
        let op5 = self.stack.pop();
        let op6 = self.stack.pop();
        self.stack.push(DelegateCall(
            Box::from(op1),
            Box::from(op2),
            Box::from(op3),
            Box::from(op4),
            Box::from(op5),
            Box::from(op6),
        ))
    }
    pub fn create2(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "CREATE2");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        let op3 = self.stack.pop();
        let op4 = self.stack.pop();
        self.stack.push(Create2(
            Box::from(op1),
            Box::from(op2),
            Box::from(op3),
            Box::from(op4),
        ))
    }
    pub fn static_call(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "STATICCALL");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        let op3 = self.stack.pop();
        let op4 = self.stack.pop();
        let op5 = self.stack.pop();
        let op6 = self.stack.pop();
        self.stack.push(StaticCall(
            Box::from(op1),
            Box::from(op2),
            Box::from(op3),
            Box::from(op4),
            Box::from(op5),
            Box::from(op6),
        ))
    }
    pub fn revert(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "REVERT");
        self.ended = true;
    }
    pub fn selfdestruct(&mut self, logger: &mut impl ContractLogger) {
        self.log_operation(logger, "SELFDESTRUCT");
        self.ended = true;
    }
}
