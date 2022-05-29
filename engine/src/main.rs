#![allow(dead_code)]

mod camera;
mod components;
mod context;
mod engine;
mod input;
mod loader;
mod math;
mod renderer;
mod util;

use context::Context;
use engine::Engine;

use colored::Colorize;

const FPS: f32 = 30.0;
const VSYNC: bool = true;

static LOGGER: Logger = Logger;

fn main() {
    let _ = log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    let (context, event_loop, gl) = Context::new();
    let engine = Engine::new(context, &gl);
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
