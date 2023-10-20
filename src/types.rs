#[derive(Clone)]
pub enum RItemType {
  Weapon,
  Drink,
  Food,
  Armor,
  Stuff
}

pub enum Action {
  Walk,
  Wait,
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

  pub fn new(name: String) -> Self{
    You {
      name,
      inv: Vec::new(),
      friends: Vec::new(),
      foes: Vec::new(),
      total_step: 0,
      wait_time: 0
    }
  }

  pub fn pick_up(&mut self, item: &'y Box<dyn Item>) {
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

pub struct Room {
  size: Size,
  items: Vec<Box<dyn Item>>,
  entities: Vec<Box<dyn Entity>>
}

#[derive(Clone)]
pub struct RItem {
  pub id: u16,
  pub name: String,
  pub kind: RItemType,
  pub icon: char
}
impl Item for RItem {
  fn get_name(&self) -> String {
    self.name.clone()
  }
}

pub trait Item{
  fn get_name(&self) -> String {
    String::new()
  }

  fn drop(&self) {

  }
  fn get(&self) {
    
  }
}

pub trait Weapon<Item> {
  fn attack() {

  }
  fn wield() {

  }
  fn sheathe() {

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