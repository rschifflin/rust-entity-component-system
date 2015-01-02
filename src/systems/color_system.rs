use pubsub::Pubsub;
use pubsub::Event;
use components::color_component::Color;
use ECS;

pub struct ColorSystem;

impl ColorSystem {
  pub fn subscribe(pubsub: &mut Pubsub<ECS, String>) {
    pubsub.subscribe("component_color".to_string(), ColorSystem::add_listener);
  }

  fn add_listener(ecs: &mut ECS, payload: String) -> Vec<Event<String>> {
    ecs.colors.update_color(payload.clone(), Color::red(payload));
    vec![
      Event {
        channel: "log".to_string(),
        payload: "Added color component!".to_string()
      }
    ]
  }
}
