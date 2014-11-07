use pubsub::Pubsub;
use pubsub::Event;
use component_store::ComponentStore;
use components::quantity_component::QuantityComponent;

pub struct QuantitySystem;

impl QuantitySystem {
  pub fn subscribe(pubsub: &mut Pubsub<ComponentStore, String>) {
    pubsub.subscribe("component_quantity".to_string(), QuantitySystem::add_listener);
  }

  fn add_listener(cs: &mut ComponentStore, payload: String) -> Vec<Event<String>> {
    cs.add_quantity_component(QuantityComponent::new(payload, 4));
    Vec::from_elem(1, Event {
      channel: "log".to_string(),
      payload: "Added quantity component!".to_string()
    })
  }
}
