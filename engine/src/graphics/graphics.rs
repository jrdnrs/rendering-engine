use std::{num::NonZeroU32, collections::HashMap};

#[cfg(feature = "opengl")]
pub use super::opengl::*;
use super::state::RasteriserState;

pub struct GraphicsContext {
    pub info: HashMap<&'static str, i64>
}

pub type ApiHandle = NonZeroU32;
pub type ApiEnum = u32;