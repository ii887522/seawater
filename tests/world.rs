use iron_ingot::Shared;
use seawater::prelude::*;
use seawater_macro::define_component;
use std::{cell::RefCell, rc::Rc};

define_component! {
  struct Position {
    x: f32,
    y: f32,
  }
}

define_component! {
  struct Collider {
    w: f32,
    h: f32,
  }
}

define_component! {
  struct Color {
    r: u8,
    g: u8,
    b: u8,
  }
}

#[test]
fn test_component_related_methods() {
  let mut world = World::new();
  world.put_component("rect", {
    let position = Position::new();
    *position.x() = 0.0;
    *position.y() = 0.0;
    position
  });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 0.0;
      *position.y() = 0.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[Some({
      let position = Position::new();
      *position.x() = 0.0;
      *position.y() = 0.0;
      position
    })]
  );
  world.put_component("circle", {
    let position = Position::new();
    *position.x() = 1.0;
    *position.y() = 1.0;
    position
  });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 0.0;
      *position.y() = 0.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 0.0;
        *position.y() = 0.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      })
    ]
  );
  world.put_component("player", {
    let position = Position::new();
    *position.x() = 2.0;
    *position.y() = 2.0;
    position
  });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 0.0;
      *position.y() = 0.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 0.0;
        *position.y() = 0.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      })
    ]
  );
  world.remove_component::<Position>("rect");
  assert_eq!(world.get_component::<Position>("rect"), &None);
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      None,
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      })
    ]
  );
  world.put_component("rect", {
    let position = Position::new();
    *position.x() = 0.0;
    *position.y() = 0.0;
    position
  });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 0.0;
      *position.y() = 0.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 0.0;
        *position.y() = 0.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      })
    ]
  );
  world.put_component("rect", {
    let position = Position::new();
    *position.x() = 3.0;
    *position.y() = 3.0;
    position
  });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 3.0;
      *position.y() = 3.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 3.0;
        *position.y() = 3.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      })
    ]
  );
  world.remove_component::<Position>("circle");
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 3.0;
      *position.y() = 3.0;
      position
    })
  );
  assert_eq!(world.get_component::<Position>("circle"), &None);
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 3.0;
        *position.y() = 3.0;
        position
      }),
      None,
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      })
    ]
  );
  world.put_component("circle", {
    let position = Position::new();
    *position.x() = 1.0;
    *position.y() = 1.0;
    position
  });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 3.0;
      *position.y() = 3.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 3.0;
        *position.y() = 3.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      })
    ]
  );
  world.remove_component::<Position>("player");
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 3.0;
      *position.y() = 3.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(world.get_component::<Position>("player"), &None);
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 3.0;
        *position.y() = 3.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      None
    ]
  );
  world.put_component("player", {
    let position = Position::new();
    *position.x() = 0.0;
    *position.y() = 0.0;
    position
  });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 3.0;
      *position.y() = 3.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 0.0;
      *position.y() = 0.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 3.0;
        *position.y() = 3.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 0.0;
        *position.y() = 0.0;
        position
      }),
    ]
  );
  world.remove_component::<Position>("rect");
  world.remove_component::<Position>("circle");
  world.remove_component::<Position>("player");
  assert_eq!(world.get_component::<Position>("rect"), &None);
  assert_eq!(world.get_component::<Position>("circle"), &None);
  assert_eq!(world.get_component::<Position>("player"), &None);
  assert_eq!(world.get_components::<Position>(), &[]);
  world.put_component("rect", {
    let position = Position::new();
    *position.x() = 0.0;
    *position.y() = 0.0;
    position
  });
  world.put_component("circle", {
    let position = Position::new();
    *position.x() = 1.0;
    *position.y() = 1.0;
    position
  });
  world.put_component("player", {
    let position = Position::new();
    *position.x() = 2.0;
    *position.y() = 2.0;
    position
  });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 0.0;
      *position.y() = 0.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 0.0;
        *position.y() = 0.0;
        position
      }),
    ]
  );
  world.put_component("rect", {
    let collider = Collider::new();
    *collider.w() = 1.0;
    *collider.h() = 1.0;
    collider
  });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 0.0;
      *position.y() = 0.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Collider>("rect"),
    &Some({
      let collider = Collider::new();
      *collider.w() = 1.0;
      *collider.h() = 1.0;
      collider
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 0.0;
        *position.y() = 0.0;
        position
      }),
    ]
  );
  assert_eq!(
    world.get_components::<Collider>(),
    &[
      None,
      None,
      Some({
        let collider = Collider::new();
        *collider.w() = 1.0;
        *collider.h() = 1.0;
        collider
      })
    ]
  );
  world.put_component("rect", {
    let color = Color::new();
    *color.r() = 255;
    *color.g() = 255;
    *color.b() = 255;
    color
  });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 0.0;
      *position.y() = 0.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Collider>("rect"),
    &Some({
      let collider = Collider::new();
      *collider.w() = 1.0;
      *collider.h() = 1.0;
      collider
    })
  );
  assert_eq!(
    world.get_component::<Color>("rect"),
    &Some({
      let color = Color::new();
      *color.r() = 255;
      *color.g() = 255;
      *color.b() = 255;
      color
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 0.0;
        *position.y() = 0.0;
        position
      }),
    ]
  );
  assert_eq!(
    world.get_components::<Collider>(),
    &[
      None,
      None,
      Some({
        let collider = Collider::new();
        *collider.w() = 1.0;
        *collider.h() = 1.0;
        collider
      })
    ]
  );
  assert_eq!(
    world.get_components::<Color>(),
    &[
      None,
      None,
      Some({
        let color = Color::new();
        *color.r() = 255;
        *color.g() = 255;
        *color.b() = 255;
        color
      })
    ]
  );
  world.remove_component::<Position>("rect");
  assert_eq!(world.get_component::<Position>("rect"), &None);
  assert_eq!(
    world.get_component::<Collider>("rect"),
    &Some({
      let collider = Collider::new();
      *collider.w() = 1.0;
      *collider.h() = 1.0;
      collider
    })
  );
  assert_eq!(
    world.get_component::<Color>("rect"),
    &Some({
      let color = Color::new();
      *color.r() = 255;
      *color.g() = 255;
      *color.b() = 255;
      color
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(world.get_component::<Position>("rect"), &None);
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      None,
    ]
  );
  assert_eq!(
    world.get_components::<Collider>(),
    &[
      None,
      None,
      Some({
        let collider = Collider::new();
        *collider.w() = 1.0;
        *collider.h() = 1.0;
        collider
      })
    ]
  );
  assert_eq!(
    world.get_components::<Color>(),
    &[
      None,
      None,
      Some({
        let color = Color::new();
        *color.r() = 255;
        *color.g() = 255;
        *color.b() = 255;
        color
      })
    ]
  );
  world.put_component("rect", {
    let position = Position::new();
    *position.x() = 0.0;
    *position.y() = 0.0;
    position
  });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 0.0;
      *position.y() = 0.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Collider>("rect"),
    &Some({
      let collider = Collider::new();
      *collider.w() = 1.0;
      *collider.h() = 1.0;
      collider
    })
  );
  assert_eq!(
    world.get_component::<Color>("rect"),
    &Some({
      let color = Color::new();
      *color.r() = 255;
      *color.g() = 255;
      *color.b() = 255;
      color
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 0.0;
        *position.y() = 0.0;
        position
      }),
    ]
  );
  assert_eq!(
    world.get_components::<Collider>(),
    &[
      None,
      None,
      Some({
        let collider = Collider::new();
        *collider.w() = 1.0;
        *collider.h() = 1.0;
        collider
      })
    ]
  );
  assert_eq!(
    world.get_components::<Color>(),
    &[
      None,
      None,
      Some({
        let color = Color::new();
        *color.r() = 255;
        *color.g() = 255;
        *color.b() = 255;
        color
      })
    ]
  );
  world.remove_component::<Collider>("rect");
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 0.0;
      *position.y() = 0.0;
      position
    })
  );
  assert_eq!(world.get_component::<Collider>("rect"), &None);
  assert_eq!(
    world.get_component::<Color>("rect"),
    &Some({
      let color = Color::new();
      *color.r() = 255;
      *color.g() = 255;
      *color.b() = 255;
      color
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 0.0;
        *position.y() = 0.0;
        position
      }),
    ]
  );
  assert_eq!(world.get_components::<Collider>(), &[]);
  assert_eq!(
    world.get_components::<Color>(),
    &[
      None,
      None,
      Some({
        let color = Color::new();
        *color.r() = 255;
        *color.g() = 255;
        *color.b() = 255;
        color
      })
    ]
  );
  world.put_component("rect", {
    let collider = Collider::new();
    *collider.w() = 1.0;
    *collider.h() = 1.0;
    collider
  });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 0.0;
      *position.y() = 0.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Collider>("rect"),
    &Some({
      let collider = Collider::new();
      *collider.w() = 1.0;
      *collider.h() = 1.0;
      collider
    })
  );
  assert_eq!(
    world.get_component::<Color>("rect"),
    &Some({
      let color = Color::new();
      *color.r() = 255;
      *color.g() = 255;
      *color.b() = 255;
      color
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 0.0;
        *position.y() = 0.0;
        position
      }),
    ]
  );
  assert_eq!(
    world.get_components::<Collider>(),
    &[
      None,
      None,
      Some({
        let collider = Collider::new();
        *collider.w() = 1.0;
        *collider.h() = 1.0;
        collider
      })
    ]
  );
  assert_eq!(
    world.get_components::<Color>(),
    &[
      None,
      None,
      Some({
        let color = Color::new();
        *color.r() = 255;
        *color.g() = 255;
        *color.b() = 255;
        color
      })
    ]
  );
  world.remove_component::<Color>("rect");
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 0.0;
      *position.y() = 0.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Collider>("rect"),
    &Some({
      let collider = Collider::new();
      *collider.w() = 1.0;
      *collider.h() = 1.0;
      collider
    })
  );
  assert_eq!(world.get_component::<Color>("rect"), &None);
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 0.0;
        *position.y() = 0.0;
        position
      }),
    ]
  );
  assert_eq!(
    world.get_components::<Collider>(),
    &[
      None,
      None,
      Some({
        let collider = Collider::new();
        *collider.w() = 1.0;
        *collider.h() = 1.0;
        collider
      })
    ]
  );
  assert_eq!(world.get_components::<Color>(), &[]);
  world.put_component("rect", {
    let color = Color::new();
    *color.r() = 255;
    *color.g() = 255;
    *color.b() = 255;
    color
  });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 0.0;
      *position.y() = 0.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Collider>("rect"),
    &Some({
      let collider = Collider::new();
      *collider.w() = 1.0;
      *collider.h() = 1.0;
      collider
    })
  );
  assert_eq!(
    world.get_component::<Color>("rect"),
    &Some({
      let color = Color::new();
      *color.r() = 255;
      *color.g() = 255;
      *color.b() = 255;
      color
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 0.0;
        *position.y() = 0.0;
        position
      }),
    ]
  );
  assert_eq!(
    world.get_components::<Collider>(),
    &[
      None,
      None,
      Some({
        let collider = Collider::new();
        *collider.w() = 1.0;
        *collider.h() = 1.0;
        collider
      })
    ]
  );
  assert_eq!(
    world.get_components::<Color>(),
    &[
      None,
      None,
      Some({
        let color = Color::new();
        *color.r() = 255;
        *color.g() = 255;
        *color.b() = 255;
        color
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
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
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
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      None,
    ]
  );
  assert_eq!(world.get_components::<Collider>(), &[]);
  assert_eq!(world.get_components::<Color>(), &[]);
  world.put_component("rect", {
    let position = Position::new();
    *position.x() = 0.0;
    *position.y() = 0.0;
    position
  });
  world.put_component("rect", {
    let collider = Position::new();
    *collider.x() = 1.0;
    *collider.y() = 1.0;
    collider
  });
  world.put_component("rect", {
    let color = Color::new();
    *color.r() = 255;
    *color.g() = 255;
    *color.b() = 255;
    color
  });
  assert_eq!(
    world.get_component::<Position>("rect"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(world.get_component::<Collider>("rect"), &None);
  assert_eq!(
    world.get_component::<Color>("rect"),
    &Some({
      let color = Color::new();
      *color.r() = 255;
      *color.g() = 255;
      *color.b() = 255;
      color
    })
  );
  assert_eq!(
    world.get_component::<Position>("circle"),
    &Some({
      let position = Position::new();
      *position.x() = 1.0;
      *position.y() = 1.0;
      position
    })
  );
  assert_eq!(
    world.get_component::<Position>("player"),
    &Some({
      let position = Position::new();
      *position.x() = 2.0;
      *position.y() = 2.0;
      position
    })
  );
  assert_eq!(
    world.get_components::<Position>(),
    &[
      Some({
        let position = Position::new();
        *position.x() = 2.0;
        *position.y() = 2.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
    ]
  );
  assert_eq!(world.get_components::<Collider>(), &[]);
  assert_eq!(
    world.get_components::<Color>(),
    &[
      None,
      None,
      Some({
        let color = Color::new();
        *color.r() = 255;
        *color.g() = 255;
        *color.b() = 255;
        color
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
  world.emit({
    let position = Position::new();
    *position.x() = 0.0;
    *position.y() = 0.0;
    position
  });
  assert_eq!(*count.borrow(), 1);
  world.emit({
    let position = Position::new();
    *position.x() = 1.0;
    *position.y() = 1.0;
    position
  });
  assert_eq!(*count.borrow(), 2);
  world.emit({
    let position = Position::new();
    *position.x() = 2.0;
    *position.y() = 2.0;
    position
  });
  assert_eq!(*count.borrow(), 3);
  world.emit({
    let collider = Collider::new();
    *collider.w() = 0.0;
    *collider.h() = 0.0;
    collider
  });
  assert_eq!(*count.borrow(), 3);
  world.emit({
    let color = Color::new();
    *color.r() = 255;
    *color.g() = 255;
    *color.b() = 255;
    color
  });
  assert_eq!(*count.borrow(), 3);
  {
    let count = Rc::clone(&count);
    world.on(move |position: &Position| {
      *count.borrow_mut() += *position.x() as i32;
    });
  }
  world.emit({
    let position = Position::new();
    *position.x() = 3.0;
    *position.y() = 3.0;
    position
  });
  assert_eq!(*count.borrow(), 7);
  {
    let count = Rc::clone(&count);
    world.on(move |position: &Position| {
      *count.borrow_mut() *= *position.y() as i32;
    });
  }
  world.emit({
    let position = Position::new();
    *position.x() = 4.0;
    *position.y() = 4.0;
    position
  });
  assert_eq!(*count.borrow(), 48);
}

#[test]
fn test_find_archetype_from_world() {
  let mut world = World::new();
  assert_eq!(
    find_archetype!((world, Position)),
    Vec::<&Option<Position>>::new()
  );
  assert_eq!(
    find_archetype!((world, Collider)),
    Vec::<&Option<Collider>>::new()
  );
  assert_eq!(
    find_archetype!((world, Color)),
    Vec::<&Option<Color>>::new()
  );
  assert_eq!(find_archetype!((world, Position, Collider)), vec![]);
  assert_eq!(find_archetype!((world, Position, Collider, Color)), vec![]);
  world.put_component("rect", {
    let position = Position::new();
    *position.x() = 4.0;
    *position.y() = 4.0;
    position
  });
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    vec![(
      &Some({
        let position = Position::new();
        *position.x() = 4.0;
        *position.y() = 4.0;
        position
      }),
      &None,
      &None
    )]
  );
  world.put_component("circle", {
    let position = Position::new();
    *position.x() = 1.0;
    *position.y() = 1.0;
    position
  });
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    vec![
      (
        &Some({
          let position = Position::new();
          *position.x() = 4.0;
          *position.y() = 4.0;
          position
        }),
        &None,
        &None
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 1.0;
          *position.y() = 1.0;
          position
        }),
        &None,
        &None
      )
    ]
  );
  world.put_component("player", {
    let position = Position::new();
    *position.x() = 2.0;
    *position.y() = 2.0;
    position
  });
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    vec![
      (
        &Some({
          let position = Position::new();
          *position.x() = 4.0;
          *position.y() = 4.0;
          position
        }),
        &None,
        &None
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 1.0;
          *position.y() = 1.0;
          position
        }),
        &None,
        &None
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 2.0;
          *position.y() = 2.0;
          position
        }),
        &None,
        &None
      )
    ]
  );
  world.put_component("circle", {
    let color = Color::new();
    *color.r() = 0;
    *color.g() = 0;
    *color.b() = 0;
    color
  });
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    vec![
      (
        &Some({
          let position = Position::new();
          *position.x() = 4.0;
          *position.y() = 4.0;
          position
        }),
        &None,
        &None
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 1.0;
          *position.y() = 1.0;
          position
        }),
        &None,
        &Some({
          let color = Color::new();
          *color.r() = 0;
          *color.g() = 0;
          *color.b() = 0;
          color
        })
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 2.0;
          *position.y() = 2.0;
          position
        }),
        &None,
        &None
      )
    ]
  );
  world.put_component("rect", {
    let collider = Collider::new();
    *collider.w() = 1.0;
    *collider.h() = 1.0;
    collider
  });
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    vec![
      (
        &Some({
          let position = Position::new();
          *position.x() = 4.0;
          *position.y() = 4.0;
          position
        }),
        &Some({
          let collider = Collider::new();
          *collider.w() = 1.0;
          *collider.h() = 1.0;
          collider
        }),
        &None
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 1.0;
          *position.y() = 1.0;
          position
        }),
        &None,
        &Some({
          let color = Color::new();
          *color.r() = 0;
          *color.g() = 0;
          *color.b() = 0;
          color
        })
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 2.0;
          *position.y() = 2.0;
          position
        }),
        &None,
        &None
      )
    ]
  );
  world.put_component("circle", {
    let collider = Collider::new();
    *collider.w() = 2.0;
    *collider.h() = 2.0;
    collider
  });
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    vec![
      (
        &Some({
          let position = Position::new();
          *position.x() = 4.0;
          *position.y() = 4.0;
          position
        }),
        &Some({
          let collider = Collider::new();
          *collider.w() = 1.0;
          *collider.h() = 1.0;
          collider
        }),
        &None
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 1.0;
          *position.y() = 1.0;
          position
        }),
        &Some({
          let collider = Collider::new();
          *collider.w() = 2.0;
          *collider.h() = 2.0;
          collider
        }),
        &Some({
          let color = Color::new();
          *color.r() = 0;
          *color.g() = 0;
          *color.b() = 0;
          color
        })
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 2.0;
          *position.y() = 2.0;
          position
        }),
        &None,
        &None
      )
    ]
  );
  world.remove_component::<Position>("player");
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    vec![
      (
        &Some({
          let position = Position::new();
          *position.x() = 4.0;
          *position.y() = 4.0;
          position
        }),
        &Some({
          let collider = Collider::new();
          *collider.w() = 1.0;
          *collider.h() = 1.0;
          collider
        }),
        &None
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 1.0;
          *position.y() = 1.0;
          position
        }),
        &Some({
          let collider = Collider::new();
          *collider.w() = 2.0;
          *collider.h() = 2.0;
          collider
        }),
        &Some({
          let color = Color::new();
          *color.r() = 0;
          *color.g() = 0;
          *color.b() = 0;
          color
        })
      )
    ]
  );
  world.remove_component::<Position>("rect");
  world.remove_component::<Collider>("rect");
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    vec![(
      &Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      &Some({
        let collider = Collider::new();
        *collider.w() = 2.0;
        *collider.h() = 2.0;
        collider
      }),
      &Some({
        let color = Color::new();
        *color.r() = 0;
        *color.g() = 0;
        *color.b() = 0;
        color
      })
    )]
  );
  world.remove_component::<Position>("circle");
  world.remove_component::<Collider>("circle");
  world.remove_component::<Color>("circle");
  assert_eq!(find_archetype!((world, Position, Collider, Color)), vec![]);
  world.put_component("rect", {
    let position = Position::new();
    *position.x() = 0.0;
    *position.y() = 0.0;
    position
  });
  world.put_component("circle", {
    let position = Position::new();
    *position.x() = 1.0;
    *position.y() = 1.0;
    position
  });
  world.put_component("player", {
    let position = Position::new();
    *position.x() = 2.0;
    *position.y() = 2.0;
    position
  });
  world.put_component("circle", {
    let color = Color::new();
    *color.r() = 0;
    *color.g() = 0;
    *color.b() = 0;
    color
  });
  world.put_component("rect", {
    let collider = Collider::new();
    *collider.w() = 1.0;
    *collider.h() = 1.0;
    collider
  });
  world.put_component("circle", {
    let collider = Collider::new();
    *collider.w() = 2.0;
    *collider.h() = 2.0;
    collider
  });
  assert_eq!(
    find_archetype!((world, Position, Collider, Color)),
    vec![
      (
        &Some({
          let position = Position::new();
          *position.x() = 1.0;
          *position.y() = 1.0;
          position
        }),
        &Some({
          let collider = Collider::new();
          *collider.w() = 2.0;
          *collider.h() = 2.0;
          collider
        }),
        &Some({
          let color = Color::new();
          *color.r() = 0;
          *color.g() = 0;
          *color.b() = 0;
          color
        })
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 0.0;
          *position.y() = 0.0;
          position
        }),
        &Some({
          let collider = Collider::new();
          *collider.w() = 1.0;
          *collider.h() = 1.0;
          collider
        }),
        &None
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 2.0;
          *position.y() = 2.0;
          position
        }),
        &None,
        &None
      )
    ]
  );
}

