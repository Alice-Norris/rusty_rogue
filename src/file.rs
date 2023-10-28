use std::{env, fs::{self, ReadDir}, path::Path, collections::HashMap};

use crate::types::{Item, RItem, RDrink};
use crate::enums::{RDrinkContainer, RItemType};

pub fn load_items(item_map: &mut HashMap<u16, Box<dyn Item>>) {
    let path = Path::new("./target/debug/res/items.toml");
    let items: Vec<Box<dyn Item>> = Vec::new();
    let file_res = fs::read_to_string(path);
    if file_res.is_ok() {
        let mut file_text = file_res.unwrap();
        file_text.retain(|c| c != '\n' && c != '\n');
        let tables: Vec<&str> = file_text.split_inclusive(']').collect();
        for table_str in tables {
          parse_items(String::from(table_str), item_map);
        }
    }
}

fn parse_items(table_str: String, item_map: &mut HashMap<u16, Box<dyn Item>>) {
  let mut attr_hash: HashMap<&str, &str> = HashMap::new();
  let (item_type_str, item_str) = table_str.as_str().split_once("=").unwrap();
  let l_braces: Vec<(usize, &str)> = item_str.match_indices("{").collect();
  let r_braces: Vec<(usize, &str)> = item_str.match_indices("}").collect();
  for i in 0..l_braces.len() {
    let l_index = l_braces[i].0;
    let r_index = r_braces[i].0;
    let item_str: String = String::from(item_str.get(l_index..r_index).unwrap().trim_matches(|c| c == '{' || c == '}'));
    let new_item = RItem::new(0, String::from("test"), RItemType::Stuff);
    item_map.insert(i as u16, new_item);
  }
}