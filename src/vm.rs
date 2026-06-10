use crate::ir::Command;
use std::cmp::max;
mod instructions;
pub(crate) struct VM {
    pub ip: usize,
    pub flush: bool,
    pub code: Vec<Command>,
    pub stack: Vec<StackValue>,
    pub heap: Vec<HeapValue>,
    pub env: Vec<StackValue>,
}
impl VM {
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
    pub fn execute(&mut self) {
        while self.ip < self.code.len() {
            match self.code[self.ip].clone() {
                Command::Add => self.add(),
                Command::Sub => self.sub(),
                Command::Mul => self.mul(),
                Command::Div => self.div(),
                Command::Mod => self.modd(),
                Command::Byte => self.byte(),
                Command::Char => self.char(),
                Command::Cls => self.cls(),
                Command::Dup => self.dup(),
                Command::Swap => self.swap(),
                Command::Del => self.drop(),
                Command::Put(value) => self.put(value),
                Command::Print => self.print(),
                Command::Eq => self.eq(),
                Command::Neq => self.neq(),
                Command::Geq => self.geq(),
                Command::Leq => self.leq(),
                Command::Gt => self.gt(),
                Command::Ls => self.ls(),
                Command::Not => self.not(),
                Command::And => self.and(),
                Command::Or => self.or(),
                Command::Xor => self.xor(),
                Command::Nor => self.nor(),
                Command::Nand => self.nand(),
                Command::Load(adress) => {
                    if adress >= self.env.len() {
                        self.stack.push(StackValue::Nil);
                    } else {
                        self.stack.push(self.env[adress].clone());
                    }
                }
                Command::Store(adress) => {
                    assert!(self.stack.len() >= 1);
                    self.env
                        .resize(max(adress + 1, self.env.len()), StackValue::Nil);
                    self.env[adress] = self.stack.pop().unwrap();
                }
                Command::Jmp(adress) => {
                    assert!(self.stack.len() >= 1);
                    if self.stack.pop().unwrap().bool().unwrap() {
                        self.ip = adress;
                        continue;
                    }
                }
                Command::Len => self.len(),
                Command::VNew => self.new_vec(),
                // Command::HNew => {
                //     let ptr = self.heap.len();
                //     self.heap.push(HeapValue::HMap(HashMap::new()));
                //     self.stack.push(StackValue::Ptr(ptr));
                // }
                Command::VPop => self.vec_pop(),
                Command::VPush => self.vec_push(),
                Command::Get => self.vec_get(),
            }
            self.ip += 1;
        }
    }
}
pub struct TypeError;
impl std::fmt::Debug for TypeError {
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
    // HMap(HashMap<StackValue, StackValue>),
    // Str(String),
}
impl HeapValue {
    pub fn len(&self) -> usize {
        match self {
            HeapValue::Vector(vec) => vec.len(),
            // HeapValue::HMap(map) => map.len(),
            // HeapValue::Str(s) => s.len(),
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
                } // HeapValue::HMap(map) => {
                  //     let elements: Vec<String> = map
                  //         .iter()
                  //         .map(|(k, v)| format!("{}: {}", print_value(k, vm), print_value(v, vm)))
                  //         .collect();
                  //     format!("{{{}}}", elements.join(", "))
                  // }
                  // HeapValue::Str(s) => s.clone(),
            }
        }
    }
}
