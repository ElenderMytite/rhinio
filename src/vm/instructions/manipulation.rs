use crate::vm::{StackValue, VM, print_value};
impl VM {
    pub fn dup(&mut self) {
        let a = self.stack.last().unwrap().clone();
        self.stack.push(a);
    }
    pub fn swap(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a);
        self.stack.push(b);
    }
    pub fn drop(&mut self) {
        self.stack.pop();
    }
    pub fn cls(&mut self) {
        if self.flush {
            println!();
            self.flush = false;
        }
        self.stack.clear();
    }
    pub fn print(&mut self) {
        let value = self.stack.pop();
        if value.is_none() {
            return;
        }
        let value = value.unwrap();
        print!("{}", print_value(&value, &self));
        self.flush = true;
    }
    pub fn put(&mut self, value: StackValue) {
        self.stack.push(value);
    }
}
