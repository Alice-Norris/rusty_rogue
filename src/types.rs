use std::collections::HashMap;

use crate::enums::{Action, RClothingLocations, RItemType, RDrinkContainer, RFoodContainer};

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

// pub struct Room {
//   size: Size,
//   items: Vec<Box<dyn Item>>,
//   entities: Vec<Box<dyn Entity>>
// }

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
}
impl Item for RItem {
  fn get_id(&self) -> u16 {
    self.id
  }

  fn get_name(&self) -> String {
    self.name.clone()
  }

}

pub struct RDrink {
  pub item: RItem,
  pub container: RDrinkContainer,
  pub flavor: String,
  pub sips: u8
}
impl RDrink {
  fn new(item: RItem, container: RDrinkContainer, flavor: String, sips: u8) -> Self {
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

pub struct RFood {
  item: RItem,
  container: RFoodContainer,
  flavor: String,
  nibbles: u8
}
impl RFood {
  fn new(item: RItem, container: RFoodContainer, flavor: String, nibbles: u8) -> Self{
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
impl Item for RItem {
  fn get_id(&self) -> u16 {
    self.id
  }

  fn get_name(&self) -> String {
    self.name.clone()
  }

  fn get_kind(&self) -> RItemType {
    self.kind
  } 
}

pub trait Item {
  fn get_id(&self) -> u16 {
    self.id
  }

  fn get_name(&self) -> String {
    self.name.clone()
  }

  fn get_kind(&self) -> RItemType {
    self.kind
  } 
}