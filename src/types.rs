use std::{num::ParseIntError, collections::HashMap};

use crate::enums::{Action, RClothingLocations, RItemType, RDrinkContainer, RFoodContainer};

pub struct Size {
  pub x: usize,
  pub y: usize,
}
impl Size {
  pub fn new(x: usize, y: usize) -> Self{
      Size {
          x,
          y
      }
  }
}

pub struct GameState<'g> {
  player: Box<You<'g>>,
  entities: Vec<&'g Box<dyn Entity>>,
  items: Vec<& 'g Box<dyn Item>>,
}
impl <'g>GameState<'g> {
  pub fn new() -> Self{
    GameState {
      player: Box::new(You::new()),
      entities: Vec::new(),
      items: Vec::new()
    }
  }
}

pub struct Room {
  size: Size,
  items: Vec<Box<dyn Item>>,
  entities: Vec<Box<dyn Entity>>
}

pub struct You<'y> {
  name: String,
  inv: Vec<&'y Box<dyn Item>>,
  friends: Vec<&'y Box<dyn Entity>>,
  foes: Vec<&'y Box<dyn Entity>>,
  total_step: u16,
  wait_time: u8
}
impl<'y> You<'y> {

  pub fn new() -> Self{
    You {
      name: String::new(),
      inv: Vec::new(),
      friends: Vec::new(),
      foes: Vec::new(),
      total_step: 0,
      wait_time: 0
    }
  }

  pub fn pick_up(&mut self, item: &'y  Box<dyn Item>) {
    self.inv.push(item);
  }

  pub fn consequences(&mut self, action: Action) {
    match action {
      Action::Wait => self.wait_time+= 1,
      Action::Walk => {
        self.wait_time = 0;
        self.total_step += 1;
      }
    }
  }


}

#[derive(Clone)]
pub struct RItem {
  pub id: u16,
  pub name: String,
  pub kind: RItemType
}
impl RItem {
  pub fn new(id: u16, name: String, kind: RItemType) -> Self {
    RItem {
      id,
      name,
      kind
    }
  }

  pub fn any_item(item_type_str: &str, item_desc_str: String) -> Result<Box<dyn Item>, String> {
    type KeyVal<'a> = (&'a str, &'a str);
    let kind: RItemType = match RItemType::try_from(item_type_str) {
      Ok(i_type) => i_type,
      Err(_) => return Err(String::from("Could not parse item type!!!"))
    };

    let kvs: Vec<&str> = item_desc_str.as_str().split(",").collect();
    let mut attr_hash: HashMap<&str, &str> = HashMap::new();
    attr_hash.insert("kind", item_type_str);
    for kv in kvs {
      let pair = match kv.split_once("=") {
        Some(pair) => {
          (pair.0.trim(), pair.1.trim().trim_matches(|c| c == '\"' || c== ' '))
        }
        None => return Err(String::from("Invalid key/value pair!!!"))
      };
      attr_hash.insert( pair.0, pair.1);
    }
    let base_item = match RItem::try_from(&attr_hash) {
      Ok(base) => base,
      Err(_) => return Err(String::from("Could not create base Item!!!"))
    };

    match kind {
      RItemType::Stuff => {
        return Ok(Box::new(base_item))
      },
      RItemType::Drink => {
        let mut item_box = match RDrink::try_from(&attr_hash) {
            Ok(drink) => Box::new(drink),
            Err(_) => return Err(String::from("Could not parse a drink!"))
        };
        item_box.item = Box::new(base_item);
        Ok(item_box)
      },
       RItemType::Food => {
        match RFood::try_from(attr_hash) {
          Ok(food) => return Ok(Box::new(food)),
          Err(_) => return Err(String::from("Could not parse a food!"))
        }
      },
      RItemType::Clothing => {
        match RClothing::try_from(attr_hash) {
          Ok(clothing) => return Ok(Box::new(clothing)),
          Err(_) => return Err(String::from("Could not parse a clothing!"))
        }
      },
      RItemType::Weapon => {
        match RWeapon::try_from(attr_hash) {
          Ok(weapon) => return Ok(Box::new(weapon)),
          Err(_) => return Err(String::from("Could not parse a food!"))
        }
      },
      RItemType::Armor => {
        match RArmor::try_from(attr_hash) {
          Ok(armor) => return Ok(Box::new(armor)),
          Err(_) => return Err(String::from("Could not parse a food!"))
        }
      }
    }
  }
}
impl Item for RItem {
  fn get_name(&self) -> String {
    self.name.clone()
  }
}
impl TryFrom<&HashMap<&str, &str>> for RItem{
  type Error = String;
  fn try_from(map: &HashMap<&str, &str>) -> Result<Self, Self::Error>{
    let id: u16 = u16::MAX;
    let name = String::new();
    let kind = RItemType::Stuff;
    for attr in ["id", "name", "kind"] {
      if !map.contains_key(attr) {
        return Err(String::from("{0} key is missing!")) 
      }
    }
    let id = match map.get("id").unwrap().parse::<u16>(){
      Ok(val) => val,
      Err(_) => return Err(String::from("Error parsing ID!!!!"))
    };
    let name = match map.get("name") {
      Some(str_ptr) => String::from(*str_ptr),
      None => return Err(String::from("Error parsing Name!!!"))
    };
    let kind: RItemType  = match map.get("kind") {
      Some(val) => {
        match RItemType::try_from(*val) {
          Ok(i_type) => i_type,
          Err(_) => return Err(String::from("Error parsing ItemType!!!"))
        }
      }
      None => return Err(String::from("Error parsing ItemType!!!"))
    };
    Ok(Self { id, name, kind})
  }
}

