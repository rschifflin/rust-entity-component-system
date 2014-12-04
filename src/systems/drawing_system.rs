use pubsub::Pubsub;
use pubsub::Event;
use ECS;

pub struct DrawingSystem;

impl DrawingSystem {
  pub fn subscribe(pubsub: &mut Pubsub<ECS, String>) {
    pubsub.subscribe("draw".to_string(), DrawingSystem::add_listener);
  }

  fn add_listener(ecs: &mut ECS, payload: String) -> Vec<Event<String>> {
    let color = ecs.colors.find_color(payload.clone().as_slice());
    let quantity = ecs.quantities.find_quantity(payload.clone().as_slice());
    println!("Drawing with Color {} and Quantity {}", color, quantity);
    Vec::new()
  }
}

