use std::collections::HashMap;

use crate::vm::{ExecutionError, HeapValue, StackValue, TypeError, VM};
impl VM {
    fn extract_hmap_from_heap_mut(
        &mut self,
        index: usize,
    ) -> Result<&mut HashMap<StackValue, StackValue>, TypeError> {
        let val = &mut self.heap[index];
        match val {
            HeapValue::HMap(map) => Ok(map),
            _ => Err(TypeError),
        }
    }
    pub fn new_hmap(&mut self) {
        let hmap = HashMap::new();
        let ptr = self.heap.len();
        self.heap.push(HeapValue::HMap(hmap));
        self.stack.push(StackValue::Pointer(ptr));
    }
    pub fn hmap_insert(&mut self) -> Result<(), ExecutionError> {
        let value = self.stack_pop()?;
        let key = self.stack_pop()?;
        let ptr = self.stack.pop().unwrap().ptr()?;
        let hmap = self.extract_hmap_from_heap_mut(ptr)?;
        eprintln!("Successfully extracted hmap: {:?}", hmap.clone());
        dbg!(key, value, ptr, hmap.clone());
        hmap.insert(key, value);
        Ok(())
    }
    pub fn hmap_get(&mut self) -> Result<(), ExecutionError> {
        let key = self.stack_pop()?;
        eprintln!("Successfully got key from the stack: {:?}", key);
        let ptr = self.stack.pop().unwrap().ptr()?;
        eprintln!("Successfully got pointer from the stack: {:?}", ptr);
        let value = {
            let hmap = self.extract_hmap_from_heap_mut(ptr)?;
            *hmap.get(&key).unwrap_or(&StackValue::Bool(false)) // TODO: implement option type
        };
        self.stack.push(value);
        Ok(())
    }
    pub fn hmap_contains(&mut self) -> Result<(), ExecutionError> {
        let key = self.stack_pop()?;
        let ptr = self.stack.pop().unwrap().ptr()?;
        let value = {
            let hmap = self.extract_hmap_from_heap_mut(ptr)?;
            hmap.contains_key(&key)
        };
        self.stack.push(StackValue::Bool(value));
        Ok(())
    }
    pub fn hmap_remove(&mut self) -> Result<(), ExecutionError> {
        let key = self.stack_pop()?;
        let ptr = self.stack.pop().unwrap().ptr()?;
        let value = {
            let hmap = self.extract_hmap_from_heap_mut(ptr)?;
            hmap.remove(&key).unwrap_or(StackValue::Bool(false)) // TODO: implement option type
        };
        self.stack.push(value);
        Ok(())
    }
}
