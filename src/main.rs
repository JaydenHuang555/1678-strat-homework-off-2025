mod info;
mod rawinfo;
use crate::info::{engine};
use crate::rawinfo::raw_info;
use std::{env, io::Read};

pub fn main() -> Result<(), ()> {

   let args: Vec<String> = std::env::args().skip(1).collect();

   for arg in args {
      println!("got arg {}", arg);
      match std::fs::File::open(&arg) {
         Ok(stream) => {
            let reader: std::io::BufReader<std::fs::File> = std::io::BufReader::new(stream);
            match serde_json::from_reader::<_, Vec<raw_info::RawAlliance>>(reader) {
               Ok(raw) => {
                  println!("able to read info");
                  println!("alliance is {}", raw[0].alliance);
                  println!("alliance is {}", raw[1].alliance);
               }
               Err(e) => {
                  eprintln!("error when reading file with error {}", e);
                  return Err(());
               }
            }
         }
         Err(e) => {
            eprintln!("unable to open file {} due to error {}", &arg, e);
            return Err(());
         }
      }
   } 

   Ok(())
}

