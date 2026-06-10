use crate::vm::{HeapValue, StackValue, VM};
impl VM {
    pub fn new_vec(&mut self) {
        let vec = Vec::new();
        let ptr = self.heap.len();
        self.heap.push(HeapValue::Vector(vec));
        self.stack.push(StackValue::Ptr(ptr));
    }
    pub fn vec_heap_mut(&mut self, index: usize) -> &mut Vec<StackValue> {
        let heap_val = &mut self.heap[index];
        match heap_val {
            HeapValue::Vector(vec) => vec,
        }
    }
    pub fn vec_pop(&mut self) {
        let ptr = self.stack.pop().unwrap().ptr().unwrap();
        let vec = self.vec_heap_mut(ptr);
        let value = vec.pop().unwrap();
        self.stack.push(value);
    }
    pub fn vec_push(&mut self) {
        let value = self.stack.pop().unwrap();
        let ptr = self.stack.last().unwrap().ptr().unwrap();
        let vec = self.vec_heap_mut(ptr);
        vec.push(value);
    }
    pub fn vec_get(&mut self) {
        let index = self.stack.pop().unwrap().int().unwrap() as usize;
        let ptr = self.stack.pop().unwrap().ptr().unwrap();
        let vec = self.vec_heap_mut(ptr);
        let idx = ((index % (vec.len() as isize) as usize + vec.len()) % vec.len()) as usize; // Handle negative indices and wrap around
        let value = vec[idx].clone();
        self.stack.push(value);
    }
    pub fn len(&mut self) {
        let ptr = self.stack.pop().unwrap().ptr().unwrap();
        let structure = &self.heap[ptr];
        self.stack.push(StackValue::Int(structure.len() as isize));
    }
}
