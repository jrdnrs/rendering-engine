use crate::platform::rustgl as gl;
use crate::graphics::GraphicsApi;

pub struct OpenGl {}

impl OpenGl {
    pub fn new(loader_function: &dyn Fn(&str) -> *const std::ffi::c_void) -> Self {
        unsafe {
            if let Err(res) = gl::load_gl_functions(loader_function) {
                for s in res {
                    println!("{s}");
                }
            }
        }
        return OpenGl {};
    }
}

impl GraphicsApi for OpenGl {

}
