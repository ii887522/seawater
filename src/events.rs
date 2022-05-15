use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub struct ComponentPutEvent<'a, T: ?Sized> {
  pub entity_id: usize,
  pub component: &'a T,
}

#[derive(Copy, Clone, Debug)]
pub struct ComponentRemovedEvent<T: ?Sized> {
  pub entity_id: usize,
  pub phantom: PhantomData<T>,
}
