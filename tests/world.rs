use iron_ingot::Shared;
use seawater::prelude::*;
use std::{borrow::Cow, cell::RefCell, rc::Rc};

#[derive(Copy, Clone, Component, Debug, PartialEq)]
struct Position {
  x: f64,
  y: f64,
}

#[derive(Copy, Clone, Component, Debug, PartialEq)]
struct Collider {
  w: f64,
  h: f64,
}

#[derive(Copy, Clone, Component, Debug, PartialEq)]
struct Color {
  r: u8,
  g: u8,
  b: u8,
}

#[test]
fn test_component_related_methods() {
  let mut world = World::new();
  world.put_component("rect", Position { x: 0.0, y: 0.0 });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 0.0, y: 0.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[Some(Position { x: 0.0, y: 0.0 })]
  );
  world.put_component("circle", Position { x: 1.0, y: 1.0 });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 0.0, y: 0.0 })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 0.0, y: 0.0 }),
      Some(Position { x: 1.0, y: 1.0 })
    ]
  );
  world.put_component("player", Position { x: 2.0, y: 2.0 });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 0.0, y: 0.0 })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 0.0, y: 0.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Position { x: 2.0, y: 2.0 })
    ]
  );
  world.remove_component::<Position>("rect");
  assert_eq!(world.get_component::<Position>("rect"), &None);
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      None,
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Position { x: 2.0, y: 2.0 })
    ]
  );
  world.put_component("rect", Position { x: 0.0, y: 0.0 });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 0.0, y: 0.0 })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 0.0, y: 0.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Position { x: 2.0, y: 2.0 })
    ]
  );
  world.put_component("rect", Position { x: 3.0, y: 3.0 });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 3.0, y: 3.0 })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 3.0, y: 3.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Position { x: 2.0, y: 2.0 })
    ]
  );
  world.remove_component::<Position>("circle");
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 3.0, y: 3.0 })
  );
  assert_eq!(world.get_component::<Position>("circle"), &None);
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 3.0, y: 3.0 }),
      None,
      Some(Position { x: 2.0, y: 2.0 })
    ]
  );
  world.put_component("circle", Position { x: 1.0, y: 1.0 });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 3.0, y: 3.0 })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 3.0, y: 3.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Position { x: 2.0, y: 2.0 })
    ]
  );
  world.remove_component::<Position>("player");
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 3.0, y: 3.0 })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(world.get_component::<Position>("player"), &None);
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 3.0, y: 3.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      None
    ]
  );
  world.put_component("player", Position { x: 0.0, y: 0.0 });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 3.0, y: 3.0 })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 0.0, y: 0.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 3.0, y: 3.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Position { x: 0.0, y: 0.0 }),
    ]
  );
  world.remove_component::<Position>("rect");
  world.remove_component::<Position>("circle");
  world.remove_component::<Position>("player");
  assert_eq!(world.get_component::<Position>("rect"), &None);
  assert_eq!(world.get_component::<Position>("circle"), &None);
  assert_eq!(world.get_component::<Position>("player"), &None);
  assert_eq!(world.get_components::<Position>(), &[]);
  world.put_component("rect", Position { x: 0.0, y: 0.0 });
  world.put_component("circle", Position { x: 1.0, y: 1.0 });
  world.put_component("player", Position { x: 2.0, y: 2.0 });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 0.0, y: 0.0 })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 2.0, y: 2.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Position { x: 0.0, y: 0.0 }),
    ]
  );
  world.put_component("rect", Collider { w: 1.0, h: 1.0 });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 0.0, y: 0.0 })
  );
  assert_eq!(
    world.get_component::<Collider>("rect"),
    &Some(Collider { w: 1.0, h: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 2.0, y: 2.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Position { x: 0.0, y: 0.0 }),
    ]
  );
  assert_eq!(
    world.get_components::<Collider>(),
    &[None, None, Some(Collider { w: 1.0, h: 1.0 })]
  );
  world.put_component(
    "rect",
    Color {
      r: 255,
      g: 255,
      b: 255,
    },
  );
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 0.0, y: 0.0 })
  );
  assert_eq!(
    world.get_component::<Collider>("rect"),
    &Some(Collider { w: 1.0, h: 1.0 })
  );
  assert_eq!(
    world.get_component::<Color>("rect"),
    &Some(Color {
      r: 255,
      g: 255,
      b: 255
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 2.0, y: 2.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Position { x: 0.0, y: 0.0 }),
    ]
  );
  assert_eq!(
    world.get_components::<Collider>(),
    &[None, None, Some(Collider { w: 1.0, h: 1.0 })]
  );
  assert_eq!(
    world.get_components::<Color>(),
    &[
      None,
      None,
      Some(Color {
        r: 255,
        g: 255,
        b: 255
      })
    ]
  );
  world.remove_component::<Position>("rect");
  assert_eq!(world.get_component::<Position>("rect"), &None);
  assert_eq!(
    world.get_component::<Collider>("rect"),
    &Some(Collider { w: 1.0, h: 1.0 })
  );
  assert_eq!(
    world.get_component::<Color>("rect"),
    &Some(Color {
      r: 255,
      g: 255,
      b: 255
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(world.get_component::<Position>("rect"), &None);
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 2.0, y: 2.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      None,
    ]
  );
  assert_eq!(
    world.get_components::<Collider>(),
    &[None, None, Some(Collider { w: 1.0, h: 1.0 })]
  );
  assert_eq!(
    world.get_components::<Color>(),
    &[
      None,
      None,
      Some(Color {
        r: 255,
        g: 255,
        b: 255
      })
    ]
  );
  world.put_component("rect", Position { x: 0.0, y: 0.0 });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 0.0, y: 0.0 })
  );
  assert_eq!(
    world.get_component::<Collider>("rect"),
    &Some(Collider { w: 1.0, h: 1.0 })
  );
  assert_eq!(
    world.get_component::<Color>("rect"),
    &Some(Color {
      r: 255,
      g: 255,
      b: 255
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 2.0, y: 2.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Position { x: 0.0, y: 0.0 }),
    ]
  );
  assert_eq!(
    world.get_components::<Collider>(),
    &[None, None, Some(Collider { w: 1.0, h: 1.0 })]
  );
  assert_eq!(
    world.get_components::<Color>(),
    &[
      None,
      None,
      Some(Color {
        r: 255,
        g: 255,
        b: 255
      })
    ]
  );
  world.remove_component::<Collider>("rect");
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 0.0, y: 0.0 })
  );
  assert_eq!(world.get_component::<Collider>("rect"), &None);
  assert_eq!(
    world.get_component::<Color>("rect"),
    &Some(Color {
      r: 255,
      g: 255,
      b: 255
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 2.0, y: 2.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Position { x: 0.0, y: 0.0 }),
    ]
  );
  assert_eq!(world.get_components::<Collider>(), &[]);
  assert_eq!(
    world.get_components::<Color>(),
    &[
      None,
      None,
      Some(Color {
        r: 255,
        g: 255,
        b: 255
      })
    ]
  );
  world.put_component("rect", Collider { w: 1.0, h: 1.0 });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 0.0, y: 0.0 })
  );
  assert_eq!(
    world.get_component::<Collider>("rect"),
    &Some(Collider { w: 1.0, h: 1.0 })
  );
  assert_eq!(
    world.get_component::<Color>("rect"),
    &Some(Color {
      r: 255,
      g: 255,
      b: 255
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 2.0, y: 2.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Position { x: 0.0, y: 0.0 }),
    ]
  );
  assert_eq!(
    world.get_components::<Collider>(),
    &[None, None, Some(Collider { w: 1.0, h: 1.0 })]
  );
  assert_eq!(
    world.get_components::<Color>(),
    &[
      None,
      None,
      Some(Color {
        r: 255,
        g: 255,
        b: 255
      })
    ]
  );
  world.remove_component::<Color>("rect");
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 0.0, y: 0.0 })
  );
  assert_eq!(
    world.get_component::<Collider>("rect"),
    &Some(Collider { w: 1.0, h: 1.0 })
  );
  assert_eq!(world.get_component::<Color>("rect"), &None);
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 2.0, y: 2.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Position { x: 0.0, y: 0.0 }),
    ]
  );
  assert_eq!(
    world.get_components::<Collider>(),
    &[None, None, Some(Collider { w: 1.0, h: 1.0 })]
  );
  assert_eq!(world.get_components::<Color>(), &[]);
  world.put_component(
    "rect",
    Color {
      r: 255,
      g: 255,
      b: 255,
    },
  );
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 0.0, y: 0.0 })
  );
  assert_eq!(
    world.get_component::<Collider>("rect"),
    &Some(Collider { w: 1.0, h: 1.0 })
  );
  assert_eq!(
    world.get_component::<Color>("rect"),
    &Some(Color {
      r: 255,
      g: 255,
      b: 255
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 2.0, y: 2.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Position { x: 0.0, y: 0.0 }),
    ]
  );
  assert_eq!(
    world.get_components::<Collider>(),
    &[None, None, Some(Collider { w: 1.0, h: 1.0 })]
  );
  assert_eq!(
    world.get_components::<Color>(),
    &[
      None,
      None,
      Some(Color {
        r: 255,
        g: 255,
        b: 255
      })
    ]
  );
  world.remove_component::<Position>("rect");
  world.remove_component::<Collider>("rect");
  world.remove_component::<Color>("rect");
  assert_eq!(world.get_component::<Position>("rect"), &None);
  assert_eq!(world.get_component::<Collider>("rect"), &None);
  assert_eq!(world.get_component::<Color>("rect"), &None);
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 2.0, y: 2.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      None,
    ]
  );
  assert_eq!(world.get_components::<Collider>(), &[]);
  assert_eq!(world.get_components::<Color>(), &[]);
  world.remove_component::<Position>("rect");
  world.remove_component::<Collider>("rect");
  world.remove_component::<Color>("rect");
  world.remove_component::<Position>("enemy");
  world.remove_component::<Collider>("enemy");
  world.remove_component::<Color>("enemy");
  assert_eq!(world.get_component::<Position>("rect"), &None);
  assert_eq!(world.get_component::<Collider>("rect"), &None);
  assert_eq!(world.get_component::<Color>("rect"), &None);
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 2.0, y: 2.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      None,
    ]
  );
  assert_eq!(world.get_components::<Collider>(), &[]);
  assert_eq!(world.get_components::<Color>(), &[]);
  world.put_component("rect", Position { x: 0.0, y: 0.0 });
  world.put_component("rect", Collider { w: 1.0, h: 1.0 });
  world.put_component(
    "rect",
    Color {
      r: 255,
      g: 255,
      b: 255,
    },
  );
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some(Position { x: 0.0, y: 0.0 })
  );
  assert_eq!(
    world.get_component::<Collider>("rect"),
    &Some(Collider { w: 1.0, h: 1.0 })
  );
  assert_eq!(
    world.get_component::<Color>("rect"),
    &Some(Color {
      r: 255,
      g: 255,
      b: 255
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some(Position { x: 1.0, y: 1.0 })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some(Position { x: 2.0, y: 2.0 })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some(Position { x: 2.0, y: 2.0 }),
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Position { x: 0.0, y: 0.0 }),
    ]
  );
  assert_eq!(
    world.get_components::<Collider>(),
    &[None, None, Some(Collider { w: 1.0, h: 1.0 })]
  );
  assert_eq!(
    world.get_components::<Color>(),
    &[
      None,
      None,
      Some(Color {
        r: 255,
        g: 255,
        b: 255
      })
    ]
  );
}

