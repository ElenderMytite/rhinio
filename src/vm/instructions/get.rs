// get can be used for both hmap and vec types in this language. When get command is called,
// vm first checks if it is a hmap or a vec, and then calls corresponding command
use crate::{
    VM,
    vm::{HeapValue, TypeError},
};
impl VM {
    fn extract_heap_value(&mut self, index: usize) -> Result<&HeapValue, TypeError> {
        Ok(&mut self.heap[index])
    }
    pub fn get(&mut self) -> Result<(), TypeError> {
        let ptr = self.stack.last().unwrap().ptr()?;
        let collection = self.extract_heap_value(ptr)?;
        match collection {
            HeapValue::HMap(_) => self.hmap_get()?,
            HeapValue::Vector(_) => self.vec_get()?,
            _ => todo!(),
        }
        Ok(())
    }
}
