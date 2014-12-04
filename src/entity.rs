use std::rand;

#[deriving(Show)]
pub struct Entity {
  pub id: String
}

impl Entity {
  pub fn new() -> Entity {
    Entity { id: rand::random::<uint>().to_string() }
  }

  pub fn get_id(&self) -> String {
    self.id.clone()
  }
}
