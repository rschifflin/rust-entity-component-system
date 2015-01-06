use pubsub::Pubsub;
use pubsub::Event;
use components::color_component::ColorComponent;
use ECS;

#[derive(Copy)]
pub struct ColorSystem;

impl ColorSystem {
  pub fn subscribe(pubsub: &mut Pubsub<ECS, String, String>) {
    pubsub.subscribe("component_color".to_string(), ColorSystem::add_listener);
  }

  fn add_listener(ecs: &mut ECS, payload: String) -> Vec<Event<String, String>> {
    ecs.colors.update_color(payload.clone(), ColorComponent::red(payload));
    vec![
      Event {
        channel: "log".to_string(),
        payload: "Added color component!".to_string()
      }
    ]
  }
}
