#[deriving(Show, Clone)]
pub struct QuantityComponent {
  eid: String,
  quantity: int
}

impl QuantityComponent {
  pub fn new(eid: String, n: int) -> QuantityComponent {
    QuantityComponent {
      eid: eid,
      quantity: n
    }
  }

  pub fn entity(&self) -> String { self.eid.clone() }
}
