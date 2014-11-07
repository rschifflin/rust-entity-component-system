use components::color_component::ColorComponent;
use components::quantity_component::QuantityComponent;

pub struct ComponentStore {
  pub color_components: Vec<ColorComponent>,
  pub quantity_components: Vec<QuantityComponent>
}

impl ComponentStore {
  pub fn new() -> ComponentStore {
    ComponentStore {
      color_components: Vec::new(),
      quantity_components: Vec::new()
    }
  }

  pub fn get_color_components(self) -> Vec<ColorComponent> {
    self.color_components
  }

  pub fn get_color_component(&self, entity: String) -> Option<ColorComponent> {
    let component_ref = self.color_components.iter().find(|e| -> bool { e.entity() == entity });
    component_ref.map(|r| r.clone())
  }

  pub fn add_color_component(&mut self, c: ColorComponent) {
    self.color_components.push(c);
  }

  pub fn get_quantity_components(self) -> Vec<QuantityComponent> {
    self.quantity_components
  }

  pub fn get_quantity_component(&self, entity: String) -> Option<QuantityComponent> {
    let component_ref = self.quantity_components.iter().find(|e| -> bool { e.entity() == entity });
    component_ref.map(|r| r.clone())
  }

  pub fn add_quantity_component(&mut self, q: QuantityComponent) {
    self.quantity_components.push(q);
  }
}
