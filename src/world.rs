use crate::{
  component_store::{self, PutResult},
  prelude::*,
  ComponentPutEvent, ComponentRemovedEvent, ComponentStore,
};

use iron_ingot::IDManager;

use std::{
  any::{Any, TypeId},
  collections::HashMap,
  fmt::{self, Debug, Formatter},
  marker::PhantomData,
  mem::transmute,
};

pub use seawater_macro::find_archetype;

type OnEvent = dyn FnMut(&dyn Any);

#[derive(Default)]
pub struct World {
  entity_id_manager: IDManager,
  component_store_map: HashMap<TypeId, Box<dyn ComponentStore>>,
  component_counts: Vec<usize>,
  systems: Vec<Box<dyn System>>,
  entity_id_map: HashMap<String, usize>,
  on_event_map: HashMap<TypeId, Vec<Box<OnEvent>>>,
}

impl Debug for World {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
    formatter
      .debug_struct("World")
      .field("entity_id_manager", &self.entity_id_manager)
      .field("component_store_map", &self.component_store_map)
      .field("component_counts", &self.component_counts)
      .field("systems", &self.systems)
      .field("entity_id_map", &self.entity_id_map)
      .finish_non_exhaustive()
  }
}

impl World {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn put_component<T: Component + 'static>(&mut self, entity_name: &str, component: T) {
    let entity_id = *self
      .entity_id_map
      .entry(entity_name.to_owned())
      .or_insert_with(|| self.entity_id_manager.next().unwrap());
    let component_store = unsafe {
      transmute::<_, &mut Box<component_store::Impl<_>>>(
        self
          .component_store_map
          .entry(TypeId::of::<T>())
          .or_insert_with(|| Box::new(component_store::Impl::<T>::new())),
      )
    };
    if component_store.put(entity_id, component) == PutResult::Added {
      self.increment_component_count(entity_id);
    }
    self.emit(ComponentPutEvent {
      entity_id,
      component: component_store.get(entity_id).as_ref().unwrap(),
    });
  }

  fn increment_component_count(&mut self, entity_id: usize) {
    if entity_id == self.component_counts.len() {
      self.component_counts.push(1);
    } else {
      self.component_counts[entity_id] += 1;
    }
  }

  pub fn get_component<T: Component + 'static>(&self, entity_name: &str) -> &Option<T> {
    if let Some(component_store) = self.component_store_map.get(&TypeId::of::<T>()) {
      if let Some(&entity_id) = self.entity_id_map.get(entity_name) {
        return unsafe { transmute::<_, &Box<component_store::Impl<_>>>(component_store) }
          .get(entity_id);
      }
    }
    &None
  }

  pub fn get_components<T: Component + 'static>(&self) -> &[Option<T>] {
    if let Some(component_store) = self.component_store_map.get(&TypeId::of::<T>()) {
      unsafe { transmute::<_, &Box<component_store::Impl<_>>>(component_store) }.get_all()
    } else {
      &[]
    }
  }

  pub fn remove_component<T: Component + 'static>(&mut self, entity_name: &str) {
    if let Some(component_store) = self.component_store_map.get_mut(&TypeId::of::<T>()) {
      if let Some(&entity_id) = self.entity_id_map.get(entity_name) {
        component_store.remove(entity_id);
        if component_store.is_empty() {
          self.component_store_map.remove(&TypeId::of::<T>());
        }
        self.component_counts[entity_id] -= 1;
        if self.component_counts[entity_id] == 0 {
          self.entity_id_map.remove(entity_name);
          self.entity_id_manager.free(entity_id).unwrap();
        }
        self.emit(ComponentRemovedEvent::<T> {
          entity_id,
          phantom: PhantomData,
        });
      }
    }
  }

  pub fn add_system(&mut self, system: impl System + 'static) {
    self.systems.push(Box::new(system));
  }

  pub fn emit<E: 'static>(&mut self, event: E) {
    if let Some(on_events) = self.on_event_map.get_mut(&TypeId::of::<E>()) {
      for on_event in on_events {
        on_event(&event);
      }
    }
  }

  pub fn on<E: 'static>(&mut self, mut on_event: impl FnMut(&E) + 'static) {
    self
      .on_event_map
      .entry(TypeId::of::<E>())
      .or_insert(vec![])
      .push(Box::new(move |event| {
        on_event(unsafe { &*(event as *const dyn Any as *const E) })
      }));
  }

  pub fn step(&mut self, dt: f32) {
    debug_assert!(dt >= 0.0, "dt must be a positive number!");
    for system in &mut self.systems {
      system.step(dt);
    }
  }
}
