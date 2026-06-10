use crate::vm::{StackValue, VM};
impl VM {
    pub fn and(&mut self) {
        let b = self.stack.pop().unwrap().bool().unwrap();
        let a = self.stack.pop().unwrap().bool().unwrap();
        self.stack.push(StackValue::Bool(a && b));
    }
    pub fn or(&mut self) {
        let b = self.stack.pop().unwrap().bool().unwrap();
        let a = self.stack.pop().unwrap().bool().unwrap();
        self.stack.push(StackValue::Bool(a || b));
    }
    pub fn not(&mut self) {
        let a = self.stack.pop().unwrap().bool().unwrap();
        self.stack.push(StackValue::Bool(!a));
    }
    pub fn xor(&mut self) {
        let b = self.stack.pop().unwrap().bool().unwrap();
        let a = self.stack.pop().unwrap().bool().unwrap();
        self.stack.push(StackValue::Bool(a ^ b));
    }
    pub fn nor(&mut self) {
        let b = self.stack.pop().unwrap().bool().unwrap();
        let a = self.stack.pop().unwrap().bool().unwrap();
        self.stack.push(StackValue::Bool(!(a || b)));
    }
    pub fn nand(&mut self) {
        let b = self.stack.pop().unwrap().bool().unwrap();
        let a = self.stack.pop().unwrap().bool().unwrap();
        self.stack.push(StackValue::Bool(!(a && b)));
    }
}
