use pubsub::Pubsub;
use pubsub::Event;
use ECS;

#[deriving(Copy)]
pub struct LoggingSystem;

impl LoggingSystem {
  pub fn subscribe(pubsub: &mut Pubsub<ECS, String>) {
    pubsub.subscribe("log".to_string(), LoggingSystem::add_listener);
  }

  #[allow(unused_variables)]
  fn add_listener(ecs: &mut ECS, payload: String) -> Vec<Event<String>> {
    println!("Logging: {}", payload);
    Vec::new()
  }
}

