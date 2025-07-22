mod info;

use std::env;
use crate::info::engine::Engine;
//  [u32; 3]
fn main() {
   let _args: Vec<String> = env::args().collect();
   let mut red: [u32; 3] = [0, 0, 0];
   let mut blue: [u32; 3] = [0, 0, 0];
   let engine: Engine = Engine::new(red, blue); 
}
