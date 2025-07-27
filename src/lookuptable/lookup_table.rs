use std::mem::{self, MaybeUninit};
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
   val: Option<T>,
}

impl<T> Default for RawNode<T> {
   fn default() -> Self {
      Self {
         prev: Option::None,
         next:Option::None,
         val: Option::None
      }
   }
}

pub struct LookUpTable<T : Copy + std::hash::Hash> {
   table: [std::mem::MaybeUninit<Node<T>>; _CAP],
   size: usize,
}

impl<T : std::hash::Hash + Copy> LookUpTable<T> {

   pub fn new() -> Self {
      let next: [std::mem::MaybeUninit<Node<T>>; _CAP] = {
         let mut next: [std::mem::MaybeUninit<Node<T>>; _CAP] = unsafe {
            std::mem::MaybeUninit::uninit().assume_init()
         };
         for i in 0 .. _CAP {
            next[i] = MaybeUninit::new(Option::None);
         }
         unsafe {
            mem::transmute::<_, [Node<T>; _CAP]>(next);
         }
      };
      Self {
         table: 
         ,
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
