use std::num::NonZeroU32;

#[cfg(feature = "opengl")]
use super::opengl;

pub struct Graphics<T: GraphicsApi> {
    api: T,
}

#[cfg(feature = "opengl")]
impl Graphics<opengl::OpenGl> {
    pub fn new(loader_function: &dyn Fn(&str) -> *const std::ffi::c_void) -> Self {
        Self {
            api: opengl::OpenGl::new(loader_function),
        }
    }
}

impl<T: GraphicsApi> std::ops::Deref for Graphics<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.api
    }
}

pub type ApiHandle = NonZeroU32;
pub type ApiEnum = u32;

pub trait GraphicsApi {
    // draw
    // clear screen
    // set viewport
    // set state
}