#[test]
fn test_find_archetype_from_heaven() {
  let mut heaven = World::new();
  assert_eq!(
    find_archetype!((heaven, Collider)),
    Vec::<&Option<Collider>>::new()
  );
  assert_eq!(
    find_archetype!((heaven, Color)),
    Vec::<&Option<Color>>::new()
  );
  assert_eq!(
    find_archetype!((heaven, Position, Collider)),
    Vec::<(&Option<Position>, &Option<Collider>)>::new()
  );
  assert_eq!(find_archetype!((heaven, Position, Collider, Color)), vec![]);
  heaven.put_component("rect", {
    let position = Position::new();
    *position.x() = 0.0;
    *position.y() = 0.0;
    position
  });
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    vec![(
      &Some({
        let position = Position::new();
        *position.x() = 0.0;
        *position.y() = 0.0;
        position
      }),
      &None,
      &None
    )]
  );
  heaven.put_component("circle", {
    let position = Position::new();
    *position.x() = 1.0;
    *position.y() = 1.0;
    position
  });
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    vec![
      (
        &Some({
          let position = Position::new();
          *position.x() = 0.0;
          *position.y() = 0.0;
          position
        }),
        &None,
        &None
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 1.0;
          *position.y() = 1.0;
          position
        }),
        &None,
        &None
      )
    ]
  );
  heaven.put_component("player", {
    let position = Position::new();
    *position.x() = 2.0;
    *position.y() = 2.0;
    position
  });
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    vec![
      (
        &Some({
          let position = Position::new();
          *position.x() = 0.0;
          *position.y() = 0.0;
          position
        }),
        &None,
        &None
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 1.0;
          *position.y() = 1.0;
          position
        }),
        &None,
        &None
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 2.0;
          *position.y() = 2.0;
          position
        }),
        &None,
        &None
      )
    ]
  );
  heaven.put_component("circle", {
    let color = Color::new();
    *color.r() = 0;
    *color.g() = 0;
    *color.b() = 0;
    color
  });
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    vec![
      (
        &Some({
          let position = Position::new();
          *position.x() = 0.0;
          *position.y() = 0.0;
          position
        }),
        &None,
        &None
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 1.0;
          *position.y() = 1.0;
          position
        }),
        &None,
        &Some({
          let color = Color::new();
          *color.r() = 0;
          *color.g() = 0;
          *color.b() = 0;
          color
        })
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 2.0;
          *position.y() = 2.0;
          position
        }),
        &None,
        &None
      )
    ]
  );
  heaven.put_component("rect", {
    let collider = Collider::new();
    *collider.w() = 1.0;
    *collider.h() = 1.0;
    collider
  });
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    vec![
      (
        &Some({
          let position = Position::new();
          *position.x() = 0.0;
          *position.y() = 0.0;
          position
        }),
        &Some({
          let collider = Collider::new();
          *collider.w() = 1.0;
          *collider.h() = 1.0;
          collider
        }),
        &None
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 1.0;
          *position.y() = 1.0;
          position
        }),
        &None,
        &Some({
          let color = Color::new();
          *color.r() = 0;
          *color.g() = 0;
          *color.b() = 0;
          color
        })
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 2.0;
          *position.y() = 2.0;
          position
        }),
        &None,
        &None
      )
    ]
  );
  heaven.put_component("circle", {
    let collider = Collider::new();
    *collider.w() = 2.0;
    *collider.h() = 2.0;
    collider
  });
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    vec![
      (
        &Some({
          let position = Position::new();
          *position.x() = 0.0;
          *position.y() = 0.0;
          position
        }),
        &Some({
          let collider = Collider::new();
          *collider.w() = 1.0;
          *collider.h() = 1.0;
          collider
        }),
        &None
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 1.0;
          *position.y() = 1.0;
          position
        }),
        &Some({
          let collider = Collider::new();
          *collider.w() = 2.0;
          *collider.h() = 2.0;
          collider
        }),
        &Some({
          let color = Color::new();
          *color.r() = 0;
          *color.g() = 0;
          *color.b() = 0;
          color
        })
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 2.0;
          *position.y() = 2.0;
          position
        }),
        &None,
        &None
      )
    ]
  );
  heaven.remove_component::<Position>("player");
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    vec![
      (
        &Some({
          let position = Position::new();
          *position.x() = 0.0;
          *position.y() = 0.0;
          position
        }),
        &Some({
          let collider = Collider::new();
          *collider.w() = 1.0;
          *collider.h() = 1.0;
          collider
        }),
        &None
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 1.0;
          *position.y() = 1.0;
          position
        }),
        &Some({
          let collider = Collider::new();
          *collider.w() = 2.0;
          *collider.h() = 2.0;
          collider
        }),
        &Some({
          let color = Color::new();
          *color.r() = 0;
          *color.g() = 0;
          *color.b() = 0;
          color
        })
      )
    ]
  );
  heaven.remove_component::<Position>("rect");
  heaven.remove_component::<Collider>("rect");
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    vec![(
      &Some({
        let position = Position::new();
        *position.x() = 1.0;
        *position.y() = 1.0;
        position
      }),
      &Some({
        let collider = Collider::new();
        *collider.w() = 2.0;
        *collider.h() = 2.0;
        collider
      }),
      &Some({
        let color = Color::new();
        *color.r() = 0;
        *color.g() = 0;
        *color.b() = 0;
        color
      })
    )]
  );
  heaven.remove_component::<Position>("circle");
  heaven.remove_component::<Collider>("circle");
  heaven.remove_component::<Color>("circle");
  assert_eq!(find_archetype!((heaven, Position, Collider, Color)), vec![]);
  heaven.put_component("rect", {
    let position = Position::new();
    *position.x() = 0.0;
    *position.y() = 0.0;
    position
  });
  heaven.put_component("circle", {
    let position = Position::new();
    *position.x() = 1.0;
    *position.y() = 1.0;
    position
  });
  heaven.put_component("player", {
    let position = Position::new();
    *position.x() = 2.0;
    *position.y() = 2.0;
    position
  });
  heaven.put_component("circle", {
    let color = Color::new();
    *color.r() = 0;
    *color.g() = 0;
    *color.b() = 0;
    color
  });
  heaven.put_component("rect", {
    let collider = Collider::new();
    *collider.w() = 1.0;
    *collider.h() = 1.0;
    collider
  });
  heaven.put_component("circle", {
    let collider = Collider::new();
    *collider.w() = 2.0;
    *collider.h() = 2.0;
    collider
  });
  assert_eq!(
    find_archetype!((heaven, Position, Collider, Color)),
    vec![
      (
        &Some({
          let position = Position::new();
          *position.x() = 1.0;
          *position.y() = 1.0;
          position
        }),
        &Some({
          let collider = Collider::new();
          *collider.w() = 2.0;
          *collider.h() = 2.0;
          collider
        }),
        &Some({
          let color = Color::new();
          *color.r() = 0;
          *color.g() = 0;
          *color.b() = 0;
          color
        })
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 0.0;
          *position.y() = 0.0;
          position
        }),
        &Some({
          let collider = Collider::new();
          *collider.w() = 1.0;
          *collider.h() = 1.0;
          collider
        }),
        &None
      ),
      (
        &Some({
          let position = Position::new();
          *position.x() = 2.0;
          *position.y() = 2.0;
          position
        }),
        &None,
        &None
      )
    ]
  );
  // let archetypes = {
  //   use rayon::prelude::*;

  //   let positions = heaven.get_components::<Position>();
  //   let colliders = heaven.get_components::<Collider>();
  //   let colors = heaven.get_components::<Color>();
  //   let a = (0..0
  //     .max(positions.len())
  //     .max(colliders.len())
  //     .max(colors.len()))
  //     .into_par_iter()
  //     .filter_map(|i| {
  //       let position = positions.get(i).unwrap_or(&None);
  //       let collider = colliders.get(i).unwrap_or(&None);
  //       let color = colors.get(i).unwrap_or(&None);
  //       if position.is_some() || collider.is_some() || color.is_some() {
  //         Some((position, collider, color))
  //       } else {
  //         None
  //       }
  //     })
  //     .collect::<Vec<_>>();
  // };
}

#[derive(Debug)]
struct MovementSystem {
  count: Shared<f32>,
}

impl System for MovementSystem {
  fn step(&mut self, dt: f32) {
    *self.count.borrow_mut() += dt;
  }
}

#[derive(Debug)]
struct CollisionSystem {
  count: Shared<f32>,
}

impl System for CollisionSystem {
  fn step(&mut self, dt: f32) {
    *self.count.borrow_mut() *= dt;
  }
}

#[derive(Debug)]
struct GraphicsSystem {
  count: Shared<f32>,
}

impl System for GraphicsSystem {
  fn step(&mut self, dt: f32) {
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
