use std::time;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use ecs::{self, View, World};

#[derive(Debug)]
struct Position {
    coords: (f32, f32, f32),
}

#[derive(Debug)]
struct Speed {
    speed: u32,
}

fn system_single(view: &mut View, dt: time::Duration) {
    for pos in view.iter_components_mut::<Position>().unwrap() {
        pos.coords.0 += 2.0;
    }
}

fn system_double(view: &mut View, dt: time::Duration) {
    for (pos, speed) in view.iter_two_components_mut::<Position, Speed>().unwrap() {
        pos.coords.0 += 2.0;
        speed.speed *= 12;
    }
}

fn system_double_zip(view: &mut View, dt: time::Duration) {
    for (pos, speed) in view
        .iter_two_components_mut_zip::<Position, Speed>()
        .unwrap()
    {
        pos.coords.0 += 2.0;
        speed.speed *= 12;
    }
}

fn setup_entities_single(n: u64) -> World {
    let mut world = World::new();
    world.register_component::<Position>();

    for _ in 0..n {
        let player = world.create_entity();

        let position = Position {
            coords: (0.0, 1.0, 2.0),
        };

        world.set_component(&player, position).unwrap();
    }

    world.add_system(system_single);
    world
}

fn setup_entities_double(n: u64) -> World {
    let mut world = World::new();
    world.register_component::<Position>();
    world.register_component::<Speed>();

    for _ in 0..n {
        let player = world.create_entity();

        let position = Position {
            coords: (0.0, 1.0, 2.0),
        };
        let speed = Speed { speed: 27 };

        world.set_component(&player, position).unwrap();
        world.set_component(&player, speed).unwrap();
    }

    world.add_system(system_double);
    world
}

fn setup_entities_double_zip(n: u64) -> World {
    let mut world = World::new();
    world.register_component::<Position>();
    world.register_component::<Speed>();

    for _ in 0..n {
        let player = world.create_entity();

        let position = Position {
            coords: (0.0, 1.0, 2.0),
        };
        let speed = Speed { speed: 27 };

        world.set_component(&player, position).unwrap();
        world.set_component(&player, speed).unwrap();
    }

    world.add_system(system_double_zip);
    world
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Component_Iterating");
    for entity_count in (0..10001).step_by(1000) {
        group.throughput(Throughput::Elements(entity_count));

        let mut world = setup_entities_single(entity_count);
        group.bench_function(BenchmarkId::new("single", entity_count), |b| {
            b.iter(|| world.run_systems())
        });

        let mut world = setup_entities_double(entity_count);
        group.bench_function(BenchmarkId::new("double", entity_count), |b| {
            b.iter(|| world.run_systems())
        });

        let mut world = setup_entities_double_zip(entity_count);
        group.bench_function(BenchmarkId::new("double (zip)", entity_count), |b| {
            b.iter(|| world.run_systems())
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
