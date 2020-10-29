use crate::evm_types::StackValue;
use ethereum_types::U256;
use std::fmt;

#[derive(Debug, Clone)]
pub struct EvmStack {
    stack: Vec<StackValue>,
    deficit: usize,
    ///Callee values edited by this function
    calee_edits: Vec<(usize, StackValue)>,
}
#[derive(Debug, Clone)]
pub struct EvmMemory {
    actual_memory: Vec<(StackValue, StackValue, StackValue)>,
}
impl EvmMemory {
    pub fn new() -> Self {
        EvmMemory {
            actual_memory: Vec::new(),
        }
    }
    pub fn store(&mut self, offset: StackValue, value: StackValue, length: StackValue) {
        for i in 0..self.actual_memory.len() {
            if self.actual_memory[i].0 == offset {
                self.actual_memory.remove(i);
                break;
            }
        }
        self.actual_memory.push((offset, value, length));
        //self.print_memory();
    }
    pub fn retrive(&self, offset: StackValue, l: StackValue) -> Option<StackValue> {
        //self.print_memory();
        for el in self.actual_memory.iter().rev() {
            if el.0 == offset {
                return Some(el.1.clone());
            }
        }
        Some(StackValue::MemoryPlaceHolder(
            Box::from(offset),
            Box::from(l),
        ))
    }
    /// Attempt to retrive more than one consecutive memory position; this is only possible if the memory offsets and lengths can be resolved as U256
    pub fn retrive_array(&self, offset: U256, length: U256) -> Vec<(usize, StackValue)> {
        //self.print_memory();
        let mut vector: Vec<(usize, StackValue)> = Vec::new();
        for el in self.actual_memory.iter().rev() {
            let pos = if let Some(x) = el.0.resolve() {
                x
            } else {
                continue;
            };
            let mem_length = if let Some(x) = el.2.resolve() {
                x
            } else {
                continue;
            };
            if pos >= offset && pos + mem_length <= offset + length {
                vector.push((pos.as_usize(), el.1.clone()));
            }
        }
        vector
    }

    pub fn print_memory(&self) {
        print!("\x1b[0;31mMEMORY:",);
        let c1 = "\x1b[0;31m";
        let c2 = "\x1b[0;37m";
        let mut counter = 0;
        let mut buffer = String::new();
        for item in &self.actual_memory {
            if counter % 2 == 0 {
                print!("{}{:?}//", c1, item);
            } else {
                print!("{}{:?}//", c2, item);
            }
            counter += 1;
        }
        print!("\x1b[0m\n")
    }
}

impl EvmStack {
    pub fn new() -> Self {
        EvmStack {
            stack: Vec::new(),
            deficit: 0,
            calee_edits: Vec::new(),
        }
    }
    fn previous_pop(&mut self, position: usize) -> StackValue {
        let mut ret = StackValue::StackPaceHolder(position);
        let mut remindex = None;
        for (index, edit) in self.calee_edits.iter().enumerate() {
            if edit.0 == position {
                ret = edit.1.clone();
                remindex = Some(index);
            }
        }
        if let Some(i) = remindex {
            self.calee_edits.remove(i);
        }
        ret
    }
    fn previous_clone(&self, position: usize) -> StackValue {
        let mut ret = StackValue::StackPaceHolder(position);
        for (index, edit) in self.calee_edits.iter().enumerate() {
            if edit.0 == position {
                ret = edit.1.clone();
            }
        }
        ret
    }
    pub fn pop(&mut self) -> StackValue {
        if let Some(v) = self.stack.pop() {
            v
        } else {
            self.deficit += 1;
            self.previous_pop(self.deficit)
        }
    }
    pub fn push(&mut self, value: StackValue) {
        if self.deficit <= 0 {
            self.stack.push(value);
        } else {
            self.calee_edits.push((self.deficit, value));
            self.deficit -= 1;
        }
    }
    pub fn clone_pos(&self, position: usize) -> StackValue {
        if position <= self.stack.len() {
            let pointer = self.stack.get(self.stack.len() - position).unwrap();
            (*pointer).clone()
        } else {
            let deficit = position - self.stack.len();
            self.previous_clone(self.deficit + deficit)
        }
    }
    pub fn swap(&mut self, i: usize) {
        if i < self.stack.len() {
            let len = self.stack.len() - 1;
            self.stack.swap(len, len - i);
        } else {
            let el1 = self.pop();
            let pre = i - self.stack.len();
            let st_deficit = self.deficit;
            let el2 = self.previous_pop(st_deficit + pre);
            self.push(el2);
            self.calee_edits.push((st_deficit + pre, el1));
        }
    }
}

impl fmt::Display for EvmStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c1 = "\x1b[0;32m";
        let c2 = "\x1b[0;37m";
        let mut buffer = String::new();
        buffer += &format!("[[{}]]", self.deficit)[..];
        for (counter, item) in self.stack.iter().enumerate() {
            if counter % 2 == 0 {
                buffer += &format!("{}{:?}//", c1, item)[..];
            } else {
                buffer += &format!("{}{:?}//", c2, item)[..];
            }
        }
        write!(f, "{}", buffer)
    }
}