#[test]
fn test_emit() {
  let mut world = World::new();
  let count = Rc::new(RefCell::new(0));
  {
    let count = Rc::clone(&count);
    world.on(move |_: &Position| {
      *count.borrow_mut() += 1;
    });
  }
  assert_eq!(*count.borrow(), 0);
  world.emit(Position { x: 0.0, y: 0.0 });
  assert_eq!(*count.borrow(), 1);
  world.emit(Position { x: 1.0, y: 1.0 });
  assert_eq!(*count.borrow(), 2);
  world.emit(Position { x: 2.0, y: 2.0 });
  assert_eq!(*count.borrow(), 3);
  world.emit(Collider { w: 2.0, h: 2.0 });
  assert_eq!(*count.borrow(), 3);
  world.emit(Color {
    r: 255,
    g: 255,
    b: 255,
  });
  assert_eq!(*count.borrow(), 3);
  {
    let count = Rc::clone(&count);
    world.on(move |&Position { x, y: _ }: &Position| {
      *count.borrow_mut() += x as i32;
    });
  }
  world.emit(Position { x: 3.0, y: 3.0 });
  assert_eq!(*count.borrow(), 7);
  {
    let count = Rc::clone(&count);
    world.on(move |&Position { x: _, y }: &Position| {
      *count.borrow_mut() *= y as i32;
    });
  }
  world.emit(Position { x: 4.0, y: 4.0 });
  assert_eq!(*count.borrow(), 48);
}

