use std::fmt::Debug;

pub trait System: Debug {
  fn step(&mut self, dt: f64);
}
