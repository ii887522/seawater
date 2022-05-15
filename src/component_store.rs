use crate::Component;
use std::fmt::Debug;

pub(super) trait ComponentStore: Debug {
  fn remove(&mut self, entity_id: usize);
  fn is_empty(&self) -> bool;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(super) enum PutResult {
  Added,
  Modified,
}

#[derive(Debug)]
pub(super) struct Impl<T: Component> {
  len: usize,
  components: Vec<Option<T>>,
}

impl<T: Component> Impl<T> {
  pub(super) fn new() -> Self {
    Self {
      len: 0,
      components: vec![],
    }
  }

  pub(super) fn put(&mut self, entity_id: usize, component: T) -> PutResult {
    if entity_id >= self.components.len() {
      self.add_outside(entity_id, component)
    } else {
      self.put_inside(entity_id, component)
    }
  }

  fn add_outside(&mut self, entity_id: usize, component: T) -> PutResult {
    self.components.resize_with(entity_id, || None);
    self.components.push(Some(component));
    self.len += 1;
    PutResult::Added
  }

  fn put_inside(&mut self, entity_id: usize, component: T) -> PutResult {
    let is_prev_component_exist = self.components[entity_id].is_some();
    self.components[entity_id] = Some(component);
    if is_prev_component_exist {
      PutResult::Modified
    } else {
      self.len += 1;
      PutResult::Added
    }
  }

  pub(super) fn get(&self, entity_id: usize) -> &Option<T> {
    &self.components[entity_id]
  }

  pub(super) fn get_all(&self) -> &[Option<T>] {
    &self.components
  }
}

impl<T: Component> ComponentStore for Impl<T> {
  fn remove(&mut self, entity_id: usize) {
    self.components[entity_id] = None;
    self.len -= 1;
  }

  fn is_empty(&self) -> bool {
    self.len == 0
  }
}