pub struct RDrink {
  pub item: Box<RItem>,
  pub container: RDrinkContainer,
  pub flavor: String,
  pub sips: u8
}
impl RDrink {
  fn new(item: Box<RItem>, container: RDrinkContainer, flavor: String, sips: u8) -> Self {
    RDrink {
      item,
      container,
      flavor,
      sips
    }
  }
  fn sip(&mut self) {
    self.sips -= 1;
  }

  fn chug(&self) {

  }

  fn throw(&self) {

  }
}
impl Item for RDrink {
  fn get_name(&self) -> String{
    let item_name = self.item.name.clone();
    println!("x");
    item_name
  }
}
impl TryFrom<&HashMap::<&str, &str>> for RDrink{
  type Error = String;
  fn try_from(map: &HashMap::<&str, &str>) -> Result<Self, Self::Error>{
    let base_item = RItem {id: 0, name: String::from(""), kind: RItemType::Stuff};
    let container: RDrinkContainer = match map.get("container").unwrap().parse::<u8>() {
        Ok(int) => {
          match RDrinkContainer::try_from(int) {
            Ok(cont) => cont,
            Err(_) => return Err(String::from("Could not parse container value!!"))
          }
        },
        Err(_) => return Err(String::from("Could not parse container value!!!"))
    };

    let flavor = match map.get("flavor") {
      Some(flavor_str) => String::from(*flavor_str),
      None => return Err(String::from("Could not parse drink from map!")) 
    };
    
    let sips = match map.get("sips").unwrap().parse::<u8>() {
      Ok(val) => val,
      Err(_) => return Err(String::from("Could not parse sips from map!"))
    };
    Ok(Self::new(Box::new(base_item), container, flavor, sips))
  }
}

pub struct RFood {
  item: Box<RItem>,
  container: RFoodContainer,
  flavor: String,
  nibbles: u8
}
impl RFood {
  fn new(item: Box<RItem>, container: RFoodContainer, flavor: String, nibbles: u8) -> Self{
    Self {
      item,
      container,
      flavor,
      nibbles
    }
  }
  fn nibble(&self) {

  }

  fn eat(&self) {

  }

  fn throw(&self) {

  }
}
impl Item for RFood {}
impl TryFrom<HashMap::<&str, &str>> for RFood{
  type Error = String;
  fn try_from(map: HashMap::<&str, &str>) -> Result<Self, Self::Error>{
    let base_item = RItem {id: 0, name: String::from(""), kind: RItemType::Stuff};
    let container: RFoodContainer = match map.get("container").unwrap().parse::<u8>() {
        Ok(int) => {
          match RFoodContainer::try_from(int) {
            Ok(cont) => cont,
            Err(_) => return Err(String::from("Could not parse container value!!"))
          }
        },
        Err(_) => return Err(String::from("Could not parse container value!!!"))
    };

    let flavor = match map.get("flavor") {
      Some(flavor_str) => String::from(*flavor_str),
      None => return Err(String::from("Could not parse drink from map!")) 
    };
    
    let nibbles = match map.get("nibbles").unwrap().parse::<u8>() {
      Ok(val) => val,
      Err(_) => return Err(String::from("Could not parse sips from map!"))
    };
    Ok(Self::new(Box::new(base_item), container, flavor, nibbles))
  }
}

