use crate::cycle_resolution::CycleSolver;
use crate::evm_function::EvmFunction;
use crate::evm_types::{StackValue, StackValue::*};
use ethereum_types::U256;
use std::collections::HashSet;
impl<'a> EvmFunction<'a> {
    pub fn execute(&mut self, cycle_solver: &mut dyn CycleSolver) {
        while !self.ended {
            let opcode = self.code[self.pc];
            match opcode {
                0x0 => self.stop(),
                0x1 => self.add(),
                0x2 => self.mul(),
                0x3 => self.sub(),
                0x4 => self.div(),
                0x5 => self.sdiv(),
                0x6 => self.mod_(),
                0x7 => self.smod(),
                0x8 => self.add_mod(),
                0x9 => self.mul_mod(),
                0xa => self.exp(),
                0xb => self.sign_extend(),
                0x10 => self.lt(),
                0x11 => self.gt(),
                0x12 => self.slt(),
                0x13 => self.sgt(),
                0x14 => self.eq(),
                0x15 => self.iszero(),
                0x16 => self.and(),
                0x17 => self.or(),
                0x18 => self.xor(),
                0x19 => self.not(),
                0x1a => self.byte(),
                0x1b => self.shl(),
                0x1c => self.shr(),
                0x1d => self.sar(),
                0x20 => self.sha3(),
                0x30 => self.address(),
                0x31 => self.balance(),
                0x32 => self.origin(),
                0x33 => self.caller(),
                0x34 => self.callvalue(),
                0x35 => self.calldata_load(),
                0x36 => self.calldata_size(),
                0x37 => self.calldata_copy(),
                0x38 => self.code_size(),
                0x39 => self.codecopy(),
                0x3a => self.gasprice(),
                0x3b => self.ext_codesize(),
                0x3c => self.ext_codecopy(),
                0x3d => self.return_data_size(),
                0x3e => self.return_data_copy(),
                0x3f => self.ext_codehash(),
                0x40 => self.blockhash(),
                0x41 => self.coinbase(),
                0x42 => self.timestamp(),
                0x43 => self.number(),
                0x44 => self.difficulty(),
                0x45 => self.gaslimit(),
                0x50 => self.pop(),
                0x51 => self.mload(),
                0x52 => self.mstore(),
                0x53 => self.mstore8(),
                0x54 => self.sload(cycle_solver),
                0x55 => self.sstore(cycle_solver),
                0x56 => self.jump(),
                0x57 => self.jumpi(),
                0x58 => self.pc(),
                0x59 => self.msize(),
                0x5a => self.gas(),
                0x5b => self.jumpdest(),
                0x60 => self.push(1),
                0x61 => self.push(2),
                0x62 => self.push(3),
                0x63 => self.push(4),
                0x64 => self.push(5),
                0x65 => self.push(6),
                0x66 => self.push(7),
                0x67 => self.push(8),
                0x68 => self.push(9),
                0x69 => self.push(10),
                0x6a => self.push(11),
                0x6b => self.push(12),
                0x6c => self.push(13),
                0x6d => self.push(14),
                0x6e => self.push(15),
                0x6f => self.push(16),
                0x70 => self.push(17),
                0x71 => self.push(18),
                0x72 => self.push(19),
                0x73 => self.push(20),
                0x74 => self.push(21),
                0x75 => self.push(22),
                0x76 => self.push(23),
                0x77 => self.push(24),
                0x78 => self.push(25),
                0x79 => self.push(26),
                0x7a => self.push(27),
                0x7b => self.push(28),
                0x7c => self.push(29),
                0x7d => self.push(30),
                0x7e => self.push(31),
                0x7f => self.push(32),
                0x80 => self.dup(1),
                0x81 => self.dup(2),
                0x82 => self.dup(3),
                0x83 => self.dup(4),
                0x84 => self.dup(5),
                0x85 => self.dup(6),
                0x86 => self.dup(7),
                0x87 => self.dup(8),
                0x88 => self.dup(9),
                0x89 => self.dup(10),
                0x8a => self.dup(11),
                0x8b => self.dup(12),
                0x8c => self.dup(13),
                0x8d => self.dup(14),
                0x8e => self.dup(15),
                0x8f => self.dup(16),
                0x90 => self.swap(1),
                0x91 => self.swap(2),
                0x92 => self.swap(3),
                0x93 => self.swap(4),
                0x94 => self.swap(5),
                0x95 => self.swap(6),
                0x96 => self.swap(7),
                0x97 => self.swap(8),
                0x98 => self.swap(9),
                0x99 => self.swap(10),
                0x9a => self.swap(11),
                0x9b => self.swap(12),
                0x9c => self.swap(13),
                0x9d => self.swap(14),
                0x9e => self.swap(15),
                0x9f => self.swap(16),
                0xa0 => self.log(0),
                0xa1 => self.log(1),
                0xa2 => self.log(2),
                0xa3 => self.log(3),
                0xa4 => self.log(4),
                0xf0 => self.create(),
                0xf1 => self.call(),
                0xf2 => self.call(),
                0xf3 => self.return_(),
                0xf4 => self.delegate_call(),
                0xf5 => self.create2(),
                0xfa => self.static_call(),
                0xfd => self.revert(),
                0xff => self.selfdestruct(),
                _ => self.ended = true,
            }
            self.pc += 1;
        }
    }
    pub fn log_operation(&self, name: &str) {
        //println!("{}\t\x1b[0;34m{}\t{}\x1b[0m", self.pc, name, self.stack);
    }

