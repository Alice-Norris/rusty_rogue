#[derive(Clone)]
pub enum RItemType {
  Stuff,
  Drink,
  Food,
  Clothing,
  Weapon,
  Armor,
}
impl TryFrom<&str> for RItemType {
  type Error = String;

  fn try_from(val: &str) -> Result<Self, Self::Error>{  
    match val {
      "stuff" => Ok(RItemType::Stuff),
      "drink" => Ok(RItemType::Drink),
      "food" => Ok(RItemType::Food),
      "clothing" => Ok(RItemType::Clothing),
      "weapon" => Ok(RItemType::Weapon),
      "armor" => Ok(RItemType::Armor),
      _ => Err(String::from("Invalid item type!!!"))
    }
  }
}
pub enum RDrinkContainer {
  PlasticBottle,
  GlassBottle,
  Cup,
  Vial
}
impl TryFrom<u8> for RDrinkContainer {
  type Error = String;

  fn try_from(val: u8) -> Result<Self, Self::Error> {
      match val {
        0 => return Ok(RDrinkContainer::PlasticBottle),
        1 => return Ok(RDrinkContainer::GlassBottle),
        2 => return Ok(RDrinkContainer::Cup),
        3 => return Ok(RDrinkContainer::Vial),
        _ => return Err(String::from("Invalid container!!!"))
      }
  }
}
pub enum RFoodContainer {
  Wrapper,
  Container,
  Bag,
  Box
}
impl TryFrom<u8> for RFoodContainer {
  type Error = String;

  fn try_from(val: u8) -> Result<Self, Self::Error> {
      match val {
        0 => return Ok(RFoodContainer::Wrapper),
        1 => return Ok(RFoodContainer::Container),
        2 => return Ok(RFoodContainer::Bag),
        3 => return Ok(RFoodContainer::Box),
        _ => return Err(String::from("Invalid container!!!"))
      }
  }
}
pub enum RClothingLocations {
  Outer,
  Head,
  Body,
  Legs,
  Arms,
  Hands,
  Feet
}
impl TryFrom<u8> for RClothingLocations {
  type Error = String;

  fn try_from(val: u8) -> Result<Self, Self::Error> {
      match val {
        0 => return Ok(RClothingLocations::Outer),
        1 => return Ok(RClothingLocations::Head),
        2 => return Ok(RClothingLocations::Body),
        3 => return Ok(RClothingLocations::Legs),
        4 => return Ok(RClothingLocations::Arms),
        5 => return Ok(RClothingLocations::Hands),
        6 => return Ok(RClothingLocations::Feet),
        _ => return Err(String::from("Invalid container!!!"))
      }
  }
}
pub enum REntityType {
  Ghost,
  Doll,
  Witch,
  Alien,
  Extradimensional,
  Squid,
  CatGirl
}

pub enum Action {
  Walk,
  Wait,
}
