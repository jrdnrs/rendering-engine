use std::time;

use glow::{self as gl, HasContext};
use glutin::{self, event_loop::EventLoop, window::Window, ContextWrapper, PossiblyCurrent};
use log::info;

pub struct Context {
    // pub gl: gl::Context,
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

fn load_opengl(window_context: ContextWrapper<PossiblyCurrent, Window>) -> gl::Context {
    unsafe { gl::Context::from_loader_function(|s| window_context.get_proc_address(s) as *const _) }
}

impl Context {
    pub fn new() -> (Self, EventLoop<()>, gl::Context) {
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

            let gl = gl::Context::from_loader_function(|s| {
                window_context.get_proc_address(s) as *const _
            });

            info!(
                "Version: {}.{}",
                gl.get_parameter_i32(gl::MAJOR_VERSION),
                gl.get_parameter_i32(gl::MINOR_VERSION)
            );
            info!(
                "Profile: {}",
                if gl.get_parameter_i32(gl::CONTEXT_PROFILE_MASK) == 1 {
                    "Core"
                } else {
                    "Compatability"
                }
            );

            let frametime = time::Duration::from_secs_f32(1.0 / crate::FPS as f32);
            let target_time = time::Instant::from(time::Instant::now() + frametime);

            (
                Context {
                    window_context,
                    // gl,
                    target_frametime: frametime,
                    target_time,
                    last_frame_time: time::Instant::now(),
                    last_frame_delta: time::Duration::new(0, 0),
                    being_resized: false,
                    being_moved: false,
                    frames: 0,
                    fullscreen: false,
                },
                event_loop,
                gl,
            )
        }
    }
}
