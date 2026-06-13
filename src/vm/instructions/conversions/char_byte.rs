use crate::vm::{ExecutionError, StackValue, VM};
impl VM {
    pub fn byte(&mut self) -> Result<(), ExecutionError> {
        let c = self.stack.pop().unwrap().char().unwrap();
        self.stack.push(StackValue::Int(c as isize));
        Ok(())
    }
    pub fn char(&mut self) -> Result<(), ExecutionError> {
        let i = self.stack.pop().unwrap().int()?;
        self.stack.push(StackValue::Char(i as u8 as char));
        Ok(())
    }
}
