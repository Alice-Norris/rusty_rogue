use crate::types::{RItem,Item};
use crate::enums::RItemType;

pub fn create_stuff() -> Vec<Box<dyn Item>> {
  vec![
    Box::new(RItem {id: 0, name: String::from("cube"), kind: RItemType::Stuff}),
    Box::new(RItem {id: 1, name: String::from("orb"), kind: RItemType::Stuff}),
  ]
}