#[test]
fn test_find_archetype_from_world() {
  let mut world = World::new();
  assert_eq!(find_archetype!((world, Position)), Cow::Borrowed(&[]));
  assert_eq!(find_archetype!((world, Collider)), Cow::Borrowed(&[]));
  assert_eq!(find_archetype!((world, Color)), Cow::Borrowed(&[]));
  assert_eq!(
    find_archetype!((world, Position, Collider)),
    Cow::Borrowed(&[])
  );
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    Cow::Borrowed(&[])
  );
  world.put_component("rect", Position { x: 0.0, y: 0.0 });
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    Cow::Borrowed(&[(Some(Position { x: 0.0, y: 0.0 }), None, None)])
  );
  world.put_component("circle", Position { x: 1.0, y: 1.0 });
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    Cow::Borrowed(&[
      (Some(Position { x: 0.0, y: 0.0 }), None, None),
      (Some(Position { x: 1.0, y: 1.0 }), None, None)
    ])
  );
  world.put_component("player", Position { x: 2.0, y: 2.0 });
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    Cow::Borrowed(&[
      (Some(Position { x: 0.0, y: 0.0 }), None, None),
      (Some(Position { x: 1.0, y: 1.0 }), None, None),
      (Some(Position { x: 2.0, y: 2.0 }), None, None)
    ])
  );
  world.put_component("circle", Color { r: 0, g: 0, b: 0 });
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    Cow::Borrowed(&[
      (Some(Position { x: 0.0, y: 0.0 }), None, None),
      (
        Some(Position { x: 1.0, y: 1.0 }),
        None,
        Some(Color { r: 0, g: 0, b: 0 })
      ),
      (Some(Position { x: 2.0, y: 2.0 }), None, None)
    ])
  );
  world.put_component("rect", Collider { w: 1.0, h: 1.0 });
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    Cow::Borrowed(&[
      (
        Some(Position { x: 0.0, y: 0.0 }),
        Some(Collider { w: 1.0, h: 1.0 }),
        None
      ),
      (
        Some(Position { x: 1.0, y: 1.0 }),
        None,
        Some(Color { r: 0, g: 0, b: 0 })
      ),
      (Some(Position { x: 2.0, y: 2.0 }), None, None)
    ])
  );
  world.put_component("circle", Collider { w: 2.0, h: 2.0 });
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    Cow::Borrowed(&[
      (
        Some(Position { x: 0.0, y: 0.0 }),
        Some(Collider { w: 1.0, h: 1.0 }),
        None
      ),
      (
        Some(Position { x: 1.0, y: 1.0 }),
        Some(Collider { w: 2.0, h: 2.0 }),
        Some(Color { r: 0, g: 0, b: 0 })
      ),
      (Some(Position { x: 2.0, y: 2.0 }), None, None)
    ])
  );
  world.remove_component::<Position>("player");
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    Cow::Borrowed(&[
      (
        Some(Position { x: 0.0, y: 0.0 }),
        Some(Collider { w: 1.0, h: 1.0 }),
        None
      ),
      (
        Some(Position { x: 1.0, y: 1.0 }),
        Some(Collider { w: 2.0, h: 2.0 }),
        Some(Color { r: 0, g: 0, b: 0 })
      )
    ])
  );
  world.remove_component::<Position>("rect");
  world.remove_component::<Collider>("rect");
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    Cow::Borrowed(&[(
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Collider { w: 2.0, h: 2.0 }),
      Some(Color { r: 0, g: 0, b: 0 })
    )])
  );
  world.remove_component::<Position>("circle");
  world.remove_component::<Collider>("circle");
  world.remove_component::<Color>("circle");
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    Cow::Borrowed(&[])
  );
  world.put_component("rect", Position { x: 0.0, y: 0.0 });
  world.put_component("circle", Position { x: 1.0, y: 1.0 });
  world.put_component("player", Position { x: 2.0, y: 2.0 });
  world.put_component("circle", Color { r: 0, g: 0, b: 0 });
  world.put_component("rect", Collider { w: 1.0, h: 1.0 });
  world.put_component("circle", Collider { w: 2.0, h: 2.0 });
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    Cow::Borrowed(&[
      (
        Some(Position { x: 1.0, y: 1.0 }),
        Some(Collider { w: 2.0, h: 2.0 }),
        Some(Color { r: 0, g: 0, b: 0 })
      ),
      (
        Some(Position { x: 0.0, y: 0.0 }),
        Some(Collider { w: 1.0, h: 1.0 }),
        None
      ),
      (Some(Position { x: 2.0, y: 2.0 }), None, None)
    ])
  );
}

