use pubsub::Pubsub;
use pubsub::Event;
use components::quantity_component::Quantity;
use ECS;

pub struct QuantitySystem;

impl QuantitySystem {
  pub fn subscribe(pubsub: &mut Pubsub<ECS, String>) {
    pubsub.subscribe("component_quantity".to_string(), QuantitySystem::add_listener);
  }

  fn add_listener(ecs: &mut ECS, payload: String) -> Vec<Event<String>> {
    ecs.quantities.update_quantity(payload.clone().as_slice(), Quantity::new(payload, 4));
    println!("Quantities: {}", ecs.quantities)
    Vec::from_elem(1, Event {
      channel: "log".to_string(),
      payload: "Added quantity component!".to_string()
    })
  }
}
