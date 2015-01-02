use pubsub::Pubsub;
use pubsub::Event;
use components::quantity_component::QuantityComponent;
use ECS;

#[deriving(Copy)]
pub struct QuantitySystem;

impl QuantitySystem {
  pub fn subscribe(pubsub: &mut Pubsub<ECS, String>) {
    pubsub.subscribe("component_quantity".to_string(), QuantitySystem::add_listener);
  }

  fn add_listener(ecs: &mut ECS, payload: String) -> Vec<Event<String>> {
    ecs.quantities.update_quantity(payload.clone(), QuantityComponent::new(payload, 4));
    vec![Event {
      channel: "log".to_string(),
      payload: "Added quantity component!".to_string()
    }]
  }
}
