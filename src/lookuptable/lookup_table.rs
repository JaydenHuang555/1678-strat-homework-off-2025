/*
*
* TODO: add chaining
*
* */
use std::hash::{Hasher};

const _CAP: usize = 256;

struct Node<T> {
   prev: Box<Node<T>>,
   next: Box<Node<T>>,
   val: T
}

pub struct LookUpTable<T : Copy + std::hash::Hash> {
   table: [Option<T>; _CAP],
   size: usize,
}

impl<T : std::hash::Hash + Copy> LookUpTable<T> {
   pub fn new() -> Self {
      Self {
         table: [Option::None; _CAP],
         size: _CAP,
      }
   }

   fn get_hash(&self, item: T) -> usize {
      let mut hasher: std::hash::DefaultHasher = std::hash::DefaultHasher::new();
      item.hash(&mut hasher); 
      (hasher.finish() as usize) % self.size
   }

   pub fn insert(&mut self, item: T) -> ()  {
      self.table[self.get_hash(item)] = Option::Some(item);
   }

   pub fn has(&self, item: T) -> bool {
      match self.table[self.get_hash(item)] {
         Option::None => false,
         Option::Some(_) => true
      }
   }

} 
