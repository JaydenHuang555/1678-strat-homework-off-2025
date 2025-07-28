/*
*
* TODO: add chaining
*
* */
use std::hash::{Hasher};

type Node<T> = Option<Box<RawNode<T>>>;

const _CAP: usize = 256;
#[derive(Clone)]
struct RawNode<T> {
   prev: Node<T>, 
   next: Node<T>, 
   val: T
}

impl<T> RawNode<T> {
   fn new(val: T) -> Self {
      Self {
         prev: Option::None,
         next: Option::None,
         val: val
      }
   }
}

pub struct LookUpTable<T : Copy + std::hash::Hash + PartialEq> {
   table: [Node<T>; _CAP],
   size: usize,
}

impl<T : std::hash::Hash + Copy + PartialEq> LookUpTable<T> {

   fn alloc_node(&self, val: T) -> Node<T> {
      Option::Some(Box::new(RawNode::new(val)))
   }

   pub fn new() -> Self {
      Self {
         table: std::array::from_fn(|_| {
            Option::None
         }),
         size: _CAP,
      }
   }

   fn get_hash(&self, item: T) -> usize {
      let mut hasher: std::hash::DefaultHasher = std::hash::DefaultHasher::new();
      item.hash(&mut hasher); 
      (hasher.finish() as usize) % self.size
   }

   pub fn insert(&mut self, item: T) -> ()  {
      self.table[self.get_hash(item)] = self.alloc_node(item);
   }

   pub fn has(&self, item: T) -> bool {
      match &self.table[self.get_hash(item)] {
         Option::None => false,
         Option::Some(val) => {
            if val.val == item {
               return true;
            }
            false
         }
      }
   }

}
