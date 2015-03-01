#[derive(Debug, Clone)]
pub struct QuantityComponent {
  eid: String,
  quantity: isize
}

impl QuantityComponent {
  pub fn new(eid: String, n: isize) -> QuantityComponent {
    QuantityComponent {
      eid: eid,
      quantity: n
    }
  }

  pub fn entity(&self) -> String { self.eid.clone() }
}
