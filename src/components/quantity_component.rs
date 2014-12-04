#[deriving(Show, Clone)]
pub struct Quantity {
  eid: String,
  quantity: int
}

impl Quantity {
  pub fn new(eid: String, n: int) -> Quantity {
    Quantity {
      eid: eid,
      quantity: n
    }
  }

  pub fn entity(&self) -> String { self.eid.clone() }
}