pub struct RClothing {
  item: Box<RItem>,
  location: RClothingLocations,
}
impl RClothing {
  fn new(item: Box<RItem>, location: RClothingLocations) -> Self {
    Self {
      item,
      location
    }
  }

  fn wear(&self) {

  }

  pub fn remove(&self) {

  }
}
impl Item for RClothing{}
impl TryFrom<HashMap::<&str, &str>> for RClothing{
  type Error = String;
  fn try_from(map: HashMap::<&str, &str>) -> Result<Self, Self::Error>{
    let base_item = RItem {id: 0, name: String::from(""), kind: RItemType::Stuff};
    let location: RClothingLocations = match map.get("location").unwrap().parse::<u8>() {
      Ok(int) => {
        match RClothingLocations::try_from(int) {
          Ok(cont) => cont,
          Err(_) => return Err(String::from("Could not parse container value!!"))
        }
      },
      Err(_) => return Err(String::from("Could not parse container value!!!"))
  };
    Ok(Self::new(Box::new(base_item), location))
  }
}

pub struct RWeapon {
  item_data: Box<RItem>,
  damage: u8,
  durability: u8,
  accuracy: u8
}

impl RWeapon {
  pub fn new(item_data: Box<RItem>, damage: u8, durability: u8, accuracy: u8) -> Self {
    Self {
      item_data,
      damage,
      durability,
      accuracy
    }
  }
  fn attack(&self) {

  }
  fn wield(&self) {

  }
  fn sheathe(&self) {

  }
}
impl Item for RWeapon {
  fn get_name(&self) -> String {
    self.item_data.name.clone()
  }

}
impl TryFrom<HashMap::<&str, &str>> for RWeapon{
  type Error = String;
  fn try_from(map: HashMap::<&str, &str>) -> Result<Self, Self::Error>{
    let base_item = RItem {id: 0, name: String::from(""), kind: RItemType::Stuff};
    let damage = match map.get("damage").unwrap().parse::<u8>() {
      Ok(val) => val,
      Err(_) => return Err(String::from("Could not parse sips from map!"))
    };

    let durability = match map.get("durability").unwrap().parse::<u8>() {
      Ok(val) => val,
      Err(_) => return Err(String::from("Could not parse sips from map!"))
    };
    
    let accuracy = match map.get("accuracy").unwrap().parse::<u8>() {
      Ok(val) => val,
      Err(_) => return Err(String::from("Could not parse sips from map!"))
    };
    Ok(Self::new(Box::new(base_item), damage, durability, accuracy))
  }
}

pub struct RArmor {
  item: Box<RItem>,
  slot: RClothingLocations,
  defense: u8,
  durability: u8
}
impl RArmor {
  fn new(item: Box<RItem>, slot: RClothingLocations, defense: u8, durability: u8) -> Self{
    Self {
      item,
      slot,
      defense,
      durability
    }
  }
  fn wear(&self) {

  }

  fn remove(&self) {

  }
}
impl Item for RArmor{}
impl TryFrom<HashMap::<&str, &str>> for RArmor{
  type Error = String;
  fn try_from(map: HashMap::<&str, &str>) -> Result<Self, Self::Error>{
    let base_item = RItem {id: 0, name: String::from(""), kind: RItemType::Stuff};
    let location: RClothingLocations = match map.get("location").unwrap().parse::<u8>() {
      Ok(int) => {
        match RClothingLocations::try_from(int) {
          Ok(cont) => cont,
          Err(_) => return Err(String::from("Could not parse container value!!"))
        }
      },
      Err(_) => return Err(String::from("Could not parse container value!!!"))
    };
    
    let defense = match map.get("defense").unwrap().parse::<u8>() {
      Ok(val) => val,
      Err(_) => return Err(String::from("Could not parse sips from map!"))
    };

    let durability = match map.get("durability").unwrap().parse::<u8>() {
      Ok(val) => val,
      Err(_) => return Err(String::from("Could not parse sips from map!"))
    };
    Ok(Self::new(Box::new(base_item), location, defense, durability))
  }
}

pub trait Entity {
  fn get_name(&self) -> String{
    String::new()
  }

  fn go(&self) {

  }

  fn search(&self) {

  }
}

pub trait Item{
  fn get_name(&self) -> String {
    String::new()
  }

  fn drop(&self) {

  }
    
}
