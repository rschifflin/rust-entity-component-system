use pubsub::Pubsub;
use pubsub::Event;
use component_store::ComponentStore;
use components::color_component::ColorComponent;

pub struct ColorSystem;

impl ColorSystem {
  pub fn subscribe(pubsub: &mut Pubsub<ComponentStore, String>) {
    pubsub.subscribe("component_color".to_string(), ColorSystem::add_listener);
  }

  fn add_listener(cs: &mut ComponentStore, payload: String) -> Vec<Event<String>> {
    cs.add_color_component(ColorComponent::red(payload));
    Vec::from_elem(1, Event {
      channel: "log".to_string(),
      payload: "Added color component!".to_string()
    })
  }
}
