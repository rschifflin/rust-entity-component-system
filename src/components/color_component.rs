#[deriving(Show, Clone)]
enum EColor {
  Red,
  Green,
  Blue
}

#[deriving(Show, Clone)]
pub struct Color {
  eid: String,
  color: EColor
}

impl Color {
  pub fn red(eid: String) -> Color {
    Color {
      eid: eid,
      color: EColor::Red
    }
  }

  pub fn entity(&self) -> String { self.eid.clone() }
}