#[test]
fn test_find_archetype_from_heaven() {
  let mut heaven = World::new();
  assert_eq!(find_archetype!((heaven, Position)), Cow::Borrowed(&[]));
  assert_eq!(find_archetype!((heaven, Collider)), Cow::Borrowed(&[]));
  assert_eq!(find_archetype!((heaven, Color)), Cow::Borrowed(&[]));
  assert_eq!(
    find_archetype!((heaven, Position, Collider)),
    Cow::Borrowed(&[])
  );
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    Cow::Borrowed(&[])
  );
  heaven.put_component("rect", Position { x: 0.0, y: 0.0 });
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    Cow::Borrowed(&[(Some(Position { x: 0.0, y: 0.0 }), None, None)])
  );
  heaven.put_component("circle", Position { x: 1.0, y: 1.0 });
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    Cow::Borrowed(&[
      (Some(Position { x: 0.0, y: 0.0 }), None, None),
      (Some(Position { x: 1.0, y: 1.0 }), None, None)
    ])
  );
  heaven.put_component("player", Position { x: 2.0, y: 2.0 });
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    Cow::Borrowed(&[
      (Some(Position { x: 0.0, y: 0.0 }), None, None),
      (Some(Position { x: 1.0, y: 1.0 }), None, None),
      (Some(Position { x: 2.0, y: 2.0 }), None, None)
    ])
  );
  heaven.put_component("circle", Color { r: 0, g: 0, b: 0 });
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    Cow::Borrowed(&[
      (Some(Position { x: 0.0, y: 0.0 }), None, None),
      (
        Some(Position { x: 1.0, y: 1.0 }),
        None,
        Some(Color { r: 0, g: 0, b: 0 })
      ),
      (Some(Position { x: 2.0, y: 2.0 }), None, None)
    ])
  );
  heaven.put_component("rect", Collider { w: 1.0, h: 1.0 });
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    Cow::Borrowed(&[
      (
        Some(Position { x: 0.0, y: 0.0 }),
        Some(Collider { w: 1.0, h: 1.0 }),
        None
      ),
      (
        Some(Position { x: 1.0, y: 1.0 }),
        None,
        Some(Color { r: 0, g: 0, b: 0 })
      ),
      (Some(Position { x: 2.0, y: 2.0 }), None, None)
    ])
  );
  heaven.put_component("circle", Collider { w: 2.0, h: 2.0 });
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    Cow::Borrowed(&[
      (
        Some(Position { x: 0.0, y: 0.0 }),
        Some(Collider { w: 1.0, h: 1.0 }),
        None
      ),
      (
        Some(Position { x: 1.0, y: 1.0 }),
        Some(Collider { w: 2.0, h: 2.0 }),
        Some(Color { r: 0, g: 0, b: 0 })
      ),
      (Some(Position { x: 2.0, y: 2.0 }), None, None)
    ])
  );
  heaven.remove_component::<Position>("player");
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    Cow::Borrowed(&[
      (
        Some(Position { x: 0.0, y: 0.0 }),
        Some(Collider { w: 1.0, h: 1.0 }),
        None
      ),
      (
        Some(Position { x: 1.0, y: 1.0 }),
        Some(Collider { w: 2.0, h: 2.0 }),
        Some(Color { r: 0, g: 0, b: 0 })
      )
    ])
  );
  heaven.remove_component::<Position>("rect");
  heaven.remove_component::<Collider>("rect");
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    Cow::Borrowed(&[(
      Some(Position { x: 1.0, y: 1.0 }),
      Some(Collider { w: 2.0, h: 2.0 }),
      Some(Color { r: 0, g: 0, b: 0 })
    )])
  );
  heaven.remove_component::<Position>("circle");
  heaven.remove_component::<Collider>("circle");
  heaven.remove_component::<Color>("circle");
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    Cow::Borrowed(&[])
  );
  heaven.put_component("rect", Position { x: 0.0, y: 0.0 });
  heaven.put_component("circle", Position { x: 1.0, y: 1.0 });
  heaven.put_component("player", Position { x: 2.0, y: 2.0 });
  heaven.put_component("circle", Color { r: 0, g: 0, b: 0 });
  heaven.put_component("rect", Collider { w: 1.0, h: 1.0 });
  heaven.put_component("circle", Collider { w: 2.0, h: 2.0 });
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    Cow::Borrowed(&[
      (
        Some(Position { x: 1.0, y: 1.0 }),
        Some(Collider { w: 2.0, h: 2.0 }),
        Some(Color { r: 0, g: 0, b: 0 })
      ),
      (
        Some(Position { x: 0.0, y: 0.0 }),
        Some(Collider { w: 1.0, h: 1.0 }),
        None
      ),
      (Some(Position { x: 2.0, y: 2.0 }), None, None)
    ])
  );
}

