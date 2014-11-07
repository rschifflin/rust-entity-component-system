use pubsub::Pubsub;
use pubsub::Event;
use component_store::ComponentStore;

pub struct DrawingSystem;

impl DrawingSystem {
  pub fn subscribe(pubsub: &mut Pubsub<ComponentStore, String>) {
    pubsub.subscribe("draw".to_string(), DrawingSystem::add_listener);
  }

  fn add_listener(cs: &mut ComponentStore, payload: String) -> Vec<Event<String>> {
    let color = cs.get_color_component(payload.clone());
    let quantity = cs.get_quantity_component(payload.clone());
    println!("Drawing with Color {} and Quantity {}", color, quantity);
    Vec::new()
  }
}

