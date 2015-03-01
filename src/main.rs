#![feature(plugin)]
#![plugin(component_store)]

extern crate pubsub;

use pubsub::Pubsub;
use pubsub::Event;

use systems::color_system::ColorSystem;
use systems::quantity_system::QuantitySystem;
use systems::logging_system::LoggingSystem;
use systems::drawing_system::DrawingSystem;

use components::color_component::ColorComponent;
use components::quantity_component::QuantityComponent;

use std::collections::HashMap;

pub mod systems {
  pub mod color_system;
  pub mod quantity_system;
  pub mod logging_system;
  pub mod drawing_system;
}

pub mod components {
  pub mod component;
  pub mod color_component;
  pub mod quantity_component;
}

component_store!{
  components:
    Quantity/Quantities
    Color
}

fn main() {
  let mut ecs = ECS::new();
  let mut pubsub: Pubsub<ECS, String, String> = Pubsub::new(&mut ecs);

  ColorSystem::subscribe(&mut pubsub);
  QuantitySystem::subscribe(&mut pubsub);
  LoggingSystem::subscribe(&mut pubsub);
  DrawingSystem::subscribe(&mut pubsub);

  pubsub.publish(
    Event {
      channel: "component_quantity".to_string(),
      payload: "123-ABC".to_string()
    }
  );

  pubsub.publish(
    Event {
      channel: "component_color".to_string(),
      payload: "123-ABC".to_string()
    }
  );

  pubsub.publish(
    Event {
      channel: "draw".to_string(),
      payload: "123-ABC".to_string()
    }
  );
}
