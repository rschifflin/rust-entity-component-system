#![feature(phase)]
#[phase(plugin)]
extern crate component_store;

extern crate pubsub;

use pubsub::Pubsub;
use pubsub::Event;

use entity::Entity;

use systems::color_system::ColorSystem;
use systems::quantity_system::QuantitySystem;
use systems::logging_system::LoggingSystem;
use systems::drawing_system::DrawingSystem;

use components::color_component::Color;
use components::quantity_component::Quantity;

use std::collections::HashMap;

pub mod entity;

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
  let mut pubsub: Pubsub<ECS, String> = Pubsub::new(&mut ecs);
  ColorSystem::subscribe(&mut pubsub);
  QuantitySystem::subscribe(&mut pubsub);
  LoggingSystem::subscribe(&mut pubsub);
  DrawingSystem::subscribe(&mut pubsub);

  let e = Entity::new();

  pubsub.publish(
    Event {
      channel: "components".to_string(),
      payload: e.get_id()
    }
  );

  pubsub.publish(
    Event {
      channel: "component_quantity".to_string(),
      payload: e.get_id()
    }
  );

  pubsub.publish(
    Event {
      channel: "component_color".to_string(),
      payload: e.get_id()
    }
  );

  pubsub.publish(
    Event {
      channel: "draw".to_string(),
      payload: e.get_id()
    }
  )
}
