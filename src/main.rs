extern crate uuid;
extern crate pubsub;

use pubsub::Pubsub;
use pubsub::Event;

use entity::Entity;
use component_store::ComponentStore;

use systems::color_system::ColorSystem;
use systems::quantity_system::QuantitySystem;
use systems::logging_system::LoggingSystem;
use systems::drawing_system::DrawingSystem;

pub mod entity;
pub mod component_store;

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

fn main() {
  let mut components = ComponentStore::new();
  let mut pubsub: Pubsub<ComponentStore, String> = Pubsub::new(&mut components);

  ColorSystem::subscribe(&mut pubsub);
  QuantitySystem::subscribe(&mut pubsub);
  LoggingSystem::subscribe(&mut pubsub);
  DrawingSystem::subscribe(&mut pubsub);

  let e = Entity::new();

  pubsub.publish(
    Event {
      channel: "component_color".to_string(),
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
      channel: "draw".to_string(),
      payload: e.get_id()
    }
  )
}
