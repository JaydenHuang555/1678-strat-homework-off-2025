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

   fn alloc_node(val: T) -> Node<T> {
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

   // pub fn insert(&mut self, item: T) -> ()  {
   //    let mut current: &mut Node<T> = &mut self.table[self.get_hash(item)];
   //    while let Some(node) = current {
   //       current = &mut node.next;
   //    }
   //    *current = self.alloc_node(item);
   // }

   pub fn insert(&mut self, item: T) {
      let mut current: &mut Node<T> = &mut self.table[self.get_hash(item)];

      loop {
         match current {
            Some(node) => {
               current = &mut node.next;
            }
            None => break,
         }
      }
      *current = Self::alloc_node(item);
   }


   pub fn has(&mut self, item: T) -> bool {
      let mut next: &mut Node<T> = &mut self.table[self.get_hash(item)];
      loop {
         match next {
            Some(node) => {
               if node.val == item {
                  return true;
               }
               next = &mut node.next;
            }
            None => return false
         }
      }
   }

}
