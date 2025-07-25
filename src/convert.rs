use crate::raw_info::{RawInfo, RawAlliance};
use crate::info::{team, alliance};

fn get_color_from_string(buff: String) -> Result<alliance::AllianceColor, String> {
   match buff.to_lowercase().as_str() {
      "blue" => Result::Ok(alliance::AllianceColor::BLUE),
      "red" => Result::Ok(alliance::AllianceColor::RED),
      _ => Result::Err(format!("unknow color {} found", buff)) 
   }
}

fn convert(raw: RawInfo) {
   let mut last_color: Option<alliance::AllianceColor> = Option::None;
   for alliance in raw.alliances {
      match get_color_from_string(alliance.alliance) {
         Ok(color) => {
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
   }
}
