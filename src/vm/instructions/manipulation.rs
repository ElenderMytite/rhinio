use crate::vm::{ExecutionError, StackValue, VM, print_value};
impl VM {
    pub fn dup(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack.last().unwrap().clone();
        self.stack.push(a);
        Ok(())
    }
    pub fn swap(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack_pop()?;
        let b = self.stack_pop()?;
        self.stack.push(a);
        self.stack.push(b);
        Ok(())
    }
    pub fn drop(&mut self) -> Result<(), ExecutionError> {
        self.stack_pop()?;
        Ok(())
    }
    pub fn cls(&mut self) {
        if self.flush {
            println!();
            self.flush = false;
        }
        self.stack.clear();
    }
    pub fn print(&mut self) -> Result<(), ExecutionError> {
        let value = self.stack_pop()?;
        println!("{}", print_value(&value, &self));
        self.flush = true;
        Ok(())
    }
    pub fn put(&mut self, value: StackValue) {
        self.stack.push(value);
    }
    pub fn load(&mut self, addr: usize) {
        let value = self
            .env
            .get(addr)
            .unwrap_or(&StackValue::Bool(false))
            .clone();
        self.stack.push(value);
    }
    pub fn store(&mut self, addr: usize) -> Result<(), ExecutionError> {
        let value = self.stack_pop()?;
        if addr >= self.env.len() {
            self.env.resize(addr + 1, StackValue::Bool(false));
        }
        self.env[addr] = value;
        Ok(())
    }
    pub fn jump(&mut self, addr: usize) -> Result<(), ExecutionError> {
        let condition = self.stack_pop()?;
        if !condition.bool().unwrap() {
            eprintln!("breaking out of block at {}", self.ip);
            return Ok(());
        }
        if addr >= self.code.len() {
            panic!("Jump address out of bounds: {}", addr);
        }
        eprintln!("jump to {addr}");
        self.ip = addr;
        Ok(())
    }
}