#[derive(Debug)]
struct MovementSystem {
  count: Shared<f64>,
}

impl System for MovementSystem {
  fn step(&mut self, dt: f64) {
    *self.count.borrow_mut() += dt;
  }
}

#[derive(Debug)]
struct CollisionSystem {
  count: Shared<f64>,
}

impl System for CollisionSystem {
  fn step(&mut self, dt: f64) {
    *self.count.borrow_mut() *= dt;
  }
}

#[derive(Debug)]
struct GraphicsSystem {
  count: Shared<f64>,
}

impl System for GraphicsSystem {
  fn step(&mut self, dt: f64) {
    *self.count.borrow_mut() -= dt;
  }
}

#[test]
fn test_step() {
  let mut world = World::new();
  let count = Rc::new(RefCell::new(0.0));
  world.add_system(MovementSystem {
    count: Rc::clone(&count),
  });
  world.step(0.0);
  assert_eq!(*count.borrow(), 0.0);
  world.step(1.0);
  assert_eq!(*count.borrow(), 1.0);
  world.step(2.0);
  assert_eq!(*count.borrow(), 3.0);
  world.step(3.0);
  assert_eq!(*count.borrow(), 6.0);
  world.add_system(CollisionSystem {
    count: Rc::clone(&count),
  });
  world.step(4.0);
  assert_eq!(*count.borrow(), 40.0);
  world.add_system(GraphicsSystem {
    count: Rc::clone(&count),
  });
  world.step(5.0);
  assert_eq!(*count.borrow(), 220.0);
}
