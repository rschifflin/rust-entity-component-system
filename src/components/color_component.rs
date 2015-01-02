#[deriving(Show, Clone)]
enum Color {
  Red,
  Green,
  Blue
}

#[deriving(Show, Clone)]
pub struct ColorComponent {
  eid: String,
  color: Color
}

impl ColorComponent {
  pub fn red(eid: String) -> ColorComponent {
    ColorComponent {
      eid: eid,
      color: Color::Red
    }
  }

  pub fn entity(&self) -> String { self.eid.clone() }
}
