use crate::raw_info::{RawInfo};
use crate::info::{alliance};
use crate::lookuptable::lookup_table::LookUpTable;

fn get_color_from_string(buff: String) -> Result<alliance::AllianceColor, String> {
   match buff.to_lowercase().as_str() {
      "blue" => Result::Ok(alliance::AllianceColor::BLUE),
      "red" => Result::Ok(alliance::AllianceColor::RED),
      _ => Result::Err(format!("unknow color {} found", buff)) 
   }
}
/*
* converts the raw info to a readable format info 
* also checks and validates that the info recieved is valid 
* */
pub fn convert(raw: RawInfo) -> (alliance::Alliance, alliance::Alliance) {
   let mut last_color: Option<alliance::AllianceColor> = Option::None;
   
   let  (mut blue_alliance, mut red_alliance): (Option<alliance::Alliance>, Option<alliance::Alliance>) = (Option::None, Option::None);

   let mut lookup: LookUpTable<u32> = LookUpTable::new(); 
   for raw_alliance in raw.alliances {
      let current_color: alliance::AllianceColor;
      match get_color_from_string(raw_alliance.alliance) {
         Ok(color) => {
            current_color = color;
            match last_color {
               None => {
                  last_color = Option::Some(color);
               }
               Some(last) => {
                  if last == color {
                     panic!("2 alliances with the same color found");
                  }
                  last_color = Option::Some(color);
               }
            }
         }
         Err(e) => {
            panic!("{}", e);
         }
      }
      let mut teams: [u32; 3] = [0; 3];
      if raw_alliance.teams.len() != teams.len() {
         panic!("raw alliance has wrong size");
      }

      for i in 0 .. raw_alliance.teams.len() {
         let next: u32 = raw_alliance.teams[i].number;
         if lookup.has(next) {
            panic!("same team number found for number {}", next);
         }
         lookup.insert(next);
         teams[i] = next;
      }

      match current_color {
         alliance::AllianceColor::BLUE => blue_alliance = Option::Some(alliance::Alliance::new(current_color, teams)),
         alliance::AllianceColor::RED => red_alliance = Option::Some(alliance::Alliance::new(current_color, teams))

      }
   }

   match (red_alliance, blue_alliance) {
      (Some(r), Some(b)) => {
         (r, b)
      }
      (_, _) => {
         panic!("unable to find red or blue alliance");
      }
   }

}
