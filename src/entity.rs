use uuid::Uuid;

#[deriving(Show)]
pub struct Entity {
  pub id: String
}

impl Entity {
  pub fn new() -> Entity {
    Entity { id: Uuid::new_v4().to_string() }
  }

  pub fn get_id(&self) -> String {
    self.id.clone()
  }
}
