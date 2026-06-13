use crate::vm::{ExecutionError, HeapValue, StackValue, TypeError, VM};
impl VM {
    pub fn new_vec(&mut self) {
        let vec = Vec::new();
        let ptr = self.heap.len();
        self.heap.push(HeapValue::Vector(vec));
        self.stack.push(StackValue::Ptr(ptr));
    }
    fn extract_vec_from_heap_mut(
        &mut self,
        index: usize,
    ) -> Result<&mut Vec<StackValue>, TypeError> {
        let heap_val = &mut self.heap[index];
        match heap_val {
            HeapValue::Vector(vec) => Ok(vec),
            _ => Err(TypeError),
        }
    }
    pub fn vec_pop(&mut self) -> Result<(), ExecutionError> {
        let ptr = self.stack.pop().unwrap().ptr()?;
        let vec = self.extract_vec_from_heap_mut(ptr);
        let value = vec?.pop().unwrap();
        self.stack.push(value);
        Ok(())
    }
    pub fn vec_push(&mut self) -> Result<(), ExecutionError> {
        let value = self.stack_pop()?;
        let ptr = self.stack.last().unwrap().ptr()?;
        let vec = self.extract_vec_from_heap_mut(ptr);
        vec?.push(value);
        Ok(())
    }
    pub fn vec_get(&mut self) -> Result<(), ExecutionError> {
        let index = self.stack.pop().unwrap().int()? as usize;
        let ptr = self.stack.pop().unwrap().ptr()?;
        let vec = self.extract_vec_from_heap_mut(ptr)?;
        let idx = ((index % (vec.len() as isize) as usize + vec.len()) % vec.len()) as usize; // Handle negative indices and wrap around
        let value = vec[idx].clone();
        self.stack.push(value);
        Ok(())
    }
    pub fn len(&mut self) -> Result<(), ExecutionError> {
        let ptr = self.stack.pop().unwrap().ptr()?;
        let structure = &self.heap[ptr];
        self.stack.push(StackValue::Int(structure.len() as isize));
        Ok(())
    }
}
