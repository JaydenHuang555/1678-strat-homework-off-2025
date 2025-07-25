mod info;
mod pair;
mod lookuptable;
mod common;
mod rawinfo;
mod convert;
use crate::info::{engine::Engine};
use crate::rawinfo::raw_info;
use std::{env, io::Read};
use crate::rawinfo::raw_info::RawAlliance;

pub fn main() -> Result<(), i32> {

   let args: Vec<String> = std::env::args().skip(1).collect();
   if args.len() == 0 {
      eprintln!("please enter args");
      return Err(1);
   }
   for arg in args {
      println!("got arg {}", &arg);
      match std::fs::File::open(&arg) {
         Ok(stream) => {
            let reader: std::io::BufReader<std::fs::File> = std::io::BufReader::new(stream);
            match serde_json::from_reader::<_, raw_info::RawInfo>(reader) {
               Ok(raw) => {
                  println!("able to read json {} and able to convert to raw", &arg);
                  convert::convert(raw);
               }
               Err(e) => {
                  eprintln!("error when reading file with error {}", e);
                  return Err(2);
               }
            }
         }
         Err(e) => {
            eprintln!("unable to open file {} due to error {}", &arg, e);
            return Err(3);
         }
      }
   }
   Ok(())
}