    /* Instructions */
    pub fn stop(&mut self) {
        self.log_operation("STOP");
        self.ended = true;
    }

    pub fn add(&mut self) {
        self.log_operation("ADD");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Add(Box::from(op1), Box::from(op2)));
    }
    pub fn mul(&mut self) {
        self.log_operation("MUL");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Mul(Box::from(op1), Box::from(op2)));
    }
    pub fn sub(&mut self) {
        self.log_operation("SUB");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Sub(Box::from(op1), Box::from(op2)));
    }
    pub fn div(&mut self) {
        self.log_operation("DIV");
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
    pub fn sdiv(&mut self) {
        self.log_operation("SDIV");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(SDiv(Box::from(op1), Box::from(op2)));
    }
    pub fn mod_(&mut self) {
        self.log_operation("MOD");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Mod(Box::from(op1), Box::from(op2)));
    }
    pub fn smod(&mut self) {
        self.log_operation("SMOD");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(SMod(Box::from(op1), Box::from(op2)));
    }
    pub fn add_mod(&mut self) {
        self.log_operation("ADDMOD");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        let op3 = self.stack.pop();
        self.stack
            .push(AddMod(Box::from(op1), Box::from(op2), Box::from(op3)));
    }
    pub fn mul_mod(&mut self) {
        self.log_operation("MULMOD");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        let op3 = self.stack.pop();
        self.stack
            .push(MulMod(Box::from(op1), Box::from(op2), Box::from(op3)));
    }
    pub fn exp(&mut self) {
        self.log_operation("EXP");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Exp(Box::from(op1), Box::from(op2)));
    }
    pub fn sign_extend(&mut self) {
        self.log_operation("SIGNEXTEND");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(SignExtend(Box::from(op1), Box::from(op2)));
    }
    pub fn lt(&mut self) {
        self.log_operation("LT");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(LT(Box::from(op1), Box::from(op2)));
    }
    pub fn gt(&mut self) {
        self.log_operation("GT");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(GT(Box::from(op1), Box::from(op2)));
    }
    pub fn slt(&mut self) {
        self.log_operation("SLT");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(SLT(Box::from(op1), Box::from(op2)));
    }
    pub fn sgt(&mut self) {
        self.log_operation("SGT");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(SGT(Box::from(op1), Box::from(op2)));
    }
    pub fn eq(&mut self) {
        self.log_operation("EQ");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(EQ(Box::from(op1), Box::from(op2)));
    }
    pub fn iszero(&mut self) {
        self.log_operation("ISZERO");
        let op = self.stack.pop();
        self.stack.push(IsZero(Box::from(op)));
    }
    pub fn and(&mut self) {
        self.log_operation("AND");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(And(Box::from(op1), Box::from(op2)));
    }
    pub fn or(&mut self) {
        self.log_operation("OR");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Or(Box::from(op1), Box::from(op2)));
    }
    pub fn xor(&mut self) {
        self.log_operation("XOR");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Xor(Box::from(op1), Box::from(op2)));
    }
    pub fn not(&mut self) {
        self.log_operation("NOT");
        let op1 = self.stack.pop();
        self.stack.push(Not(Box::from(op1)));
    }
    pub fn byte(&mut self) {
        self.log_operation("BYTE");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Byte(Box::from(op1), Box::from(op2)));
    }
    pub fn shl(&mut self) {
        self.log_operation("SHL");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(ShL(Box::from(op1), Box::from(op2)));
    }
    pub fn shr(&mut self) {
        self.log_operation("SHR");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Shr(Box::from(op1), Box::from(op2)));
    }
    pub fn sar(&mut self) {
        self.log_operation("SAR");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.stack.push(Sar(Box::from(op1), Box::from(op2)));
    }
    pub fn sha3(&mut self) {
        self.log_operation("SHA3");
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
    pub fn address(&mut self) {
        self.log_operation("ADDRESS");
        self.stack.push(Address);
    }
    pub fn balance(&mut self) {
        self.log_operation("BALANCE");
        let address = self.stack.pop();
        self.stack.push(Balance(Box::from(address)));
    }
    pub fn origin(&mut self) {
        self.log_operation("ORIGIN");
        self.stack.push(Origin);
    }
    pub fn caller(&mut self) {
        self.log_operation("CALLER");
        self.stack.push(Caller);
    }
    pub fn callvalue(&mut self) {
        self.log_operation("CALLVALUE");
        self.stack.push(CallValue);
    }
    pub fn calldata_load(&mut self) {
        self.log_operation("CALLDATALOAD");
        let op1 = self.stack.pop();
        self.stack.push(CallDataLoad(Box::from(op1)));
    }
    pub fn calldata_size(&mut self) {
        self.log_operation("CALLDATASIZE");
        self.stack.push(CallDataSize);
    }
    pub fn calldata_copy(&mut self) {
        self.log_operation("CALLDATACOPY");
        let length = self.stack.pop();
        let offset = self.stack.pop();
        let dest_offset = self.stack.pop();
        let length_clone = length.clone();
        let value = CalldataCopy(Box::from(length), Box::from(offset));
        self.memory.store(dest_offset, value, length_clone);
    }
    pub fn code_size(&mut self) {
        self.log_operation("CODESIZE");
        self.stack.push(CodeSize);
    }
    pub fn codecopy(&mut self) {
        self.log_operation("CODECOPY");
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
    pub fn gasprice(&mut self) {
        self.log_operation("GASPRICE");
        self.stack.push(GasPrice);
    }
    pub fn ext_codesize(&mut self) {
        self.log_operation("EXTCODESIZE");
        let op1 = self.stack.pop();
        self.stack.push(ExtCodeSize(Box::from(op1)));
    }
    pub fn ext_codecopy(&mut self) {
        self.log_operation("EXTCODECOPY");
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
    pub fn return_data_size(&mut self) {
        self.log_operation("RETURNDATASIZE");
        self.stack.push(ReturnDataSize);
    }
    pub fn return_data_copy(&mut self) {
        self.log_operation("RETURNDATACOPY");
        let length = self.stack.pop();
        let code_offset = self.stack.pop();
        let dest_offset = self.stack.pop();
        let value = ReturnDataCopy(Box::from(code_offset), Box::from(length.clone()));
        self.memory.store(dest_offset, value, length);
    }
    pub fn ext_codehash(&mut self) {
        self.log_operation("EXTCODEHASH");
        let op1 = self.stack.pop();
        self.stack.push(ExtCodeHash(Box::from(op1)));
    }
    pub fn blockhash(&mut self) {
        self.log_operation("BLOCKHASH");
        let op1 = self.stack.pop();
        self.stack.push(Blockhash(Box::from(op1)));
    }
    pub fn coinbase(&mut self) {
        self.log_operation("COINBASE");
        self.stack.push(CoinBase);
    }
    pub fn timestamp(&mut self) {
        self.log_operation("TIMESTAMP");
        self.stack.push(TimeStamp);
    }
    pub fn number(&mut self) {
        self.log_operation("NUMBER");
        self.stack.push(Number);
    }
    pub fn difficulty(&mut self) {
        self.log_operation("DIFFICULTY");
        self.stack.push(Difficulty);
    }
    pub fn gaslimit(&mut self) {
        self.log_operation("GASLIMIT");
        self.stack.push(GasLimit);
    }
    pub fn pop(&mut self) {
        self.log_operation("POP");
        self.stack.pop();
    }
    pub fn mload(&mut self) {
        self.log_operation("MLOAD");
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
    pub fn mstore(&mut self) {
        self.log_operation("MSTORE");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.memory.store(op1, op2, ActualValue(U256::from(32)));
    }
    pub fn mstore8(&mut self) {
        self.log_operation("MSTORE8");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.memory.store(op1, op2, ActualValue(U256::from(1)));
    }
    pub fn sload(&mut self, cycle_solver: &mut dyn CycleSolver) {
        self.log_operation("SLOAD");
        let op1 = self.stack.pop();
        self.stack.push(SLoad(Box::from(op1.clone())));
        self.storage_access_read.insert(op1);
    }
    pub fn sstore(&mut self, cycle_solver: &mut dyn CycleSolver) {
        self.log_operation("SSTORE");
        let storage = self.stack.pop();
        self.stack.pop();
        self.storage_access_write.insert(storage);
    }
    pub fn jump(&mut self) {
        self.log_operation("JUMP");
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
    pub fn jumpi(&mut self) {
        self.log_operation("JUMPI");
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
    pub fn pc(&mut self) {
        self.log_operation("PC");
        self.stack.push(ActualValue(U256::from(self.pc)));
    }
    pub fn msize(&mut self) {
        self.log_operation("MSIZE");
        self.stack.push(MSize);
    }
    pub fn gas(&mut self) {
        self.log_operation("GAS");
        self.stack.push(Gas);
    }
    pub fn jumpdest(&mut self) {
        self.log_operation("JUMPDEST");
        /*if self.jumped {
            self.jumped = false;
            return;
        }
        self.maybecycle.insert(self.pc);*/
    }
    pub fn push(&mut self, length: usize) {
        let value = &self.code[self.pc + 1..(self.pc + length + 1)];
        self.log_operation(&format!("PUSH({:?})", value)[..]);
        self.pc += length;
        self.stack.push(ActualValue(U256::from(value)));
    }
    pub fn dup(&mut self, n: usize) {
        self.log_operation(&format!("DUP {}", n)[..]);
        let value = self.stack.clone_pos(n);
        self.stack.push(value);
    }
    pub fn swap(&mut self, n: usize) {
        self.log_operation(&format!("SWAP {}", n)[..]);
        self.stack.swap(n);
    }
    pub fn log(&mut self, n: usize) {
        self.log_operation(&format!("LOG {}", n)[..]);
        self.stack.pop();
        self.stack.pop();
        for _ in 0..n {
            self.stack.pop();
        }
    }
    pub fn create(&mut self) {
        self.log_operation("CREATE");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        let op3 = self.stack.pop();
        self.stack
            .push(Create(Box::from(op1), Box::from(op2), Box::from(op3)));
    }
    pub fn call(&mut self) {
        self.log_operation("CALL");
        let op1 = self.stack.pop();
        let address = self.stack.pop();
        let op3 = self.stack.pop();
        let offset = self.stack.pop();
        let mem_length = self.stack.pop();
        let op6 = self.stack.pop();
        let op7 = self.stack.pop();
        let mem_value = self.memory.retrive(offset.clone(), mem_length.clone());
        self.external_calls
            .insert((address.clone(), mem_value.unwrap()));
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
    pub fn return_(&mut self) {
        self.log_operation("RETURN");
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        self.return_value = Some((op2, op1));
        self.ended = true;
    }
    pub fn delegate_call(&mut self) {
        self.log_operation("DELEGATECALL");
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
    pub fn create2(&mut self) {
        self.log_operation("CREATE2");
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
    pub fn static_call(&mut self) {
        self.log_operation("STATICCALL");
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
    pub fn revert(&mut self) {
        self.log_operation("REVERT");
        // Revert all changes
        self.storage_access_read = HashSet::new();
        self.storage_access_write = HashSet::new();
        self.external_calls = HashSet::new();
        self.ended = true;
    }
    pub fn selfdestruct(&mut self) {
        self.log_operation("SELFDESTRUCT");
        self.ended = true;
    }
}
