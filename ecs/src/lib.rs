mod archetype;
mod component;
mod entity;
mod system;
mod world;

use std::time;

pub use entity::*;
pub use world::*;

#[derive(Debug)]
struct Position {
    coords: (f32, f32, f32),
}

#[derive(Debug)]
struct Speed {
    speed: u32,
}

pub fn a_whole_new_world() {
    let mut world = World::new();
    world.register_component::<Position>();
    world.register_component::<Speed>();

    for i in 0..10000 {
        let player = world.create_entity();

        if i % 2 == 0 {
            let position = Position {
                coords: (0.0, 1.0, 2.0),
            };
            world.set_component(&player, position).unwrap();
        } else {
            let position = Position {
                coords: (0.0, 1.0, 2.0),
            };
            let speed = Speed { speed: 27 };

            world.set_component(&player, position).unwrap();
            world.set_component(&player, speed).unwrap();
        }
    }

    world.add_system(move_system);

    world.run_systems();
}

fn move_system(view: &mut View, _dt: time::Duration) {
    for (pos, speed) in view.iter_two_components_mut::<Position, Speed>().unwrap() {
        //    println!("pos: {:?}, speed: {:?}", pos, speed)
        pos.coords.0 += 2.0;
        speed.speed *= 12;
    }
}
