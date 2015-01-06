#[derive(Show, Rand)]
pub struct Entity {
  pub id: String
}

impl Entity {
  pub fn get_id(&self) -> String {
    self.id.clone()
  }
}
