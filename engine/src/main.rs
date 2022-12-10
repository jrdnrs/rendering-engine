#![allow(dead_code)]

mod components;
mod context;
mod engine;
mod graphics;
mod input;
mod math;
mod memory_manager;
mod renderer;
mod resource_manager;
mod util;
mod platform;

use colored::Colorize;
use context::Context;
use engine::Engine;

const FPS: f32 = 60.0;
const VSYNC: bool = true;
const SAMPLES: u32 = 1;
const WIDTH: u32 = 1600;
const HEIGHT: u32 = 900;

static LOGGER: Logger = Logger;

fn main() {
    let _ = log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    let (context, event_loop) = Context::new();
    let engine = Engine::new(context);
    engine.run(event_loop);
}
struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let col = match record.level() {
            log::Level::Error => "red",
            log::Level::Debug => "magenta",
            log::Level::Info => "white",
            log::Level::Warn => "yellow",
            log::Level::Trace => "green",
        };

        println!(
            "[{}]\t{}",
            record.level().as_str().color(col),
            record.args()
        );
    }

    fn flush(&self) {}
}
