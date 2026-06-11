use std::collections::HashMap;

use crate::vm::{HeapValue, StackValue, TypeError, VM};
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
    pub fn new_hmap(&mut self) -> Result<(), TypeError> {
        let vec = HashMap::new();
        let ptr = self.heap.len();
        self.heap.push(HeapValue::HMap(vec));
        self.stack.push(StackValue::Ptr(ptr));
        Ok(())
    }
    pub fn hmap_insert(&mut self) -> Result<(), TypeError> {
        let value = self.stack.pop().unwrap();
        let key = self.stack.pop().unwrap();
        let ptr = self.stack.pop().unwrap().ptr()?;
        let hmap = self.extract_hmap_from_heap_mut(ptr)?;
        hmap.insert(key, value);
        Ok(())
    }
    pub fn hmap_get(&mut self) -> Result<(), TypeError> {
        let key = self.stack.pop().unwrap();
        let ptr = self.stack.pop().unwrap().ptr()?;
        let value = {
            let hmap = self.extract_hmap_from_heap_mut(ptr)?;
            *hmap.get(&key).unwrap_or(&StackValue::Nil)
        };
        self.stack.push(value);
        Ok(())
    }
    pub fn hmap_contains(&mut self) -> Result<(), TypeError> {
        let key = self.stack.pop().unwrap();
        let ptr = self.stack.pop().unwrap().ptr()?;
        let value = {
            let hmap = self.extract_hmap_from_heap_mut(ptr)?;
            hmap.contains_key(&key)
        };
        self.stack.push(StackValue::Bool(value));
        Ok(())
    }
    pub fn hmap_remove(&mut self) -> Result<(), TypeError> {
        let key = self.stack.pop().unwrap();
        let ptr = self.stack.pop().unwrap().ptr()?;
        let value = {
            let hmap = self.extract_hmap_from_heap_mut(ptr)?;
            hmap.remove(&key).unwrap_or(StackValue::Nil)
        };
        self.stack.push(value);
        Ok(())
    }
}
