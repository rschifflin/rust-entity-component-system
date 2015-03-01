use pubsub::Pubsub;
use pubsub::Event;
use ECS;

#[derive(Copy)]
pub struct DrawingSystem;

impl DrawingSystem {
  pub fn subscribe(pubsub: &mut Pubsub<ECS, String, String>) {
    pubsub.subscribe("draw".to_string(), DrawingSystem::add_listener);
  }

  fn add_listener(ecs: &mut ECS, payload: String) -> Vec<Event<String, String>> {
    let color = ecs.colors.find_color(&payload);
    let quantity = ecs.quantities.find_quantity(&payload);
    println!("Drawing with Color {:?} and Quantity {:?}", color, quantity);
    Vec::new()
  }
}

