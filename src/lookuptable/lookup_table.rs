/*
*
* TODO: add chaining
*
* */
use std::hash::{Hash, Hasher};

const _CAP: usize = 256;

pub struct LookUpTable<'a, T : 'a + Copy + std::hash::Hash> {
   table: [Option<&'a  T>; _CAP],
   size: usize,
}

impl<'a, T : 'a + std::hash::Hash + Copy> LookUpTable<'a, T> {
   pub fn new() -> Self {
      Self {
         table: [Option::None; _CAP],
         size: _CAP,
      }
   }

   fn get_hash(&self, item: &T) -> usize {
      let mut hasher: std::hash::DefaultHasher = std::hash::DefaultHasher::new();
      item.hash(&mut hasher); 
      (hasher.finish() as usize) % self.size
   }

   pub fn insert(&mut self, item: &'a T) -> ()  {
      self.table[self.get_hash(item)] = Option::Some(&item);
   }

   pub fn has(&self, item: &T) -> bool {
      match self.table[self.get_hash(item)] {
         Option::None => false,
         Option::Some(_) => true
      }
   }

} 
