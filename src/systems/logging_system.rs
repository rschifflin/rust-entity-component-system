use pubsub::Pubsub;
use pubsub::Event;
use component_store::ComponentStore;

pub struct LoggingSystem;

impl LoggingSystem {
  pub fn subscribe(pubsub: &mut Pubsub<ComponentStore, String>) {
    pubsub.subscribe("log".to_string(), LoggingSystem::add_listener);
  }

  #[allow(unused_variables)]
  fn add_listener(cs: &mut ComponentStore, payload: String) -> Vec<Event<String>> {
    println!("Logging: {}", payload);
    Vec::new()
  }
}

