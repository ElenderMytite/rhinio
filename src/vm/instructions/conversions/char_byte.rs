use crate::vm::{ExecutionError, StackValue, Type, VM};
impl VM {
    pub fn byte(&mut self) -> Result<(), ExecutionError> {
        let c = self.stack.pop().unwrap().char().unwrap();
        self.stack.push(StackValue::Int(c as isize));
        Ok(())
    }
    pub fn char(&mut self) -> Result<(), ExecutionError> {
        let i = self.stack.pop().unwrap().int()?;
        self.stack.push(StackValue::Char(checked_char(i as u32)?));
        Ok(())
    }
}
fn checked_char(x: u32) -> Result<char, ExecutionError> {
    match char::from_u32(x) {
        Some(c) => Ok(c),
        None => Err(ExecutionError::ConversionError {
            from: Type::Int,
            to: Type::Char,
        }),
    }
}
