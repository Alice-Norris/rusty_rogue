use crate::types::{RItem, RItemType, Weapon, Item};

pub fn create_item_data() -> Vec<Box<dyn Item>> {
  vec![
    Box::new(RItem {id: 0, name: String::from("cube"), kind: RItemType::Stuff, icon: '□'}),
    Box::new(RItem {id: 1, name: String::from("orb"), kind: RItemType::Stuff, icon: '◍'})
  ]
}