use crate::ir::Command;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};
mod instructions;
impl From<TypeError> for ExecutionError {
    fn from(value: TypeError) -> Self {
        Self::TypeMismatch(value)
    }
}
#[derive(Debug, Clone, Copy)]
pub enum ExecutionError {
    TypeMismatch(TypeError),
    StackUnderflow,
}
pub struct VM {
    pub ip: usize,
    flush: bool,
    pub code: Vec<Command>,
    /// storage for temporary calculations
    pub stack: Vec<StackValue>,
    /// storage for large data (vector, hash map or a string)
    pub heap: Vec<HeapValue>,
    /// storage for variables
    pub env: Vec<StackValue>,
}
impl VM {
    pub fn stack_pop(&mut self) -> Result<StackValue, ExecutionError> {
        let value = self.stack.pop();
        match value {
            Some(x) => Ok(x),
            None => Err(ExecutionError::StackUnderflow),
        }
    }
    pub fn new(code: Vec<Command>) -> Self {
        Self {
            ip: 0,
            flush: false,
            code: code,
            stack: Vec::new(),
            heap: Vec::new(),
            env: Vec::new(),
        }
    }
    /// prints debugging information before executing each command
    pub fn execute_program(&mut self, debug: bool) -> Result<(), ExecutionError> {
        while self.ip < self.code.len() {
            if debug {
                eprintln!(
                    "{}: {:?}; {:?}",
                    self.ip,
                    self.stack.clone(),
                    self.code[self.ip].clone()
                )
            }
            self.execute_command()?;
        }
        self.collect_garbage();
        Ok(())
        // garbage collection
    }
    fn collect_garbage(&mut self) {
        let mut used = HashSet::new();
        for var in self.env.iter() {
            if let StackValue::Ptr(idx) = var {
                used.insert(idx);
            }
        }
        let mut idx = -1;
        self.heap.retain(|_| {
            idx += 1;
            used.contains(&(idx as usize))
        });
    }
    /// reads command at instruction pointer ip, and calls corresponding function, then increases instruction pointer by one
    fn execute_command(&mut self) -> Result<(), ExecutionError> {
        let command = self.code[self.ip];
        self.ip += 1;
        match command {
            Command::Cls => self.cls(),
            Command::VNew => self.new_vec(),
            Command::HNew => self.new_hmap(),
            Command::Add => return self.add(),
            Command::Sub => return self.sub(),
            Command::Mul => return self.mul(),
            Command::Div => return self.div(),
            Command::Mod => return self.modd(),
            Command::Byte => return self.byte(),
            Command::Char => return self.char(),
            Command::Dup => return self.dup(),
            Command::Swap => return self.swap(),
            Command::Del => return self.drop(),
            Command::Put(value) => self.put(value),
            Command::Print => return self.print(),
            Command::Eq => return self.eq(),
            Command::Neq => return self.neq(),
            Command::Geq => return self.geq(),
            Command::Leq => return self.leq(),
            Command::Gt => return self.gt(),
            Command::Ls => return self.ls(),
            Command::Not => return self.not(),
            Command::And => return self.and(),
            Command::Or => return self.or(),
            Command::Xor => return self.xor(),
            Command::Nor => return self.nor(),
            Command::Nand => return self.nand(),
            Command::Load(addr) => self.load(addr),
            Command::Store(addr) => return self.store(addr),
            Command::Jmp(addr) => return self.jump(addr),
            Command::Get => return self.get(1),
            Command::HContains => return self.hmap_contains(),
            Command::Len => return self.len(),
            Command::VPop => return self.vec_pop(),
            Command::HRemove => return self.hmap_remove(),
            Command::VPush => return self.vec_push(),
            Command::HInsert => return self.hmap_insert(),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Copy)]
pub struct TypeError;
impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Value popped from the self.stack has an unexpected type!"
        )
    }
}
#[derive(Debug, Clone)]
pub enum HeapValue {
    Vector(Vec<StackValue>),
    HMap(HashMap<StackValue, StackValue>),
    _Str(String),
}
impl HeapValue {
    pub fn len(&self) -> usize {
        match self {
            HeapValue::Vector(vec) => vec.len(),
            HeapValue::HMap(map) => map.len(),
            HeapValue::_Str(s) => s.len(),
        }
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum StackValue {
    Bool(bool),
    Int(isize),
    Char(char),
    Ptr(usize), // index in the self.heap
    Nil,
}
impl StackValue {
    pub fn int(&self) -> Result<isize, TypeError> {
        match &self {
            Self::Int(x) => Ok(*x),
            _ => Err(TypeError),
        }
    }
    pub fn bool(&self) -> Result<bool, TypeError> {
        match &self {
            Self::Bool(b) => Ok(*b),
            _ => Err(TypeError),
        }
    }
    pub fn char(&self) -> Result<char, TypeError> {
        match &self {
            Self::Char(c) => Ok(*c),
            _ => Err(TypeError),
        }
    }
    pub fn ptr(&self) -> Result<usize, TypeError> {
        match &self {
            Self::Ptr(p) => Ok(*p),
            _ => Err(TypeError),
        }
    }
}
pub(super) fn print_value(value: &StackValue, vm: &VM) -> String {
    match value {
        StackValue::Nil => "Nil".to_string(),
        StackValue::Int(x) => x.to_string(),
        StackValue::Bool(b) => b.to_string(),
        StackValue::Char(c) => c.to_string(),
        StackValue::Ptr(p) => {
            let heap_val = &vm.heap[*p];
            match heap_val {
                HeapValue::Vector(vec) => {
                    let elements: Vec<String> = vec.iter().map(|v| print_value(v, vm)).collect();
                    format!("[{}]", elements.join(", "))
                }
                HeapValue::HMap(map) => {
                    let elements: Vec<String> = map
                        .iter()
                        .map(|(k, v)| format!("{}: {}", print_value(k, vm), print_value(v, vm)))
                        .collect();
                    format!("{{{}}}", elements.join(", "))
                }
                HeapValue::_Str(s) => s.clone(),
            }
        }
    }
}
