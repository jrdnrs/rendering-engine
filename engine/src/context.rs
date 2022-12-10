use std::time;

use glutin::{self, event_loop::EventLoop, window::Window, PossiblyCurrent};
use log::info;

use crate::graphics::{self, GraphicsContext};

pub struct Context {
    pub graphics_context: GraphicsContext,
    pub window_context: glutin::ContextWrapper<PossiblyCurrent, Window>,

    pub target_frametime: time::Duration,
    pub target_time: time::Instant,

    pub last_frame_time: time::Instant,
    pub last_frame_delta: time::Duration,

    pub being_resized: bool,
    pub being_moved: bool,

    pub frames: u64,

    pub fullscreen: bool,
}

impl Context {
    pub fn new() -> (Self, EventLoop<()>) {
        unsafe {
            let event_loop = glutin::event_loop::EventLoop::new();

            let window_builder = glutin::window::WindowBuilder::new()
                .with_title("Hello world!")
                .with_inner_size(glutin::dpi::LogicalSize::new(crate::WIDTH, crate::HEIGHT));

            let window_context = glutin::ContextBuilder::new()
                .with_vsync(crate::VSYNC)
                .with_double_buffer(Some(true))
                // .with_multisampling(crate::SAMPLES)
                .with_gl_profile(glutin::GlProfile::Core)
                .with_gl(glutin::GlRequest::Latest)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap();

            let graphics_context = graphics::GraphicsContext::init(&|s| {
                window_context.get_proc_address(s) as *const _
            });

            let frametime = time::Duration::from_secs_f32(1.0 / crate::FPS as f32);
            let target_time = time::Instant::from(time::Instant::now() + frametime);

            let context = Context {
                graphics_context,
                window_context,
                target_frametime: frametime,
                target_time,
                last_frame_time: time::Instant::now(),
                last_frame_delta: time::Duration::new(0, 0),
                being_resized: false,
                being_moved: false,
                frames: 0,
                fullscreen: false,
            };

            #[cfg(debug_assertions)]
            context.debug_info();

            (context, event_loop)
        }
    }

    fn debug_info(&self) {
        info!(
            "Uniform Buffer Offset Alignment: {} B",
            self.graphics_context
                .info
                .get("uniform_buffer_offset_alignment")
                .unwrap()
        );
        info!(
            "Uniform Buffer Size: {:.2} MB",
            *self
                .graphics_context
                .info
                .get("max_uniform_buffer_size")
                .unwrap() as f64
                / 1_000_000.0,
        );
        info!(
            "Shader Storage Offset Alignment: {} B",
            self.graphics_context
                .info
                .get("shader_storage_offset_alignment")
                .unwrap(),
        );
        info!(
            "Shader Storage Size: {:.2} MB",
            *self
                .graphics_context
                .info
                .get("max_shader_storage_size")
                .unwrap() as f64
                / 1_000_000.0,
        );
    }
}
