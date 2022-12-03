// A lot of this is modified code from https://github.com/grovesNL/glow

#![allow(clippy::upper_case_acronyms)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::unused_unit)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]

use core::{fmt::Debug, hash::Hash};
use std::{
    ffi::{CStr, CString},
    num::{NonZeroU32, NonZeroU64},
};

use super::gl46 as native_gl;
pub use super::gl46::{load_gl_functions, GLsync};

pub struct ActiveUniform {
    pub size: i32,
    pub utype: u32,
    pub name: String,
}

pub struct ActiveAttribute {
    pub size: i32,
    pub atype: u32,
    pub name: String,
}

pub struct ActiveTransformFeedback {
    pub size: i32,
    pub tftype: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct DebugMessageLogEntry {
    source: u32,
    msg_type: u32,
    id: u32,
    severity: u32,
    message: String,
}

pub enum PixelPackData<'a> {
    BufferOffset(u32),
    Slice(&'a mut [u8]),
}

pub enum PixelUnpackData<'a> {
    BufferOffset(u32),
    Slice(&'a [u8]),
}

pub enum CompressedPixelUnpackData<'a> {
    BufferRange(core::ops::Range<u32>),
    Slice(&'a [u8]),
}

fn non_zero_u32_gl_name(value: native_gl::GLuint) -> NonZeroU32 {
    NonZeroU32::new(value as u32).expect("expected non-zero GL name")
}

fn non_zero_u64_gl_name(value: native_gl::GLuint64) -> NonZeroU64 {
    NonZeroU64::new(value as u64).expect("expected non-zero GL name")
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GlShader(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GlProgram(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GlBuffer(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GlVertexArray(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GlTexture(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GlTextureHandle(pub NonZeroU64);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GlSampler(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GlFence(pub native_gl::GLsync);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GlFramebuffer(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GlRenderbuffer(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GlQuery(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GlUniformLocation(pub native_gl::GLuint);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GlTransformFeedback(pub NonZeroU32);

pub unsafe fn create_framebuffer() -> Result<GlFramebuffer, String> {
    let mut name = 0;
    native_gl::glGenFramebuffers(1, &mut name);
    Ok(GlFramebuffer(non_zero_u32_gl_name(name)))
}

pub unsafe fn create_named_framebuffer() -> Result<GlFramebuffer, String> {
    let mut name = 0;
    native_gl::glCreateFramebuffers(1, &mut name);
    Ok(GlFramebuffer(non_zero_u32_gl_name(name)))
}

pub unsafe fn is_framebuffer(framebuffer: GlFramebuffer) -> bool {
    native_gl::glIsFramebuffer(framebuffer.0.get()) != 0
}

pub unsafe fn create_query() -> Result<GlQuery, String> {
    let mut name = 0;
    native_gl::glGenQueries(1, &mut name);
    Ok(GlQuery(non_zero_u32_gl_name(name)))
}

pub unsafe fn create_renderbuffer() -> Result<GlRenderbuffer, String> {
    let mut name = 0;
    native_gl::glGenRenderbuffers(1, &mut name);
    Ok(GlRenderbuffer(non_zero_u32_gl_name(name)))
}

pub unsafe fn create_named_renderbuffer() -> Result<GlRenderbuffer, String> {
    let mut name = 0;
    native_gl::glCreateRenderbuffers(1, &mut name);
    Ok(GlRenderbuffer(non_zero_u32_gl_name(name)))
}

pub unsafe fn is_renderbuffer(renderbuffer: GlRenderbuffer) -> bool {
    native_gl::glIsRenderbuffer(renderbuffer.0.get()) != 0
}

pub unsafe fn create_sampler() -> Result<GlSampler, String> {
    let mut name = 0;
    native_gl::glGenSamplers(1, &mut name);
    Ok(GlSampler(non_zero_u32_gl_name(name)))
}

pub unsafe fn create_shader(shader_type: u32) -> Result<GlShader, String> {
    Ok(GlShader(non_zero_u32_gl_name(native_gl::glCreateShader(
        shader_type as u32,
    ))))
}

pub unsafe fn is_shader(shader: GlShader) -> bool {
    native_gl::glIsShader(shader.0.get()) != 0
}

pub unsafe fn create_texture() -> Result<GlTexture, String> {
    let mut name = 0;
    native_gl::glGenTextures(1, &mut name);
    Ok(GlTexture(non_zero_u32_gl_name(name)))
}

pub unsafe fn create_named_texture(target: u32) -> Result<GlTexture, String> {
    let mut name = 0;
    native_gl::glCreateTextures(target, 1, &mut name);
    Ok(GlTexture(non_zero_u32_gl_name(name)))
}

pub unsafe fn is_texture(texture: GlTexture) -> bool {
    native_gl::glIsTexture(texture.0.get()) != 0
}

pub unsafe fn delete_shader(shader: GlShader) {
    native_gl::glDeleteShader(shader.0.get());
}

pub unsafe fn shader_source(shader: GlShader, source: &str) {
    native_gl::glShaderSource(
        shader.0.get(),
        1,
        &(source.as_ptr() as *const native_gl::GLchar),
        &(source.len() as native_gl::GLint),
    );
}

pub unsafe fn compile_shader(shader: GlShader) {
    native_gl::glCompileShader(shader.0.get());
}

pub unsafe fn get_shader_compile_status(shader: GlShader) -> bool {
    let mut status = 0;
    native_gl::glGetShaderiv(shader.0.get(), COMPILE_STATUS, &mut status);
    1 == status
}

pub unsafe fn get_shader_info_log(shader: GlShader) -> String {
    let mut length = 0;
    native_gl::glGetShaderiv(shader.0.get(), INFO_LOG_LENGTH, &mut length);
    if length > 0 {
        let mut log = String::with_capacity(length as usize);
        log.extend(std::iter::repeat('\0').take(length as usize));
        native_gl::glGetShaderInfoLog(
            shader.0.get(),
            length,
            &mut length,
            (&log[..]).as_ptr() as *mut native_gl::GLchar,
        );
        log.truncate(length as usize);
        log
    } else {
        String::from("")
    }
}

pub unsafe fn get_tex_image(target: u32, level: i32, format: u32, ty: u32, pixels: PixelPackData) {
    native_gl::glGetTexImage(
        target,
        level,
        format,
        ty,
        match pixels {
            PixelPackData::BufferOffset(offset) => offset as *mut std::ffi::c_void,
            PixelPackData::Slice(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
        },
    );
}

pub unsafe fn create_program() -> Result<GlProgram, String> {
    Ok(GlProgram(
        non_zero_u32_gl_name(native_gl::glCreateProgram()),
    ))
}

pub unsafe fn is_program(program: GlProgram) -> bool {
    native_gl::glIsProgram(program.0.get()) != 0
}

pub unsafe fn delete_program(program: GlProgram) {
    native_gl::glDeleteProgram(program.0.get());
}

pub unsafe fn attach_shader(program: GlProgram, shader: GlShader) {
    native_gl::glAttachShader(program.0.get(), shader.0.get());
}

pub unsafe fn detach_shader(program: GlProgram, shader: GlShader) {
    native_gl::glDetachShader(program.0.get(), shader.0.get());
}

pub unsafe fn link_program(program: GlProgram) {
    native_gl::glLinkProgram(program.0.get());
}

pub unsafe fn get_program_link_status(program: GlProgram) -> bool {
    let mut status = 0;
    native_gl::glGetProgramiv(program.0.get(), LINK_STATUS, &mut status);
    1 == status
}

pub unsafe fn get_program_info_log(program: GlProgram) -> String {
    let mut length = 0;
    native_gl::glGetProgramiv(program.0.get(), INFO_LOG_LENGTH, &mut length);
    if length > 0 {
        let mut log = String::with_capacity(length as usize);
        log.extend(std::iter::repeat('\0').take(length as usize));
        native_gl::glGetProgramInfoLog(
            program.0.get(),
            length,
            &mut length,
            (&log[..]).as_ptr() as *mut native_gl::GLchar,
        );
        log.truncate(length as usize);
        log
    } else {
        String::from("")
    }
}

pub unsafe fn get_active_uniforms(program: GlProgram) -> u32 {
    let mut count = 0;
    native_gl::glGetProgramiv(program.0.get(), ACTIVE_UNIFORMS, &mut count);
    count as u32
}

pub unsafe fn get_active_uniform(program: GlProgram, index: u32) -> Option<ActiveUniform> {
    let mut uniform_max_size = 0;
    native_gl::glGetProgramiv(
        program.0.get(),
        ACTIVE_UNIFORM_MAX_LENGTH,
        &mut uniform_max_size,
    );

    let mut name = String::with_capacity(uniform_max_size as usize);
    name.extend(std::iter::repeat('\0').take(uniform_max_size as usize));
    let mut length = 0;
    let mut size = 0;
    let mut utype = 0;
    native_gl::glGetActiveUniform(
        program.0.get(),
        index,
        uniform_max_size,
        &mut length,
        &mut size,
        &mut utype,
        name.as_ptr() as *mut native_gl::GLchar,
    );
    name.truncate(length as usize);

    Some(ActiveUniform { size, utype, name })
}

pub unsafe fn use_program(program: Option<GlProgram>) {
    native_gl::glUseProgram(program.map(|p| p.0.get()).unwrap_or(0));
}

pub unsafe fn create_buffer() -> Result<GlBuffer, String> {
    let mut buffer = 0;
    native_gl::glGenBuffers(1, &mut buffer);
    Ok(GlBuffer(non_zero_u32_gl_name(buffer)))
}

pub unsafe fn create_named_buffer() -> Result<GlBuffer, String> {
    let mut buffer = 0;
    native_gl::glCreateBuffers(1, &mut buffer);
    Ok(GlBuffer(non_zero_u32_gl_name(buffer)))
}

pub unsafe fn is_buffer(buffer: GlBuffer) -> bool {
    native_gl::glIsBuffer(buffer.0.get()) != 0
}

pub unsafe fn bind_buffer(target: u32, buffer: Option<GlBuffer>) {
    native_gl::glBindBuffer(target, buffer.map(|b| b.0.get()).unwrap_or(0));
}

pub unsafe fn bind_buffer_base(target: u32, index: u32, buffer: Option<GlBuffer>) {
    native_gl::glBindBufferBase(target, index, buffer.map(|b| b.0.get()).unwrap_or(0));
}

pub unsafe fn bind_buffer_range(
    target: u32,
    index: u32,
    buffer: Option<GlBuffer>,
    offset: i32,
    size: i32,
) {
    native_gl::glBindBufferRange(
        target,
        index,
        buffer.map(|b| b.0.get()).unwrap_or(0),
        offset as isize,
        size as isize,
    );
}

pub unsafe fn bind_vertex_buffer(
    binding_index: u32,
    buffer: Option<GlBuffer>,
    offset: i32,
    stride: i32,
) {
    native_gl::glBindVertexBuffer(
        binding_index,
        buffer.map(|b| b.0.get()).unwrap_or(0),
        offset as isize,
        stride,
    );
}

pub unsafe fn bind_framebuffer(target: u32, framebuffer: Option<GlFramebuffer>) {
    native_gl::glBindFramebuffer(target, framebuffer.map(|fb| fb.0.get()).unwrap_or(0));
}

pub unsafe fn bind_renderbuffer(target: u32, renderbuffer: Option<GlRenderbuffer>) {
    native_gl::glBindRenderbuffer(target, renderbuffer.map(|rb| rb.0.get()).unwrap_or(0));
}

pub unsafe fn blit_framebuffer(
    src_x0: i32,
    src_y0: i32,
    src_x1: i32,
    src_y1: i32,
    dst_x0: i32,
    dst_y0: i32,
    dst_x1: i32,
    dst_y1: i32,
    mask: u32,
    filter: u32,
) {
    native_gl::glBlitFramebuffer(
        src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
    );
}

pub unsafe fn blit_named_framebuffer(
    read_framebuffer: GlFramebuffer,
    draw_framebuffer: GlFramebuffer,
    src_x0: i32,
    src_y0: i32,
    src_x1: i32,
    src_y1: i32,
    dst_x0: i32,
    dst_y0: i32,
    dst_x1: i32,
    dst_y1: i32,
    mask: u32,
    filter: u32,
) {
    native_gl::glBlitNamedFramebuffer(
        read_framebuffer.0.get(),
        draw_framebuffer.0.get(),
        src_x0,
        src_y0,
        src_x1,
        src_y1,
        dst_x0,
        dst_y0,
        dst_x1,
        dst_y1,
        mask,
        filter,
    );
}

pub unsafe fn blit_from_default_framebuffer(
    draw_framebuffer: GlFramebuffer,
    src_x0: i32,
    src_y0: i32,
    src_x1: i32,
    src_y1: i32,
    dst_x0: i32,
    dst_y0: i32,
    dst_x1: i32,
    dst_y1: i32,
    mask: u32,
    filter: u32,
) {
    native_gl::glBlitNamedFramebuffer(
        0,
        draw_framebuffer.0.get(),
        src_x0,
        src_y0,
        src_x1,
        src_y1,
        dst_x0,
        dst_y0,
        dst_x1,
        dst_y1,
        mask,
        filter,
    );
}

pub unsafe fn blit_to_default_framebuffer(
    read_framebuffer: GlFramebuffer,
    src_x0: i32,
    src_y0: i32,
    src_x1: i32,
    src_y1: i32,
    dst_x0: i32,
    dst_y0: i32,
    dst_x1: i32,
    dst_y1: i32,
    mask: u32,
    filter: u32,
) {
    native_gl::glBlitNamedFramebuffer(
        read_framebuffer.0.get(),
        0,
        src_x0,
        src_y0,
        src_x1,
        src_y1,
        dst_x0,
        dst_y0,
        dst_x1,
        dst_y1,
        mask,
        filter,
    );
}

pub unsafe fn create_vertex_array() -> Result<GlVertexArray, String> {
    let mut vertex_array = 0;
    native_gl::glGenVertexArrays(1, &mut vertex_array);
    Ok(GlVertexArray(non_zero_u32_gl_name(vertex_array)))
}

pub unsafe fn create_named_vertex_array() -> Result<GlVertexArray, String> {
    let mut vertex_array = 0;
    native_gl::glCreateVertexArrays(1, &mut vertex_array);
    Ok(GlVertexArray(non_zero_u32_gl_name(vertex_array)))
}

pub unsafe fn delete_vertex_array(vertex_array: GlVertexArray) {
    native_gl::glDeleteVertexArrays(1, &vertex_array.0.get());
}

pub unsafe fn bind_vertex_array(vertex_array: Option<GlVertexArray>) {
    native_gl::glBindVertexArray(vertex_array.map(|va| va.0.get()).unwrap_or(0));
}

pub unsafe fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    native_gl::glClearColor(red, green, blue, alpha);
}

pub unsafe fn supports_f64_precision() -> bool {
    true
}

pub unsafe fn clear_depth_f64(depth: f64) {
    native_gl::glClearDepth(depth);
}

pub unsafe fn clear_depth_f32(depth: f32) {
    native_gl::glClearDepthf(depth);
}

pub unsafe fn clear_stencil(stencil: i32) {
    native_gl::glClearStencil(stencil);
}

pub unsafe fn clear(mask: u32) {
    native_gl::glClear(mask);
}

pub unsafe fn patch_parameter_i32(parameter: u32, value: i32) {
    native_gl::glPatchParameteri(parameter, value);
}

pub unsafe fn pixel_store_i32(parameter: u32, value: i32) {
    native_gl::glPixelStorei(parameter, value);
}

pub unsafe fn pixel_store_bool(parameter: u32, value: bool) {
    native_gl::glPixelStorei(parameter, value as i32);
}

pub unsafe fn bind_frag_data_location(program: GlProgram, color_number: u32, name: &str) {
    native_gl::glBindFragDataLocation(
        program.0.get(),
        color_number,
        name.as_ptr() as *const native_gl::GLchar,
    );
}

pub unsafe fn buffer_data_size(target: u32, size: i32, usage: u32) {
    native_gl::glBufferData(target, size as isize, std::ptr::null(), usage);
}

pub unsafe fn buffer_data_u8_slice(target: u32, data: &[u8], usage: u32) {
    native_gl::glBufferData(
        target,
        data.len() as isize,
        data.as_ptr() as *const std::ffi::c_void,
        usage,
    );
}

pub unsafe fn named_buffer_data_u8_slice(buffer: GlBuffer, data: &[u8], usage: u32) {
    native_gl::glNamedBufferData(
        buffer.0.get(),
        data.len() as isize,
        data.as_ptr() as *const std::ffi::c_void,
        usage,
    );
}

pub unsafe fn buffer_sub_data_u8_slice(target: u32, offset: i32, src_data: &[u8]) {
    native_gl::glBufferSubData(
        target,
        offset as isize,
        src_data.len() as isize,
        src_data.as_ptr() as *const std::ffi::c_void,
    );
}

pub unsafe fn get_buffer_sub_data(target: u32, offset: i32, dst_data: &mut [u8]) {
    native_gl::glGetBufferSubData(
        target,
        offset as isize,
        dst_data.len() as isize,
        dst_data.as_mut_ptr() as *mut std::ffi::c_void,
    );
}

pub unsafe fn buffer_storage(target: u32, size: i32, data: Option<&[u8]>, flags: u32) {
    let size = size as isize;
    let data = data.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void;
    if native_gl::glBufferStorage_is_loaded() {
        native_gl::glBufferStorage(target, size, data, flags);
    } else {
        native_gl::glBufferStorageEXT(target, size, data, flags);
    }
}

pub unsafe fn named_buffer_storage(buffer: GlBuffer, size: i32, data: Option<&[u8]>, flags: u32) {
    let size = size as isize;
    let data = data.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void;
    native_gl::glNamedBufferStorage(buffer.0.get(), size, data, flags);
}

pub unsafe fn check_framebuffer_status(target: u32) -> u32 {
    native_gl::glCheckFramebufferStatus(target)
}

pub unsafe fn check_named_framebuffer_status(framebuffer: GlFramebuffer, target: u32) -> u32 {
    native_gl::glCheckNamedFramebufferStatus(framebuffer.0.get(), target)
}

pub unsafe fn clear_buffer_i32_slice(target: u32, draw_buffer: u32, values: &[i32]) {
    native_gl::glClearBufferiv(target, draw_buffer as i32, values.as_ptr());
}

pub unsafe fn clear_buffer_u32_slice(target: u32, draw_buffer: u32, values: &[u32]) {
    native_gl::glClearBufferuiv(target, draw_buffer as i32, values.as_ptr());
}

pub unsafe fn clear_buffer_f32_slice(target: u32, draw_buffer: u32, values: &[f32]) {
    native_gl::glClearBufferfv(target, draw_buffer as i32, values.as_ptr());
}

pub unsafe fn clear_buffer_depth_stencil(target: u32, draw_buffer: u32, depth: f32, stencil: i32) {
    native_gl::glClearBufferfi(target, draw_buffer as i32, depth, stencil);
}

pub unsafe fn clear_named_framebuffer_f32(
    framebuffer: GlFramebuffer,
    buffer: u32,
    draw_buffer: i32,
    value: f32,
) {
    native_gl::glClearNamedFramebufferfv(
        framebuffer.0.get(),
        buffer,
        draw_buffer,
        &value as *const _,
    );
}

pub unsafe fn clear_named_framebuffer_i32(
    framebuffer: GlFramebuffer,
    buffer: u32,
    draw_buffer: i32,
    value: i32,
) {
    native_gl::glClearNamedFramebufferiv(
        framebuffer.0.get(),
        buffer,
        draw_buffer,
        &value as *const _,
    );
}

pub unsafe fn clear_named_framebuffer_u32(
    framebuffer: GlFramebuffer,
    buffer: u32,
    draw_buffer: i32,
    value: u32,
) {
    native_gl::glClearNamedFramebufferuiv(
        framebuffer.0.get(),
        buffer,
        draw_buffer,
        &value as *const _,
    );
}

pub unsafe fn client_wait_sync(fence: GlFence, flags: u32, timeout: i32) -> u32 {
    native_gl::glClientWaitSync(fence.0, flags, timeout as u64)
}

pub unsafe fn wait_sync(fence: GlFence, flags: u32, timeout: u64) {
    native_gl::glWaitSync(fence.0, flags, timeout)
}

pub unsafe fn copy_buffer_sub_data(
    src_target: u32,
    dst_target: u32,
    src_offset: i32,
    dst_offset: i32,
    size: i32,
) {
    native_gl::glCopyBufferSubData(
        src_target,
        dst_target,
        src_offset as isize,
        dst_offset as isize,
        size as isize,
    );
}

pub unsafe fn copy_image_sub_data(
    src_name: GlTexture,
    src_target: u32,
    src_level: i32,
    src_x: i32,
    src_y: i32,
    src_z: i32,
    dst_name: GlTexture,
    dst_target: u32,
    dst_level: i32,
    dst_x: i32,
    dst_y: i32,
    dst_z: i32,
    src_width: i32,
    src_height: i32,
    src_depth: i32,
) {
    native_gl::glCopyImageSubData(
        src_name.0.get(),
        src_target,
        src_level,
        src_x,
        src_y,
        src_z,
        dst_name.0.get(),
        dst_target,
        dst_level,
        dst_x,
        dst_y,
        dst_z,
        src_width,
        src_height,
        src_depth,
    );
}

pub unsafe fn copy_tex_image_2d(
    target: u32,
    level: i32,
    internal_format: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    border: i32,
) {
    native_gl::glCopyTexImage2D(target, level, internal_format, x, y, width, height, border);
}

pub unsafe fn copy_tex_sub_image_2d(
    target: u32,
    level: i32,
    x_offset: i32,
    y_offset: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    native_gl::glCopyTexSubImage2D(target, level, x_offset, y_offset, x, y, width, height);
}

pub unsafe fn copy_tex_sub_image_3d(
    target: u32,
    level: i32,
    x_offset: i32,
    y_offset: i32,
    z_offset: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    native_gl::glCopyTexSubImage3D(
        target, level, x_offset, y_offset, z_offset, x, y, width, height,
    );
}

pub unsafe fn delete_buffer(buffer: GlBuffer) {
    native_gl::glDeleteBuffers(1, &buffer.0.get());
}

pub unsafe fn delete_framebuffer(framebuffer: GlFramebuffer) {
    native_gl::glDeleteFramebuffers(1, &framebuffer.0.get());
}

pub unsafe fn delete_query(query: GlQuery) {
    native_gl::glDeleteQueries(1, &query.0.get());
}

pub unsafe fn delete_renderbuffer(renderbuffer: GlRenderbuffer) {
    native_gl::glDeleteRenderbuffers(1, &renderbuffer.0.get());
}

pub unsafe fn delete_sampler(sampler: GlSampler) {
    native_gl::glDeleteSamplers(1, &sampler.0.get());
}

pub unsafe fn delete_sync(fence: GlFence) {
    native_gl::glDeleteSync(fence.0);
}

pub unsafe fn delete_texture(texture: GlTexture) {
    native_gl::glDeleteTextures(1, &texture.0.get());
}

pub unsafe fn disable(parameter: u32) {
    native_gl::glDisable(parameter);
}

pub unsafe fn disable_draw_buffer(parameter: u32, draw_buffer: u32) {
    native_gl::glDisablei(parameter, draw_buffer);
}

pub unsafe fn disable_vertex_attrib_array(index: u32) {
    native_gl::glDisableVertexAttribArray(index);
}

pub unsafe fn dispatch_compute(groups_x: u32, groups_y: u32, groups_z: u32) {
    native_gl::glDispatchCompute(groups_x, groups_y, groups_z);
}

pub unsafe fn dispatch_compute_indirect(offset: i32) {
    native_gl::glDispatchComputeIndirect(offset as isize);
}

pub unsafe fn draw_arrays(mode: u32, first: i32, count: i32) {
    native_gl::glDrawArrays(mode as u32, first, count);
}

pub unsafe fn draw_arrays_instanced(mode: u32, first: i32, count: i32, instance_count: i32) {
    native_gl::glDrawArraysInstanced(mode as u32, first, count, instance_count);
}

pub unsafe fn draw_arrays_instanced_base_instance(
    mode: u32,
    first: i32,
    count: i32,
    instance_count: i32,
    base_instance: u32,
) {
    native_gl::glDrawArraysInstancedBaseInstance(
        mode as u32,
        first,
        count,
        instance_count,
        base_instance,
    );
}

pub unsafe fn draw_arrays_indirect_offset(mode: u32, offset: i32) {
    native_gl::glDrawArraysIndirect(mode, offset as *const std::ffi::c_void);
}

pub unsafe fn draw_buffer(draw_buffer: u32) {
    native_gl::glDrawBuffer(draw_buffer);
}

pub unsafe fn draw_buffers(buffers: &[u32]) {
    native_gl::glDrawBuffers(buffers.len() as i32, buffers.as_ptr());
}

pub unsafe fn draw_elements(mode: u32, count: i32, element_type: u32, offset: i32) {
    native_gl::glDrawElements(
        mode as u32,
        count,
        element_type as u32,
        offset as *const std::ffi::c_void,
    );
}

pub unsafe fn draw_elements_base_vertex(
    mode: u32,
    count: i32,
    element_type: u32,
    offset: i32,
    base_vertex: i32,
) {
    native_gl::glDrawElementsBaseVertex(
        mode as u32,
        count,
        element_type as u32,
        offset as *const std::ffi::c_void,
        base_vertex,
    );
}

pub unsafe fn draw_elements_instanced(
    mode: u32,
    count: i32,
    element_type: u32,
    offset: i32,
    instance_count: i32,
) {
    native_gl::glDrawElementsInstanced(
        mode as u32,
        count,
        element_type as u32,
        offset as *const std::ffi::c_void,
        instance_count,
    );
}

pub unsafe fn draw_elements_instanced_base_vertex(
    mode: u32,
    count: i32,
    element_type: u32,
    offset: i32,
    instance_count: i32,
    base_vertex: i32,
) {
    native_gl::glDrawElementsInstancedBaseVertex(
        mode as u32,
        count,
        element_type as u32,
        offset as *const std::ffi::c_void,
        instance_count,
        base_vertex,
    );
}

pub unsafe fn draw_elements_instanced_base_vertex_base_instance(
    mode: u32,
    count: i32,
    element_type: u32,
    offset: i32,
    instance_count: i32,
    base_vertex: i32,
    base_instance: u32,
) {
    native_gl::glDrawElementsInstancedBaseVertexBaseInstance(
        mode as u32,
        count,
        element_type as u32,
        offset as *const std::ffi::c_void,
        instance_count,
        base_vertex,
        base_instance,
    );
}

pub unsafe fn draw_elements_indirect_offset(mode: u32, element_type: u32, offset: i32) {
    native_gl::glDrawElementsIndirect(mode, element_type, offset as *const std::ffi::c_void);
}

pub unsafe fn multi_draw_arrays(mode: u32, first: &[i32], count: &[i32]) {
    native_gl::glMultiDrawArrays(
        mode,
        first.as_ptr(),
        count.as_ptr(),
        first.len().min(count.len()) as i32,
    );
}

pub unsafe fn multi_draw_arrays_indirect_offset(
    mode: u32,
    offset: i32,
    draw_count: i32,
    stride: i32,
) {
    native_gl::glMultiDrawArraysIndirect(
        mode,
        offset as *const std::ffi::c_void,
        draw_count,
        stride,
    );
}

pub unsafe fn multi_draw_elements(mode: u32, count: &[i32], element_type: u32, offset: &[i32]) {
    native_gl::glMultiDrawElements(
        mode,
        count.as_ptr(),
        element_type,
        offset.as_ptr() as *const *const std::ffi::c_void,
        count.len().min(offset.len()) as i32,
    );
}

pub unsafe fn multi_draw_elements_base_vertex(
    mode: u32,
    count: &[i32],
    element_type: u32,
    offset: &[i32],
    base_vertex: &[i32],
) {
    native_gl::glMultiDrawElementsBaseVertex(
        mode,
        count.as_ptr(),
        element_type,
        offset.as_ptr() as *const *const std::ffi::c_void,
        count.len().min(offset.len()).min(base_vertex.len()) as i32,
        base_vertex.as_ptr(),
    );
}

pub unsafe fn multi_draw_elements_indirect_offset(
    mode: u32,
    element_type: u32,
    offset: i32,
    draw_count: i32,
    stride: i32,
) {
    native_gl::glMultiDrawElementsIndirect(
        mode,
        element_type,
        offset as *const std::ffi::c_void,
        draw_count,
        stride,
    );
}

pub unsafe fn enable(parameter: u32) {
    native_gl::glEnable(parameter);
}

pub unsafe fn is_enabled(parameter: u32) -> bool {
    native_gl::glIsEnabled(parameter) != 0
}

pub unsafe fn enable_draw_buffer(parameter: u32, draw_buffer: u32) {
    native_gl::glEnablei(parameter, draw_buffer);
}

pub unsafe fn enable_vertex_array_attrib(vao: GlVertexArray, index: u32) {
    native_gl::glEnableVertexArrayAttrib(vao.0.get(), index);
}

pub unsafe fn enable_vertex_attrib_array(index: u32) {
    native_gl::glEnableVertexAttribArray(index);
}

pub unsafe fn flush() {
    native_gl::glFlush();
}

pub unsafe fn framebuffer_renderbuffer(
    target: u32,
    attachment: u32,
    renderbuffer_target: u32,
    renderbuffer: Option<GlRenderbuffer>,
) {
    native_gl::glFramebufferRenderbuffer(
        target,
        attachment,
        renderbuffer_target,
        renderbuffer.map(|rb| rb.0.get()).unwrap_or(0),
    );
}

pub unsafe fn framebuffer_texture(
    target: u32,
    attachment: u32,
    texture: Option<GlTexture>,
    level: i32,
) {
    native_gl::glFramebufferTexture(
        target,
        attachment,
        texture.map(|t| t.0.get()).unwrap_or(0),
        level,
    );
}

pub unsafe fn framebuffer_texture_2d(
    target: u32,
    attachment: u32,
    texture_target: u32,
    texture: Option<GlTexture>,
    level: i32,
) {
    native_gl::glFramebufferTexture2D(
        target,
        attachment,
        texture_target,
        texture.map(|t| t.0.get()).unwrap_or(0),
        level,
    );
}

pub unsafe fn framebuffer_texture_3d(
    target: u32,
    attachment: u32,
    texture_target: u32,
    texture: Option<GlTexture>,
    level: i32,
    layer: i32,
) {
    native_gl::glFramebufferTexture3D(
        target,
        attachment,
        texture_target,
        texture.map(|t| t.0.get()).unwrap_or(0),
        level,
        layer,
    );
}

pub unsafe fn framebuffer_texture_layer(
    target: u32,
    attachment: u32,
    texture: Option<GlTexture>,
    level: i32,
    layer: i32,
) {
    native_gl::glFramebufferTextureLayer(
        target,
        attachment,
        texture.map(|t| t.0.get()).unwrap_or(0),
        level,
        layer,
    );
}

pub unsafe fn named_framebuffer_draw_buffer(framebuffer: GlFramebuffer, color_buffer: u32) {
    native_gl::glNamedFramebufferDrawBuffer(framebuffer.0.get(), color_buffer)
}

pub unsafe fn named_framebuffer_read_buffer(framebuffer: GlFramebuffer, source: u32) {
    native_gl::glNamedFramebufferReadBuffer(framebuffer.0.get(), source)
}

pub unsafe fn named_framebuffer_parameter_i32(
    framebuffer: GlFramebuffer,
    parameter_name: u32,
    value: i32,
) {
    native_gl::glNamedFramebufferParameteri(framebuffer.0.get(), parameter_name, value)
}

pub unsafe fn named_framebuffer_renderbuffer(
    framebuffer: GlFramebuffer,
    attachment: u32,
    renderbuffer_target: u32,
    renderbuffer: self::GlRenderbuffer,
) {
    native_gl::glNamedFramebufferRenderbuffer(
        framebuffer.0.get(),
        attachment,
        renderbuffer_target,
        renderbuffer.0.get(),
    )
}

pub unsafe fn named_framebuffer_texture(
    framebuffer: GlFramebuffer,
    attachment: u32,
    texture: GlTexture,
    level: i32,
) {
    native_gl::glNamedFramebufferTexture(framebuffer.0.get(), attachment, texture.0.get(), level)
}

pub unsafe fn named_renderbuffer_storage(
    renderbuffer: self::GlRenderbuffer,
    internal_format: u32,
    width: i32,
    height: i32,
) {
    native_gl::glNamedRenderbufferStorage(renderbuffer.0.get(), internal_format, width, height)
}

pub unsafe fn named_renderbuffer_storage_multisample(
    renderbuffer: self::GlRenderbuffer,
    samples: i32,
    internal_format: u32,
    width: i32,
    height: i32,
) {
    native_gl::glNamedRenderbufferStorageMultisample(
        renderbuffer.0.get(),
        samples,
        internal_format,
        width,
        height,
    )
}

pub unsafe fn front_face(value: u32) {
    native_gl::glFrontFace(value as u32);
}

pub unsafe fn get_error() -> u32 {
    native_gl::glGetError()
}

pub unsafe fn get_tex_parameter_i32(target: u32, parameter: u32) -> i32 {
    let mut value = 0;
    native_gl::glGetTexParameteriv(target, parameter, &mut value);
    value
}

pub unsafe fn get_texture_handle(texture: GlTexture) -> GlTextureHandle {
    GlTextureHandle(non_zero_u64_gl_name(native_gl::glGetTextureHandleARB(
        texture.0.get(),
    )))
}

pub unsafe fn make_texture_handle_non_resident(texture_handle: GlTextureHandle) {
    native_gl::glMakeTextureHandleNonResidentARB(texture_handle.0.get())
}

pub unsafe fn make_texture_handle_resident(texture_handle: GlTextureHandle) {
    native_gl::glMakeTextureHandleResidentARB(texture_handle.0.get())
}

pub unsafe fn get_buffer_parameter_i32(target: u32, parameter: u32) -> i32 {
    let mut value = 0;
    native_gl::glGetBufferParameteriv(target, parameter, &mut value);
    value
}

pub unsafe fn get_parameter_i32(parameter: u32) -> i32 {
    let mut value = 0;
    native_gl::glGetIntegerv(parameter, &mut value);
    value
}

pub unsafe fn get_parameter_i64(parameter: u32) -> i64 {
    let mut value = 0;
    native_gl::glGetInteger64v(parameter, &mut value);
    value
}

pub unsafe fn get_parameter_i32_slice(parameter: u32, out: &mut [i32]) {
    native_gl::glGetIntegerv(parameter, &mut out[0]);
}

pub unsafe fn get_parameter_f32(parameter: u32) -> f32 {
    let mut value: f32 = 0.0;
    native_gl::glGetFloatv(parameter, &mut value);
    value
}

pub unsafe fn get_parameter_f32_slice(parameter: u32, out: &mut [f32]) {
    native_gl::glGetFloatv(parameter, &mut out[0]);
}

pub unsafe fn get_parameter_indexed_i32(parameter: u32, index: u32) -> i32 {
    let mut value = 0;
    native_gl::glGetIntegeri_v(parameter, index, &mut value);
    value
}

pub unsafe fn get_parameter_indexed_string(parameter: u32, index: u32) -> String {
    let raw_ptr = native_gl::glGetStringi(parameter, index);
    std::ffi::CStr::from_ptr(raw_ptr as *const native_gl::GLchar)
        .to_str()
        .unwrap()
        .to_owned()
}

pub unsafe fn get_parameter_string(parameter: u32) -> String {
    let raw_ptr = native_gl::glGetString(parameter);
    std::ffi::CStr::from_ptr(raw_ptr as *const native_gl::GLchar)
        .to_str()
        .unwrap()
        .to_owned()
}

pub unsafe fn get_uniform_location(program: GlProgram, name: &str) -> Option<GlUniformLocation> {
    let name = CString::new(name).unwrap();
    let uniform_location =
        native_gl::glGetUniformLocation(program.0.get(), name.as_ptr() as *const native_gl::GLchar);
    if uniform_location < 0 {
        None
    } else {
        Some(GlUniformLocation(uniform_location as u32))
    }
}

pub unsafe fn get_attrib_location(program: GlProgram, name: &str) -> Option<u32> {
    let name = CString::new(name).unwrap();
    let attrib_location =
        native_gl::glGetAttribLocation(program.0.get(), name.as_ptr() as *const native_gl::GLchar);
    if attrib_location < 0 {
        None
    } else {
        Some(attrib_location as u32)
    }
}

pub unsafe fn bind_attrib_location(program: GlProgram, index: u32, name: &str) {
    let name = CString::new(name).unwrap();
    native_gl::glBindAttribLocation(
        program.0.get(),
        index,
        name.as_ptr() as *const native_gl::GLchar,
    );
}

pub unsafe fn get_active_attributes(program: GlProgram) -> u32 {
    let mut count = 0;
    native_gl::glGetProgramiv(program.0.get(), ACTIVE_ATTRIBUTES, &mut count);
    count as u32
}

pub unsafe fn get_active_attribute(program: GlProgram, index: u32) -> Option<ActiveAttribute> {
    let mut attribute_max_size = 0;
    native_gl::glGetProgramiv(
        program.0.get(),
        ACTIVE_ATTRIBUTE_MAX_LENGTH,
        &mut attribute_max_size,
    );
    let mut name = String::with_capacity(attribute_max_size as usize);
    name.extend(std::iter::repeat('\0').take(attribute_max_size as usize));
    let mut length = 0;
    let mut size = 0;
    let mut atype = 0;
    native_gl::glGetActiveAttrib(
        program.0.get(),
        index,
        attribute_max_size,
        &mut length,
        &mut size,
        &mut atype,
        name.as_ptr() as *mut native_gl::GLchar,
    );

    name.truncate(length as usize);

    Some(ActiveAttribute { name, size, atype })
}

pub unsafe fn get_sync_status(fence: GlFence) -> u32 {
    let mut len = 0;
    let mut values = [UNSIGNALED as i32];
    native_gl::glGetSynciv(
        fence.0,
        SYNC_STATUS,
        values.len() as i32,
        &mut len,
        values.as_mut_ptr(),
    );
    values[0] as u32
}

pub unsafe fn is_sync(fence: GlFence) -> bool {
    1 == native_gl::glIsSync(fence.0)
}

pub unsafe fn renderbuffer_storage(target: u32, internal_format: u32, width: i32, height: i32) {
    native_gl::glRenderbufferStorage(target, internal_format, width, height);
}

pub unsafe fn renderbuffer_storage_multisample(
    target: u32,
    samples: i32,
    internal_format: u32,
    width: i32,
    height: i32,
) {
    native_gl::glRenderbufferStorageMultisample(target, samples, internal_format, width, height);
}

pub unsafe fn sampler_parameter_f32(sampler: GlSampler, name: u32, value: f32) {
    native_gl::glSamplerParameterf(sampler.0.get(), name, value);
}

pub unsafe fn sampler_parameter_f32_slice(sampler: GlSampler, name: u32, value: &[f32]) {
    native_gl::glSamplerParameterfv(sampler.0.get(), name, value.as_ptr());
}

pub unsafe fn sampler_parameter_i32(sampler: GlSampler, name: u32, value: i32) {
    native_gl::glSamplerParameteri(sampler.0.get(), name, value);
}

pub unsafe fn generate_mipmap(target: u32) {
    native_gl::glGenerateMipmap(target);
}

pub unsafe fn generate_texture_mipmap(texture: GlTexture) {
    native_gl::glGenerateTextureMipmap(texture.0.get());
}

pub unsafe fn tex_image_1d(
    target: u32,
    level: i32,
    internal_format: i32,
    width: i32,
    border: i32,
    format: u32,
    ty: u32,
    pixels: Option<&[u8]>,
) {
    native_gl::glTexImage1D(
        target,
        level,
        internal_format,
        width,
        border,
        format,
        ty,
        pixels.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void,
    );
}

pub unsafe fn compressed_tex_image_1d(
    target: u32,
    level: i32,
    internal_format: i32,
    width: i32,
    border: i32,
    image_size: i32,
    pixels: &[u8],
) {
    native_gl::glCompressedTexImage1D(
        target,
        level,
        internal_format as u32,
        width,
        border,
        image_size,
        pixels.as_ptr() as *const std::ffi::c_void,
    );
}

pub unsafe fn tex_image_2d(
    target: u32,
    level: i32,
    internal_format: i32,
    width: i32,
    height: i32,
    border: i32,
    format: u32,
    ty: u32,
    pixels: Option<&[u8]>,
) {
    native_gl::glTexImage2D(
        target,
        level,
        internal_format,
        width,
        height,
        border,
        format,
        ty,
        pixels.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void,
    );
}

pub unsafe fn tex_image_2d_multisample(
    target: u32,
    samples: i32,
    internal_format: i32,
    width: i32,
    height: i32,
    fixed_sample_locations: bool,
) {
    native_gl::glTexImage2DMultisample(
        target,
        samples,
        internal_format as u32,
        width,
        height,
        if fixed_sample_locations { 1 } else { 0 },
    );
}

pub unsafe fn compressed_tex_image_2d(
    target: u32,
    level: i32,
    internal_format: i32,
    width: i32,
    height: i32,
    border: i32,
    image_size: i32,
    pixels: &[u8],
) {
    native_gl::glCompressedTexImage2D(
        target,
        level,
        internal_format as u32,
        width,
        height,
        border,
        image_size,
        pixels.as_ptr() as *const std::ffi::c_void,
    );
}

pub unsafe fn tex_image_3d(
    target: u32,
    level: i32,
    internal_format: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    ty: u32,
    pixels: Option<&[u8]>,
) {
    native_gl::glTexImage3D(
        target,
        level,
        internal_format,
        width,
        height,
        depth,
        border,
        format,
        ty,
        pixels.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void,
    );
}

pub unsafe fn compressed_tex_image_3d(
    target: u32,
    level: i32,
    internal_format: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    image_size: i32,
    pixels: &[u8],
) {
    native_gl::glCompressedTexImage3D(
        target,
        level,
        internal_format as u32,
        width,
        height,
        depth,
        border,
        image_size,
        pixels.as_ptr() as *const std::ffi::c_void,
    );
}

pub unsafe fn tex_storage_1d(target: u32, levels: i32, internal_format: u32, width: i32) {
    native_gl::glTexStorage1D(target, levels, internal_format, width);
}

pub unsafe fn tex_storage_2d(
    target: u32,
    levels: i32,
    internal_format: u32,
    width: i32,
    height: i32,
) {
    native_gl::glTexStorage2D(target, levels, internal_format, width, height);
}

pub unsafe fn texture_storage_2d(
    texture: GlTexture,
    levels: i32,
    internal_format: u32,
    width: i32,
    height: i32,
) {
    native_gl::glTextureStorage2D(texture.0.get(), levels, internal_format, width, height);
}

pub unsafe fn tex_storage_2d_multisample(
    target: u32,
    samples: i32,
    internal_format: u32,
    width: i32,
    height: i32,
    fixed_sample_locations: bool,
) {
    native_gl::glTexStorage2DMultisample(
        target,
        samples,
        internal_format,
        width,
        height,
        if fixed_sample_locations { 1 } else { 0 },
    );
}

pub unsafe fn texture_storage_2d_multisample(
    texture: GlTexture,
    samples: i32,
    internal_format: u32,
    width: i32,
    height: i32,
    fixed_sample_locations: bool,
) {
    native_gl::glTextureStorage2DMultisample(
        texture.0.get(),
        samples,
        internal_format,
        width,
        height,
        if fixed_sample_locations { 1 } else { 0 },
    );
}

pub unsafe fn tex_storage_3d(
    target: u32,
    levels: i32,
    internal_format: u32,
    width: i32,
    height: i32,
    depth: i32,
) {
    native_gl::glTexStorage3D(target, levels, internal_format, width, height, depth);
}

pub unsafe fn texture_storage_3d(
    texture: GlTexture,
    levels: i32,
    internal_format: u32,
    width: i32,
    height: i32,
    depth: i32,
) {
    native_gl::glTextureStorage3D(
        texture.0.get(),
        levels,
        internal_format,
        width,
        height,
        depth,
    );
}

pub unsafe fn get_uniform_i32(program: GlProgram, location: &GlUniformLocation, v: &mut [i32]) {
    native_gl::glGetUniformiv(
        program.0.get() as u32,
        location.0 as i32,
        v.as_mut_ptr() as *mut i32,
    )
}

pub unsafe fn get_uniform_f32(program: GlProgram, location: &GlUniformLocation, v: &mut [f32]) {
    native_gl::glGetUniformfv(
        program.0.get() as u32,
        location.0 as i32,
        v.as_mut_ptr() as *mut f32,
    )
}

pub unsafe fn uniform_1_i32(location: Option<&GlUniformLocation>, x: i32) {
    if let Some(loc) = location {
        native_gl::glUniform1i(loc.0 as i32, x);
    }
}

pub unsafe fn uniform_2_i32(location: Option<&GlUniformLocation>, x: i32, y: i32) {
    if let Some(loc) = location {
        native_gl::glUniform2i(loc.0 as i32, x, y);
    }
}

pub unsafe fn uniform_3_i32(location: Option<&GlUniformLocation>, x: i32, y: i32, z: i32) {
    if let Some(loc) = location {
        native_gl::glUniform3i(loc.0 as i32, x, y, z);
    }
}

pub unsafe fn uniform_4_i32(location: Option<&GlUniformLocation>, x: i32, y: i32, z: i32, w: i32) {
    if let Some(loc) = location {
        native_gl::glUniform4i(loc.0 as i32, x, y, z, w);
    }
}

pub unsafe fn uniform_1_i32_slice(location: Option<&GlUniformLocation>, v: &[i32]) {
    if let Some(loc) = location {
        native_gl::glUniform1iv(loc.0 as i32, v.len() as i32, v.as_ptr());
    }
}

pub unsafe fn uniform_2_i32_slice(location: Option<&GlUniformLocation>, v: &[i32]) {
    if let Some(loc) = location {
        native_gl::glUniform2iv(loc.0 as i32, v.len() as i32 / 2, v.as_ptr());
    }
}

pub unsafe fn uniform_3_i32_slice(location: Option<&GlUniformLocation>, v: &[i32]) {
    if let Some(loc) = location {
        native_gl::glUniform3iv(loc.0 as i32, v.len() as i32 / 3, v.as_ptr());
    }
}

pub unsafe fn uniform_4_i32_slice(location: Option<&GlUniformLocation>, v: &[i32]) {
    if let Some(loc) = location {
        native_gl::glUniform4iv(loc.0 as i32, v.len() as i32 / 4, v.as_ptr());
    }
}

pub unsafe fn uniform_1_u32(location: Option<&GlUniformLocation>, x: u32) {
    if let Some(loc) = location {
        native_gl::glUniform1ui(loc.0 as i32, x);
    }
}

pub unsafe fn uniform_2_u32(location: Option<&GlUniformLocation>, x: u32, y: u32) {
    if let Some(loc) = location {
        native_gl::glUniform2ui(loc.0 as i32, x, y);
    }
}

pub unsafe fn uniform_3_u32(location: Option<&GlUniformLocation>, x: u32, y: u32, z: u32) {
    if let Some(loc) = location {
        native_gl::glUniform3ui(loc.0 as i32, x, y, z);
    }
}

pub unsafe fn uniform_4_u32(location: Option<&GlUniformLocation>, x: u32, y: u32, z: u32, w: u32) {
    if let Some(loc) = location {
        native_gl::glUniform4ui(loc.0 as i32, x, y, z, w);
    }
}

pub unsafe fn uniform_1_u32_slice(location: Option<&GlUniformLocation>, v: &[u32]) {
    if let Some(loc) = location {
        native_gl::glUniform1uiv(loc.0 as i32, v.len() as i32, v.as_ptr());
    }
}

pub unsafe fn uniform_2_u32_slice(location: Option<&GlUniformLocation>, v: &[u32]) {
    if let Some(loc) = location {
        native_gl::glUniform2uiv(loc.0 as i32, v.len() as i32 / 2, v.as_ptr());
    }
}

pub unsafe fn uniform_3_u32_slice(location: Option<&GlUniformLocation>, v: &[u32]) {
    if let Some(loc) = location {
        native_gl::glUniform3uiv(loc.0 as i32, v.len() as i32 / 3, v.as_ptr());
    }
}

pub unsafe fn uniform_4_u32_slice(location: Option<&GlUniformLocation>, v: &[u32]) {
    if let Some(loc) = location {
        native_gl::glUniform4uiv(loc.0 as i32, v.len() as i32 / 4, v.as_ptr());
    }
}

pub unsafe fn uniform_1_f32(location: Option<&GlUniformLocation>, x: f32) {
    if let Some(loc) = location {
        native_gl::glUniform1f(loc.0 as i32, x);
    }
}

pub unsafe fn uniform_2_f32(location: Option<&GlUniformLocation>, x: f32, y: f32) {
    if let Some(loc) = location {
        native_gl::glUniform2f(loc.0 as i32, x, y);
    }
}

pub unsafe fn uniform_3_f32(location: Option<&GlUniformLocation>, x: f32, y: f32, z: f32) {
    if let Some(loc) = location {
        native_gl::glUniform3f(loc.0 as i32, x, y, z);
    }
}

pub unsafe fn uniform_4_f32(location: Option<&GlUniformLocation>, x: f32, y: f32, z: f32, w: f32) {
    if let Some(loc) = location {
        native_gl::glUniform4f(loc.0 as i32, x, y, z, w);
    }
}

pub unsafe fn uniform_1_f32_slice(location: Option<&GlUniformLocation>, v: &[f32]) {
    if let Some(loc) = location {
        native_gl::glUniform1fv(loc.0 as i32, v.len() as i32, v.as_ptr());
    }
}

pub unsafe fn uniform_2_f32_slice(location: Option<&GlUniformLocation>, v: &[f32]) {
    if let Some(loc) = location {
        native_gl::glUniform2fv(loc.0 as i32, v.len() as i32 / 2, v.as_ptr());
    }
}

pub unsafe fn uniform_3_f32_slice(location: Option<&GlUniformLocation>, v: &[f32]) {
    if let Some(loc) = location {
        native_gl::glUniform3fv(loc.0 as i32, v.len() as i32 / 3, v.as_ptr());
    }
}

pub unsafe fn uniform_4_f32_slice(location: Option<&GlUniformLocation>, v: &[f32]) {
    if let Some(loc) = location {
        native_gl::glUniform4fv(loc.0 as i32, v.len() as i32 / 4, v.as_ptr());
    }
}

pub unsafe fn uniform_matrix_2_f32_slice(
    location: Option<&GlUniformLocation>,
    transpose: bool,
    v: &[f32],
) {
    if let Some(loc) = location {
        native_gl::glUniformMatrix2fv(
            loc.0 as i32,
            v.len() as i32 / 4,
            transpose as u8,
            v.as_ptr(),
        );
    }
}

pub unsafe fn uniform_matrix_3_f32_slice(
    location: Option<&GlUniformLocation>,
    transpose: bool,
    v: &[f32],
) {
    if let Some(loc) = location {
        native_gl::glUniformMatrix3fv(
            loc.0 as i32,
            v.len() as i32 / 9,
            transpose as u8,
            v.as_ptr(),
        );
    }
}

pub unsafe fn uniform_matrix_4_f32_slice(
    location: Option<&GlUniformLocation>,
    transpose: bool,
    v: &[f32],
) {
    if let Some(loc) = location {
        native_gl::glUniformMatrix4fv(
            loc.0 as i32,
            v.len() as i32 / 16,
            transpose as u8,
            v.as_ptr(),
        );
    }
}

pub unsafe fn unmap_buffer(target: u32) {
    native_gl::glUnmapBuffer(target);
}

pub unsafe fn unmap_named_buffer(buffer: GlBuffer) {
    native_gl::glUnmapNamedBuffer(buffer.0.get());
}

pub unsafe fn cull_face(value: u32) {
    native_gl::glCullFace(value as u32);
}

pub unsafe fn color_mask(red: bool, green: bool, blue: bool, alpha: bool) {
    native_gl::glColorMask(red as u8, green as u8, blue as u8, alpha as u8);
}

pub unsafe fn color_mask_draw_buffer(
    draw_buffer: u32,
    red: bool,
    green: bool,
    blue: bool,
    alpha: bool,
) {
    native_gl::glColorMaski(draw_buffer, red as u8, green as u8, blue as u8, alpha as u8);
}

pub unsafe fn depth_mask(value: bool) {
    native_gl::glDepthMask(value as u8);
}

pub unsafe fn blend_color(red: f32, green: f32, blue: f32, alpha: f32) {
    native_gl::glBlendColor(red, green, blue, alpha);
}

pub unsafe fn line_width(width: f32) {
    native_gl::glLineWidth(width);
}

pub unsafe fn map_buffer_range(target: u32, offset: i32, length: i32, access: u32) -> *mut u8 {
    native_gl::glMapBufferRange(target, offset as isize, length as isize, access) as *mut u8
}

pub unsafe fn map_named_buffer_range(
    buffer: GlBuffer,
    offset: i32,
    length: i32,
    access: u32,
) -> *mut u8 {
    native_gl::glMapNamedBufferRange(buffer.0.get(), offset as isize, length as isize, access)
        as *mut u8
}

pub unsafe fn flush_mapped_buffer_range(target: u32, offset: i32, length: i32) {
    native_gl::glFlushMappedBufferRange(target, offset as isize, length as isize)
}

pub unsafe fn invalidate_buffer_sub_data(target: u32, offset: i32, length: i32) {
    native_gl::glInvalidateBufferSubData(target, offset as isize, length as isize)
}

pub unsafe fn invalidate_framebuffer(target: u32, attachments: &[u32]) {
    native_gl::glInvalidateFramebuffer(target, attachments.len() as i32, attachments.as_ptr());
}

pub unsafe fn polygon_offset(factor: f32, units: f32) {
    native_gl::glPolygonOffset(factor, units);
}

pub unsafe fn polygon_mode(face: u32, mode: u32) {
    native_gl::glPolygonMode(face as u32, mode as u32);
}

pub unsafe fn finish() {
    native_gl::glFinish();
}

pub unsafe fn bind_texture(target: u32, texture: Option<GlTexture>) {
    native_gl::glBindTexture(target, texture.map(|t| t.0.get()).unwrap_or(0));
}

pub unsafe fn bind_sampler(unit: u32, sampler: Option<GlSampler>) {
    native_gl::glBindSampler(unit, sampler.map(|s| s.0.get()).unwrap_or(0));
}

pub unsafe fn active_texture(unit: u32) {
    native_gl::glActiveTexture(unit);
}

pub unsafe fn fence_sync(condition: u32, flags: u32) -> Result<GlFence, String> {
    Ok(GlFence(native_gl::glFenceSync(condition as u32, flags)))
}

pub unsafe fn tex_parameter_f32(target: u32, parameter: u32, value: f32) {
    native_gl::glTexParameterf(target, parameter, value);
}

pub unsafe fn tex_parameter_i32(target: u32, parameter: u32, value: i32) {
    native_gl::glTexParameteri(target, parameter, value);
}

pub unsafe fn texture_parameter_i32(texture: GlTexture, parameter: u32, value: i32) {
    native_gl::glTextureParameteri(texture.0.get(), parameter, value);
}

pub unsafe fn tex_parameter_f32_slice(target: u32, parameter: u32, values: &[f32]) {
    native_gl::glTexParameterfv(target, parameter, values.as_ptr());
}

pub unsafe fn tex_parameter_i32_slice(target: u32, parameter: u32, values: &[i32]) {
    native_gl::glTexParameteriv(target, parameter, values.as_ptr());
}

pub unsafe fn tex_sub_image_2d(
    target: u32,
    level: i32,
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
    format: u32,
    ty: u32,
    pixels: PixelUnpackData,
) {
    native_gl::glTexSubImage2D(
        target,
        level,
        x_offset,
        y_offset,
        width,
        height,
        format,
        ty,
        match pixels {
            PixelUnpackData::BufferOffset(offset) => offset as *const std::ffi::c_void,
            PixelUnpackData::Slice(data) => data.as_ptr() as *const std::ffi::c_void,
        },
    );
}

pub unsafe fn texture_sub_image_2d(
    texture: GlTexture,
    level: i32,
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
    format: u32,
    ty: u32,
    pixels: PixelUnpackData,
) {
    native_gl::glTextureSubImage2D(
        texture.0.get(),
        level,
        x_offset,
        y_offset,
        width,
        height,
        format,
        ty,
        match pixels {
            PixelUnpackData::BufferOffset(offset) => offset as *const std::ffi::c_void,
            PixelUnpackData::Slice(data) => data.as_ptr() as *const std::ffi::c_void,
        },
    );
}

pub unsafe fn compressed_tex_sub_image_2d(
    target: u32,
    level: i32,
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
    format: u32,
    pixels: CompressedPixelUnpackData,
) {
    let (data, image_size) = match pixels {
        CompressedPixelUnpackData::BufferRange(ref range) => (
            range.start as *const std::ffi::c_void,
            (range.end - range.start) as i32,
        ),
        CompressedPixelUnpackData::Slice(data) => {
            (data.as_ptr() as *const std::ffi::c_void, data.len() as i32)
        }
    };

    native_gl::glCompressedTexSubImage2D(
        target, level, x_offset, y_offset, width, height, format, image_size, data,
    );
}

pub unsafe fn compressed_texture_sub_image_2d(
    texture: GlTexture,
    level: i32,
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
    format: u32,
    pixels: CompressedPixelUnpackData,
) {
    let (data, image_size) = match pixels {
        CompressedPixelUnpackData::BufferRange(ref range) => (
            range.start as *const std::ffi::c_void,
            (range.end - range.start) as i32,
        ),
        CompressedPixelUnpackData::Slice(data) => {
            (data.as_ptr() as *const std::ffi::c_void, data.len() as i32)
        }
    };

    native_gl::glCompressedTextureSubImage2D(
        texture.0.get(),
        level,
        x_offset,
        y_offset,
        width,
        height,
        format,
        image_size,
        data,
    );
}

pub unsafe fn tex_sub_image_3d(
    target: u32,
    level: i32,
    x_offset: i32,
    y_offset: i32,
    z_offset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    ty: u32,
    pixels: PixelUnpackData,
) {
    native_gl::glTexSubImage3D(
        target,
        level,
        x_offset,
        y_offset,
        z_offset,
        width,
        height,
        depth,
        format,
        ty,
        match pixels {
            PixelUnpackData::BufferOffset(offset) => offset as *const std::ffi::c_void,
            PixelUnpackData::Slice(data) => data.as_ptr() as *const std::ffi::c_void,
        },
    );
}

pub unsafe fn texture_sub_image_3d(
    texture: GlTexture,
    level: i32,
    x_offset: i32,
    y_offset: i32,
    z_offset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    ty: u32,
    pixels: PixelUnpackData,
) {
    native_gl::glTextureSubImage3D(
        texture.0.get(),
        level,
        x_offset,
        y_offset,
        z_offset,
        width,
        height,
        depth,
        format,
        ty,
        match pixels {
            PixelUnpackData::BufferOffset(offset) => offset as *const std::ffi::c_void,
            PixelUnpackData::Slice(data) => data.as_ptr() as *const std::ffi::c_void,
        },
    );
}

pub unsafe fn compressed_tex_sub_image_3d(
    target: u32,
    level: i32,
    x_offset: i32,
    y_offset: i32,
    z_offset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    pixels: CompressedPixelUnpackData,
) {
    let (data, image_size) = match pixels {
        CompressedPixelUnpackData::BufferRange(ref range) => (
            range.start as *const std::ffi::c_void,
            (range.end - range.start) as i32,
        ),
        CompressedPixelUnpackData::Slice(data) => {
            (data.as_ptr() as *const std::ffi::c_void, data.len() as i32)
        }
    };

    native_gl::glCompressedTexSubImage3D(
        target, level, x_offset, y_offset, z_offset, width, height, depth, format, image_size, data,
    );
}

pub unsafe fn compressed_texture_sub_image_3d(
    texture: GlTexture,
    level: i32,
    x_offset: i32,
    y_offset: i32,
    z_offset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    pixels: CompressedPixelUnpackData,
) {
    let (data, image_size) = match pixels {
        CompressedPixelUnpackData::BufferRange(ref range) => (
            range.start as *const std::ffi::c_void,
            (range.end - range.start) as i32,
        ),
        CompressedPixelUnpackData::Slice(data) => {
            (data.as_ptr() as *const std::ffi::c_void, data.len() as i32)
        }
    };

    native_gl::glCompressedTextureSubImage3D(
        texture.0.get(),
        level,
        x_offset,
        y_offset,
        z_offset,
        width,
        height,
        depth,
        format,
        image_size,
        data,
    );
}

pub unsafe fn depth_func(func: u32) {
    native_gl::glDepthFunc(func as u32);
}

pub unsafe fn depth_range_f32(near: f32, far: f32) {
    native_gl::glDepthRangef(near, far);
}

pub unsafe fn depth_range_f64(near: f64, far: f64) {
    native_gl::glDepthRange(near, far);
}

pub unsafe fn depth_range_f64_slice(first: u32, count: i32, values: &[[f64; 2]]) {
    native_gl::glDepthRangeArrayv(first, count, values.as_ptr() as *const f64);
}

pub unsafe fn scissor(x: i32, y: i32, width: i32, height: i32) {
    native_gl::glScissor(x, y, width, height);
}

pub unsafe fn scissor_slice(first: u32, count: i32, scissors: &[[i32; 4]]) {
    native_gl::glScissorArrayv(first, count, scissors.as_ptr() as *const i32);
}

pub unsafe fn vertex_array_attrib_binding_f32(vao: GlVertexArray, index: u32, binding_index: u32) {
    native_gl::glVertexArrayAttribBinding(vao.0.get(), index, binding_index);
}

pub unsafe fn vertex_array_attrib_format_f32(
    vao: GlVertexArray,
    index: u32,
    size: i32,
    data_type: u32,
    normalized: bool,
    relative_offset: u32,
) {
    native_gl::glVertexArrayAttribFormat(
        vao.0.get(),
        index,
        size,
        data_type,
        normalized as u8,
        relative_offset,
    );
}

pub unsafe fn vertex_array_attrib_format_i32(
    vao: GlVertexArray,
    index: u32,
    size: i32,
    data_type: u32,
    relative_offset: u32,
) {
    native_gl::glVertexArrayAttribIFormat(vao.0.get(), index, size, data_type, relative_offset);
}

pub unsafe fn vertex_array_element_buffer(vao: GlVertexArray, buffer: Option<GlBuffer>) {
    native_gl::glVertexArrayElementBuffer(vao.0.get(), buffer.map(|b| b.0.get()).unwrap_or(0));
}

pub unsafe fn vertex_array_vertex_buffer(
    vao: GlVertexArray,
    binding_index: u32,
    buffer: Option<GlBuffer>,
    offset: i32,
    stride: i32,
) {
    native_gl::glVertexArrayVertexBuffer(
        vao.0.get(),
        binding_index,
        buffer.map(|b| b.0.get()).unwrap_or(0),
        offset as isize,
        stride,
    );
}

pub unsafe fn vertex_attrib_divisor(index: u32, divisor: u32) {
    native_gl::glVertexAttribDivisor(index, divisor);
}

pub unsafe fn vertex_array_binding_divisor(vao: GlVertexArray, binding_index: u32, divisor: u32) {
    native_gl::glVertexArrayBindingDivisor(vao.0.get(), binding_index, divisor)
}

pub unsafe fn vertex_attrib_pointer_f32(
    index: u32,
    size: i32,
    data_type: u32,
    normalized: bool,
    stride: i32,
    offset: i32,
) {
    native_gl::glVertexAttribPointer(
        index,
        size,
        data_type,
        normalized as u8,
        stride,
        offset as *const std::ffi::c_void,
    );
}

pub unsafe fn vertex_attrib_pointer_i32(
    index: u32,
    size: i32,
    data_type: u32,
    stride: i32,
    offset: i32,
) {
    native_gl::glVertexAttribIPointer(
        index,
        size,
        data_type,
        stride,
        offset as *const std::ffi::c_void,
    );
}

pub unsafe fn vertex_attrib_pointer_f64(
    index: u32,
    size: i32,
    data_type: u32,
    stride: i32,
    offset: i32,
) {
    native_gl::glVertexAttribLPointer(
        index,
        size,
        data_type,
        stride,
        offset as *const std::ffi::c_void,
    );
}

pub unsafe fn vertex_attrib_format_f32(
    index: u32,
    size: i32,
    data_type: u32,
    normalized: bool,
    relative_offset: u32,
) {
    native_gl::glVertexAttribFormat(index, size, data_type, normalized as u8, relative_offset);
}

pub unsafe fn vertex_attrib_format_i32(
    index: u32,
    size: i32,
    data_type: u32,
    relative_offset: u32,
) {
    native_gl::glVertexAttribIFormat(index, size, data_type, relative_offset);
}

pub unsafe fn vertex_attrib_1_f32(index: u32, x: f32) {
    native_gl::glVertexAttrib1f(index, x);
}

pub unsafe fn vertex_attrib_2_f32(index: u32, x: f32, y: f32) {
    native_gl::glVertexAttrib2f(index, x, y);
}

pub unsafe fn vertex_attrib_3_f32(index: u32, x: f32, y: f32, z: f32) {
    native_gl::glVertexAttrib3f(index, x, y, z);
}

pub unsafe fn vertex_attrib_4_f32(index: u32, x: f32, y: f32, z: f32, w: f32) {
    native_gl::glVertexAttrib4f(index, x, y, z, w);
}

pub unsafe fn vertex_attrib_1_f32_slice(index: u32, v: &[f32]) {
    native_gl::glVertexAttrib1fv(index, v.as_ptr());
}

pub unsafe fn vertex_attrib_2_f32_slice(index: u32, v: &[f32]) {
    native_gl::glVertexAttrib2fv(index, v.as_ptr());
}

pub unsafe fn vertex_attrib_3_f32_slice(index: u32, v: &[f32]) {
    native_gl::glVertexAttrib3fv(index, v.as_ptr());
}

pub unsafe fn vertex_attrib_4_f32_slice(index: u32, v: &[f32]) {
    native_gl::glVertexAttrib4fv(index, v.as_ptr());
}

pub unsafe fn vertex_attrib_binding(attrib_index: u32, binding_index: u32) {
    native_gl::glVertexAttribBinding(attrib_index, binding_index);
}

pub unsafe fn vertex_binding_divisor(binding_index: u32, divisor: u32) {
    native_gl::glVertexBindingDivisor(binding_index, divisor);
}

pub unsafe fn viewport(x: i32, y: i32, width: i32, height: i32) {
    native_gl::glViewport(x, y, width, height);
}

pub unsafe fn viewport_f32_slice(first: u32, count: i32, values: &[[f32; 4]]) {
    native_gl::glViewportArrayv(first, count, values.as_ptr() as *const f32);
}

pub unsafe fn blend_equation(mode: u32) {
    native_gl::glBlendEquation(mode as u32);
}

pub unsafe fn blend_equation_draw_buffer(draw_buffer: u32, mode: u32) {
    native_gl::glBlendEquationi(draw_buffer, mode as u32);
}

pub unsafe fn blend_equation_separate(mode_rgb: u32, mode_alpha: u32) {
    native_gl::glBlendEquationSeparate(mode_rgb as u32, mode_alpha as u32);
}

pub unsafe fn blend_equation_separate_draw_buffer(
    draw_buffer: u32,
    mode_rgb: u32,
    mode_alpha: u32,
) {
    native_gl::glBlendEquationSeparatei(draw_buffer, mode_rgb as u32, mode_alpha as u32);
}

pub unsafe fn blend_func(src: u32, dst: u32) {
    native_gl::glBlendFunc(src as u32, dst as u32);
}

pub unsafe fn blend_func_draw_buffer(draw_buffer: u32, src: u32, dst: u32) {
    native_gl::glBlendFunci(draw_buffer, src as u32, dst as u32);
}

pub unsafe fn blend_func_separate(src_rgb: u32, dst_rgb: u32, src_alpha: u32, dst_alpha: u32) {
    native_gl::glBlendFuncSeparate(
        src_rgb as u32,
        dst_rgb as u32,
        src_alpha as u32,
        dst_alpha as u32,
    );
}

pub unsafe fn blend_func_separate_draw_buffer(
    draw_buffer: u32,
    src_rgb: u32,
    dst_rgb: u32,
    src_alpha: u32,
    dst_alpha: u32,
) {
    native_gl::glBlendFuncSeparatei(
        draw_buffer,
        src_rgb as u32,
        dst_rgb as u32,
        src_alpha as u32,
        dst_alpha as u32,
    );
}

pub unsafe fn stencil_func(func: u32, reference: i32, mask: u32) {
    native_gl::glStencilFunc(func as u32, reference, mask);
}

pub unsafe fn stencil_func_separate(face: u32, func: u32, reference: i32, mask: u32) {
    native_gl::glStencilFuncSeparate(face as u32, func as u32, reference, mask);
}

pub unsafe fn stencil_mask(mask: u32) {
    native_gl::glStencilMask(mask);
}

pub unsafe fn stencil_mask_separate(face: u32, mask: u32) {
    native_gl::glStencilMaskSeparate(face as u32, mask);
}

pub unsafe fn stencil_op(stencil_fail: u32, depth_fail: u32, pass: u32) {
    native_gl::glStencilOp(stencil_fail as u32, depth_fail as u32, pass as u32);
}

pub unsafe fn stencil_op_separate(face: u32, stencil_fail: u32, depth_fail: u32, pass: u32) {
    native_gl::glStencilOpSeparate(
        face as u32,
        stencil_fail as u32,
        depth_fail as u32,
        pass as u32,
    );
}

pub unsafe fn debug_message_control(
    source: u32,
    msg_type: u32,
    severity: u32,
    ids: &[u32],
    enabled: bool,
) {
    let ids_ptr = if ids.is_empty() {
        std::ptr::null()
    } else {
        ids.as_ptr()
    };

    native_gl::glDebugMessageControl(
        source,
        msg_type,
        severity,
        ids.len() as i32,
        ids_ptr,
        enabled as u8,
    );
}

pub unsafe fn debug_message_insert<S>(source: u32, msg_type: u32, id: u32, severity: u32, msg: S)
where
    S: AsRef<str>,
{
    let message = msg.as_ref().as_bytes();
    let length = message.len() as i32;
    native_gl::glDebugMessageInsert(
        source,
        msg_type,
        id,
        severity,
        length,
        message.as_ptr() as *const native_gl::GLchar,
    );
}

pub unsafe fn debug_message_callback<F>(mut callback: F)
where
    F: FnMut(u32, u32, u32, u32, &str),
{
    native_gl::glDebugMessageCallback(
        Some(raw_debug_message_callback::<F>),
        &mut callback as *mut _ as *mut std::ffi::c_void,
    );
}

pub unsafe fn get_debug_message_log(count: u32) -> Vec<DebugMessageLogEntry> {
    let ct = count as usize;
    let mut sources = Vec::with_capacity(ct);
    let mut types = Vec::with_capacity(ct);
    let mut ids = Vec::with_capacity(ct);
    let mut severities = Vec::with_capacity(ct);
    let mut lengths = Vec::with_capacity(ct);
    let buf_size = (count * MAX_DEBUG_MESSAGE_LENGTH) as i32;
    let mut message_log = Vec::with_capacity(buf_size as usize);

    let received = native_gl::glGetDebugMessageLog(
        count,
        buf_size,
        sources.as_mut_ptr(),
        types.as_mut_ptr(),
        ids.as_mut_ptr(),
        severities.as_mut_ptr(),
        lengths.as_mut_ptr(),
        message_log.as_mut_ptr(),
    ) as usize;

    sources.set_len(received);
    types.set_len(received);
    ids.set_len(received);
    severities.set_len(received);
    lengths.set_len(received);
    message_log.set_len(buf_size as usize);

    let mut entries = Vec::new();
    let mut offset = 0;
    for i in 0..received {
        let message = std::ffi::CStr::from_ptr(message_log[offset..].as_ptr()).to_string_lossy();
        offset += lengths[i] as usize;
        entries.push(DebugMessageLogEntry {
            source: sources[i],
            msg_type: types[i],
            id: ids[i],
            severity: severities[i],
            message: message.to_string(),
        });
    }

    entries
}

pub unsafe fn push_debug_group<S>(source: u32, id: u32, message: S)
where
    S: AsRef<str>,
{
    let msg = message.as_ref().as_bytes();
    let length = msg.len() as i32;
    native_gl::glPushDebugGroup(source, id, length, msg.as_ptr() as *const native_gl::GLchar);
}

pub unsafe fn pop_debug_group() {
    native_gl::glPopDebugGroup();
}

pub unsafe fn object_label<S>(identifier: u32, name: u32, label: Option<S>)
where
    S: AsRef<str>,
{
    match label {
        Some(l) => {
            let lbl = l.as_ref().as_bytes();
            let length = lbl.len() as i32;
            native_gl::glObjectLabel(
                identifier,
                name,
                length,
                lbl.as_ptr() as *const native_gl::GLchar,
            );
        }
        None => native_gl::glObjectLabel(identifier, name, 0, std::ptr::null()),
    }
}

pub unsafe fn get_object_label(identifier: u32, name: u32) -> String {
    let max_label_length = get_parameter_i32(MAX_LABEL_LENGTH);
    let mut len = 0;
    let mut label_buf = Vec::with_capacity(max_label_length as usize);
    native_gl::glGetObjectLabel(
        identifier,
        name,
        max_label_length,
        &mut len,
        label_buf.as_mut_ptr(),
    );
    label_buf.set_len(len as usize);
    std::ffi::CStr::from_ptr(label_buf.as_ptr())
        .to_str()
        .unwrap()
        .to_owned()
}

pub unsafe fn object_ptr_label<S>(sync: GlFence, label: Option<S>)
where
    S: AsRef<str>,
{
    match label {
        Some(l) => {
            let lbl = l.as_ref().as_bytes();
            let length = lbl.len() as i32;
            native_gl::glObjectPtrLabel(
                sync.0 as *mut std::ffi::c_void,
                length,
                lbl.as_ptr() as *const native_gl::GLchar,
            );
        }
        None => native_gl::glObjectPtrLabel(sync.0 as *mut std::ffi::c_void, 0, std::ptr::null()),
    }
}

pub unsafe fn get_object_ptr_label(sync: GlFence) -> String {
    let max_label_length = get_parameter_i32(MAX_LABEL_LENGTH);
    let mut len = 0;
    let mut label_buf = Vec::with_capacity(max_label_length as usize);
    native_gl::glGetObjectPtrLabel(
        sync.0 as *mut std::ffi::c_void,
        max_label_length,
        &mut len,
        label_buf.as_mut_ptr(),
    );
    label_buf.set_len(len as usize);
    std::ffi::CStr::from_ptr(label_buf.as_ptr())
        .to_str()
        .unwrap()
        .to_owned()
}

pub unsafe fn get_uniform_block_index(program: GlProgram, name: &str) -> Option<u32> {
    let name = CString::new(name).unwrap();
    let index = native_gl::glGetUniformBlockIndex(program.0.get(), name.as_ptr());
    if index == INVALID_INDEX {
        None
    } else {
        Some(index)
    }
}

pub unsafe fn uniform_block_binding(program: GlProgram, index: u32, binding: u32) {
    native_gl::glUniformBlockBinding(program.0.get(), index, binding);
}

pub unsafe fn get_shader_storage_block_index(program: GlProgram, name: &str) -> Option<u32> {
    let name = CString::new(name).unwrap();
    let index =
        native_gl::glGetProgramResourceIndex(program.0.get(), SHADER_STORAGE_BLOCK, name.as_ptr());
    if index == INVALID_INDEX {
        None
    } else {
        Some(index)
    }
}

pub unsafe fn shader_storage_block_binding(program: GlProgram, index: u32, binding: u32) {
    native_gl::glShaderStorageBlockBinding(program.0.get(), index, binding);
}

pub unsafe fn read_buffer(src: u32) {
    native_gl::glReadBuffer(src);
}

pub unsafe fn read_pixels(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    gltype: u32,
    pixels: PixelPackData,
) {
    native_gl::glReadPixels(
        x,
        y,
        width,
        height,
        format,
        gltype,
        match pixels {
            PixelPackData::BufferOffset(offset) => offset as *mut std::ffi::c_void,
            PixelPackData::Slice(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
        },
    );
}

pub unsafe fn begin_query(target: u32, query: GlQuery) {
    native_gl::glBeginQuery(target, query.0.get());
}

pub unsafe fn end_query(target: u32) {
    native_gl::glEndQuery(target);
}

pub unsafe fn get_query_parameter_u32(query: GlQuery, parameter: u32) -> u32 {
    let mut value = 0;
    native_gl::glGetQueryObjectuiv(query.0.get(), parameter, &mut value);
    value
}

pub unsafe fn create_transform_feedback() -> Result<GlTransformFeedback, String> {
    let mut name = 0;
    native_gl::glGenTransformFeedbacks(1, &mut name);
    Ok(GlTransformFeedback(non_zero_u32_gl_name(name)))
}

pub unsafe fn delete_transform_feedback(transform_feedback: GlTransformFeedback) {
    native_gl::glDeleteTransformFeedbacks(1, &transform_feedback.0.get());
}

pub unsafe fn bind_transform_feedback(
    target: u32,
    transform_feedback: Option<GlTransformFeedback>,
) {
    native_gl::glBindTransformFeedback(
        target,
        transform_feedback.map(|tf| tf.0.get()).unwrap_or(0),
    );
}

pub unsafe fn begin_transform_feedback(primitive_mode: u32) {
    native_gl::glBeginTransformFeedback(primitive_mode);
}

pub unsafe fn end_transform_feedback() {
    native_gl::glEndTransformFeedback();
}

pub unsafe fn pause_transform_feedback() {
    native_gl::glPauseTransformFeedback();
}

pub unsafe fn resume_transform_feedback() {
    native_gl::glResumeTransformFeedback();
}

pub unsafe fn transform_feedback_varyings(program: GlProgram, varyings: &[&str], buffer_mode: u32) {
    let strings: Vec<CString> = varyings
        .iter()
        .copied()
        .map(CString::new)
        .collect::<Result<_, _>>()
        .unwrap();
    let varyings: Vec<_> = strings.iter().map(|c_str| c_str.as_ptr()).collect();

    native_gl::glTransformFeedbackVaryings(
        program.0.get(),
        varyings.len() as i32,
        varyings.as_ptr(),
        buffer_mode,
    );
}

pub unsafe fn get_transform_feedback_varying(
    program: GlProgram,
    index: u32,
) -> Option<ActiveTransformFeedback> {
    const buf_size: usize = 256;
    const bytes: [u8; buf_size] = [0; buf_size];

    let size: i32 = 0;
    let tftype: u32 = 0;
    let c_name = CString::new(bytes.to_vec()).unwrap();
    let c_name_buf = c_name.into_raw();

    native_gl::glGetTransformFeedbackVarying(
        program.0.get(),
        index,
        buf_size as i32,
        std::ptr::null_mut(),
        size as *mut i32,
        tftype as *mut u32,
        c_name_buf,
    );

    let name = CString::from_raw(c_name_buf).into_string().unwrap();

    Some(ActiveTransformFeedback { size, tftype, name })
}

pub unsafe fn memory_barrier(barriers: u32) {
    native_gl::glMemoryBarrier(barriers);
}

pub unsafe fn memory_barrier_by_region(barriers: u32) {
    native_gl::glMemoryBarrierByRegion(barriers);
}

pub unsafe fn bind_image_texture(
    unit: u32,
    texture: GlTexture,
    level: i32,
    layered: bool,
    layer: i32,
    access: u32,
    format: u32,
) {
    native_gl::glBindImageTexture(
        unit,
        texture.0.get(),
        level,
        layered as u8,
        layer,
        access,
        format,
    );
}
pub unsafe fn get_active_uniform_block_parameter_i32(
    program: GlProgram,
    uniform_block_index: u32,
    parameter: u32,
) -> i32 {
    let mut value = 0;
    native_gl::glGetActiveUniformBlockiv(
        program.0.get(),
        uniform_block_index,
        parameter,
        &mut value,
    );
    value
}

pub unsafe fn get_active_uniform_block_parameter_i32_slice(
    program: GlProgram,
    uniform_block_index: u32,
    parameter: u32,
    out: &mut [i32],
) {
    native_gl::glGetActiveUniformBlockiv(
        program.0.get(),
        uniform_block_index,
        parameter,
        out.as_mut_ptr(),
    );
}
pub unsafe fn get_active_uniform_block_name(
    program: GlProgram,
    uniform_block_index: u32,
) -> String {
    // Probe for the length of the name of the uniform block, and, failing
    // that, fall back to allocating a buffer that is 256 bytes long. This
    // should be good enough for pretty much all contexts, including faulty
    // or partially faulty ones.
    let len = get_active_uniform_block_parameter_i32(
        program,
        uniform_block_index,
        UNIFORM_BLOCK_NAME_LENGTH,
    );
    let len = if native_gl::glGetError() == NO_ERROR && len > 0 {
        len as usize
    } else {
        256
    };

    let mut buffer = vec![0; len];
    let mut length = 0;
    native_gl::glGetActiveUniformBlockName(
        program.0.get(),
        uniform_block_index,
        buffer.len() as _,
        &mut length,
        buffer.as_mut_ptr(),
    );

    if length > 0 {
        assert_eq!(
            std::mem::size_of::<u8>(),
            std::mem::size_of::<native_gl::GLchar>(),
            "This operation is only safe in systems in which the length of \
                a GLchar is the same as that of an u8"
        );
        assert_eq!(
            std::mem::align_of::<u8>(),
            std::mem::align_of::<native_gl::GLchar>(),
            "This operation is only safe in systems in which the alignment \
                of a GLchar is the same as that of an u8"
        );
        let buffer = std::slice::from_raw_parts(
            buffer.as_ptr() as *const u8,
            (length as usize + 1).min(buffer.len()),
        );

        let name = CStr::from_bytes_with_nul(&buffer[..])
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        name
    } else {
        String::from("")
    }
}

extern "system" fn raw_debug_message_callback<F>(
    source: u32,
    gltype: u32,
    id: u32,
    severity: u32,
    length: i32,
    message: *const native_gl::GLchar,
    user_param: *mut std::ffi::c_void,
) where
    F: FnMut(u32, u32, u32, u32, &str),
{
    std::panic::catch_unwind(move || unsafe {
        let callback: &mut F = &mut *(user_param as *mut _);
        let slice = std::slice::from_raw_parts(message as *const u8, length as usize);
        let msg = std::str::from_utf8(slice).unwrap();
        (callback)(source, gltype, id, severity, msg);
    })
    .ok();
}

pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: u32 = 0x92D9;
pub const ACTIVE_ATTRIBUTES: u32 = 0x8B89;
pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: u32 = 0x8B8A;
pub const ACTIVE_PROGRAM: u32 = 0x8259;
pub const ACTIVE_RESOURCES: u32 = 0x92F5;
pub const ACTIVE_SUBROUTINES: u32 = 0x8DE5;
pub const ACTIVE_SUBROUTINE_MAX_LENGTH: u32 = 0x8E48;
pub const ACTIVE_SUBROUTINE_UNIFORMS: u32 = 0x8DE6;
pub const ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: u32 = 0x8E47;
pub const ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: u32 = 0x8E49;
pub const ACTIVE_TEXTURE: u32 = 0x84E0;
pub const ACTIVE_UNIFORMS: u32 = 0x8B86;
pub const ACTIVE_UNIFORM_BLOCKS: u32 = 0x8A36;
pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: u32 = 0x8A35;
pub const ACTIVE_UNIFORM_MAX_LENGTH: u32 = 0x8B87;
pub const ACTIVE_VARIABLES: u32 = 0x9305;
pub const ALIASED_LINE_WIDTH_RANGE: u32 = 0x846E;
pub const ALL_BARRIER_BITS: u32 = 0xFFFFFFFF;
pub const ALL_SHADER_BITS: u32 = 0xFFFFFFFF;
pub const ALPHA: u32 = 0x1906;
pub const ALREADY_SIGNALED: u32 = 0x911A;
pub const ALWAYS: u32 = 0x0207;
pub const AND: u32 = 0x1501;
pub const AND_INVERTED: u32 = 0x1504;
pub const AND_REVERSE: u32 = 0x1502;
pub const ANY_SAMPLES_PASSED: u32 = 0x8C2F;
pub const ANY_SAMPLES_PASSED_CONSERVATIVE: u32 = 0x8D6A;
pub const ARRAY_BUFFER: u32 = 0x8892;
pub const ARRAY_BUFFER_BINDING: u32 = 0x8894;
pub const ARRAY_SIZE: u32 = 0x92FB;
pub const ARRAY_STRIDE: u32 = 0x92FE;
pub const ATOMIC_COUNTER_BARRIER_BIT: u32 = 0x00001000;
pub const ATOMIC_COUNTER_BUFFER: u32 = 0x92C0;
pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: u32 = 0x92C5;
pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: u32 = 0x92C6;
pub const ATOMIC_COUNTER_BUFFER_BINDING: u32 = 0x92C1;
pub const ATOMIC_COUNTER_BUFFER_DATA_SIZE: u32 = 0x92C4;
pub const ATOMIC_COUNTER_BUFFER_INDEX: u32 = 0x9301;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: u32 = 0x90ED;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: u32 = 0x92CB;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: u32 = 0x92CA;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: u32 = 0x92C8;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: u32 = 0x92C9;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: u32 = 0x92C7;
pub const ATOMIC_COUNTER_BUFFER_SIZE: u32 = 0x92C3;
pub const ATOMIC_COUNTER_BUFFER_START: u32 = 0x92C2;
pub const ATTACHED_SHADERS: u32 = 0x8B85;
pub const AUTO_GENERATE_MIPMAP: u32 = 0x8295;
pub const BACK: u32 = 0x0405;
pub const BACK_LEFT: u32 = 0x0402;
pub const BACK_RIGHT: u32 = 0x0403;
pub const BGR: u32 = 0x80E0;
pub const BGRA: u32 = 0x80E1;
pub const BGRA_INTEGER: u32 = 0x8D9B;
pub const BGR_INTEGER: u32 = 0x8D9A;
pub const BLEND: u32 = 0x0BE2;
pub const BLEND_COLOR: u32 = 0x8005;
pub const BLEND_DST: u32 = 0x0BE0;
pub const BLEND_DST_ALPHA: u32 = 0x80CA;
pub const BLEND_DST_RGB: u32 = 0x80C8;
pub const BLEND_EQUATION: u32 = 0x8009;
pub const BLEND_EQUATION_ALPHA: u32 = 0x883D;
pub const BLEND_EQUATION_RGB: u32 = 0x8009;
pub const BLEND_SRC: u32 = 0x0BE1;
pub const BLEND_SRC_ALPHA: u32 = 0x80CB;
pub const BLEND_SRC_RGB: u32 = 0x80C9;
pub const BLOCK_INDEX: u32 = 0x92FD;
pub const BLUE: u32 = 0x1905;
pub const BLUE_INTEGER: u32 = 0x8D96;
pub const BOOL: u32 = 0x8B56;
pub const BOOL_VEC2: u32 = 0x8B57;
pub const BOOL_VEC3: u32 = 0x8B58;
pub const BOOL_VEC4: u32 = 0x8B59;
pub const BUFFER: u32 = 0x82E0;
pub const BUFFER_ACCESS: u32 = 0x88BB;
pub const BUFFER_ACCESS_FLAGS: u32 = 0x911F;
pub const BUFFER_BINDING: u32 = 0x9302;
pub const BUFFER_DATA_SIZE: u32 = 0x9303;
pub const BUFFER_IMMUTABLE_STORAGE: u32 = 0x821F;
pub const BUFFER_IMMUTABLE_STORAGE_EXT: u32 = 0x821F;
pub const BUFFER_MAPPED: u32 = 0x88BC;
pub const BUFFER_MAP_LENGTH: u32 = 0x9120;
pub const BUFFER_MAP_OFFSET: u32 = 0x9121;
pub const BUFFER_MAP_POINTER: u32 = 0x88BD;
pub const BUFFER_SIZE: u32 = 0x8764;
pub const BUFFER_STORAGE_FLAGS: u32 = 0x8220;
pub const BUFFER_STORAGE_FLAGS_EXT: u32 = 0x8220;
pub const BUFFER_UPDATE_BARRIER_BIT: u32 = 0x00000200;
pub const BUFFER_USAGE: u32 = 0x8765;
pub const BUFFER_VARIABLE: u32 = 0x92E5;
pub const BYTE: u32 = 0x1400;
pub const CAVEAT_SUPPORT: u32 = 0x82B8;
pub const CCW: u32 = 0x0901;
pub const CLAMP_READ_COLOR: u32 = 0x891C;
pub const CLAMP_TO_BORDER: u32 = 0x812D;
pub const CLAMP_TO_EDGE: u32 = 0x812F;
pub const CLEAR: u32 = 0x1500;
pub const CLEAR_BUFFER: u32 = 0x82B4;
pub const CLEAR_TEXTURE: u32 = 0x9365;
pub const CLIENT_MAPPED_BUFFER_BARRIER_BIT: u32 = 0x00004000;
pub const CLIENT_MAPPED_BUFFER_BARRIER_BIT_EXT: u32 = 0x00004000;
pub const CLIENT_STORAGE_BIT: u32 = 0x0200;
pub const CLIENT_STORAGE_BIT_EXT: u32 = 0x0200;
pub const CLIPPING_INPUT_PRIMITIVES: u32 = 0x82F6;
pub const CLIPPING_OUTPUT_PRIMITIVES: u32 = 0x82F7;
pub const CLIP_DEPTH_MODE: u32 = 0x935D;
pub const CLIP_DISTANCE0: u32 = 0x3000;
pub const CLIP_DISTANCE1: u32 = 0x3001;
pub const CLIP_DISTANCE2: u32 = 0x3002;
pub const CLIP_DISTANCE3: u32 = 0x3003;
pub const CLIP_DISTANCE4: u32 = 0x3004;
pub const CLIP_DISTANCE5: u32 = 0x3005;
pub const CLIP_DISTANCE6: u32 = 0x3006;
pub const CLIP_DISTANCE7: u32 = 0x3007;
pub const CLIP_ORIGIN: u32 = 0x935C;
pub const COLOR: u32 = 0x1800;
pub const COLOR_ATTACHMENT0: u32 = 0x8CE0;
pub const COLOR_ATTACHMENT1: u32 = 0x8CE1;
pub const COLOR_ATTACHMENT10: u32 = 0x8CEA;
pub const COLOR_ATTACHMENT11: u32 = 0x8CEB;
pub const COLOR_ATTACHMENT12: u32 = 0x8CEC;
pub const COLOR_ATTACHMENT13: u32 = 0x8CED;
pub const COLOR_ATTACHMENT14: u32 = 0x8CEE;
pub const COLOR_ATTACHMENT15: u32 = 0x8CEF;
pub const COLOR_ATTACHMENT16: u32 = 0x8CF0;
pub const COLOR_ATTACHMENT17: u32 = 0x8CF1;
pub const COLOR_ATTACHMENT18: u32 = 0x8CF2;
pub const COLOR_ATTACHMENT19: u32 = 0x8CF3;
pub const COLOR_ATTACHMENT2: u32 = 0x8CE2;
pub const COLOR_ATTACHMENT20: u32 = 0x8CF4;
pub const COLOR_ATTACHMENT21: u32 = 0x8CF5;
pub const COLOR_ATTACHMENT22: u32 = 0x8CF6;
pub const COLOR_ATTACHMENT23: u32 = 0x8CF7;
pub const COLOR_ATTACHMENT24: u32 = 0x8CF8;
pub const COLOR_ATTACHMENT25: u32 = 0x8CF9;
pub const COLOR_ATTACHMENT26: u32 = 0x8CFA;
pub const COLOR_ATTACHMENT27: u32 = 0x8CFB;
pub const COLOR_ATTACHMENT28: u32 = 0x8CFC;
pub const COLOR_ATTACHMENT29: u32 = 0x8CFD;
pub const COLOR_ATTACHMENT3: u32 = 0x8CE3;
pub const COLOR_ATTACHMENT30: u32 = 0x8CFE;
pub const COLOR_ATTACHMENT31: u32 = 0x8CFF;
pub const COLOR_ATTACHMENT4: u32 = 0x8CE4;
pub const COLOR_ATTACHMENT5: u32 = 0x8CE5;
pub const COLOR_ATTACHMENT6: u32 = 0x8CE6;
pub const COLOR_ATTACHMENT7: u32 = 0x8CE7;
pub const COLOR_ATTACHMENT8: u32 = 0x8CE8;
pub const COLOR_ATTACHMENT9: u32 = 0x8CE9;
pub const COLOR_BUFFER_BIT: u32 = 0x00004000;
pub const COLOR_CLEAR_VALUE: u32 = 0x0C22;
pub const COLOR_COMPONENTS: u32 = 0x8283;
pub const COLOR_ENCODING: u32 = 0x8296;
pub const COLOR_LOGIC_OP: u32 = 0x0BF2;
pub const COLOR_RENDERABLE: u32 = 0x8286;
pub const COLOR_WRITEMASK: u32 = 0x0C23;
pub const COMMAND_BARRIER_BIT: u32 = 0x00000040;
pub const COMPARE_REF_TO_TEXTURE: u32 = 0x884E;
pub const COMPATIBLE_SUBROUTINES: u32 = 0x8E4B;
pub const COMPILE_STATUS: u32 = 0x8B81;
pub const COMPRESSED_R11_EAC: u32 = 0x9270;
pub const COMPRESSED_RED: u32 = 0x8225;
pub const COMPRESSED_RED_RGTC1: u32 = 0x8DBB;
pub const COMPRESSED_RG: u32 = 0x8226;
pub const COMPRESSED_RG11_EAC: u32 = 0x9272;
pub const COMPRESSED_RGB: u32 = 0x84ED;
pub const COMPRESSED_RGB8_ETC2: u32 = 0x9274;
pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: u32 = 0x9276;
pub const COMPRESSED_RGBA: u32 = 0x84EE;
pub const COMPRESSED_RGBA8_ETC2_EAC: u32 = 0x9278;
pub const COMPRESSED_RGBA_BPTC_UNORM: u32 = 0x8E8C;
pub const COMPRESSED_RGB_BPTC_SIGNED_FLOAT: u32 = 0x8E8E;
pub const COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: u32 = 0x8E8F;
pub const COMPRESSED_RG_RGTC2: u32 = 0x8DBD;
pub const COMPRESSED_SIGNED_R11_EAC: u32 = 0x9271;
pub const COMPRESSED_SIGNED_RED_RGTC1: u32 = 0x8DBC;
pub const COMPRESSED_SIGNED_RG11_EAC: u32 = 0x9273;
pub const COMPRESSED_SIGNED_RG_RGTC2: u32 = 0x8DBE;
pub const COMPRESSED_SRGB: u32 = 0x8C48;
pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: u32 = 0x9279;
pub const COMPRESSED_SRGB8_ETC2: u32 = 0x9275;
pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: u32 = 0x9277;
pub const COMPRESSED_SRGB_ALPHA: u32 = 0x8C49;
pub const COMPRESSED_SRGB_ALPHA_BPTC_UNORM: u32 = 0x8E8D;
pub const COMPRESSED_TEXTURE_FORMATS: u32 = 0x86A3;
pub const COMPUTE_SHADER: u32 = 0x91B9;
pub const COMPUTE_SHADER_BIT: u32 = 0x00000020;
pub const COMPUTE_SHADER_INVOCATIONS: u32 = 0x82F5;
pub const COMPUTE_SUBROUTINE: u32 = 0x92ED;
pub const COMPUTE_SUBROUTINE_UNIFORM: u32 = 0x92F3;
pub const COMPUTE_TEXTURE: u32 = 0x82A0;
pub const COMPUTE_WORK_GROUP_SIZE: u32 = 0x8267;
pub const CONDITION_SATISFIED: u32 = 0x911C;
pub const CONSTANT_ALPHA: u32 = 0x8003;
pub const CONSTANT_COLOR: u32 = 0x8001;
pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: u32 = 0x00000002;
pub const CONTEXT_CORE_PROFILE_BIT: u32 = 0x00000001;
pub const CONTEXT_FLAGS: u32 = 0x821E;
pub const CONTEXT_FLAG_DEBUG_BIT: u32 = 0x00000002;
pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: u32 = 0x00000001;
pub const CONTEXT_FLAG_NO_ERROR_BIT: u32 = 0x00000008;
pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT: u32 = 0x00000004;
pub const CONTEXT_LOST: u32 = 0x0507;
pub const CONTEXT_PROFILE_MASK: u32 = 0x9126;
pub const CONTEXT_RELEASE_BEHAVIOR: u32 = 0x82FB;
pub const CONTEXT_RELEASE_BEHAVIOR_FLUSH: u32 = 0x82FC;
pub const COPY: u32 = 0x1503;
pub const COPY_INVERTED: u32 = 0x150C;
pub const COPY_READ_BUFFER: u32 = 0x8F36;
pub const COPY_READ_BUFFER_BINDING: u32 = 0x8F36;
pub const COPY_READ_BUFFER_NV: u32 = 0x8F36;
pub const COPY_WRITE_BUFFER: u32 = 0x8F37;
pub const COPY_WRITE_BUFFER_BINDING: u32 = 0x8F37;
pub const COPY_WRITE_BUFFER_NV: u32 = 0x8F37;
pub const CULL_FACE: u32 = 0x0B44;
pub const CULL_FACE_MODE: u32 = 0x0B45;
pub const CURRENT_PROGRAM: u32 = 0x8B8D;
pub const CURRENT_QUERY: u32 = 0x8865;
pub const CURRENT_VERTEX_ATTRIB: u32 = 0x8626;
pub const CW: u32 = 0x0900;
pub const DEBUG_CALLBACK_FUNCTION: u32 = 0x8244;
pub const DEBUG_CALLBACK_FUNCTION_ARB: u32 = 0x8244;
pub const DEBUG_CALLBACK_USER_PARAM: u32 = 0x8245;
pub const DEBUG_CALLBACK_USER_PARAM_ARB: u32 = 0x8245;
pub const DEBUG_GROUP_STACK_DEPTH: u32 = 0x826D;
pub const DEBUG_LOGGED_MESSAGES: u32 = 0x9145;
pub const DEBUG_LOGGED_MESSAGES_ARB: u32 = 0x9145;
pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: u32 = 0x8243;
pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_ARB: u32 = 0x8243;
pub const DEBUG_OUTPUT: u32 = 0x92E0;
pub const DEBUG_OUTPUT_SYNCHRONOUS: u32 = 0x8242;
pub const DEBUG_OUTPUT_SYNCHRONOUS_ARB: u32 = 0x8242;
pub const DEBUG_SEVERITY_HIGH: u32 = 0x9146;
pub const DEBUG_SEVERITY_HIGH_ARB: u32 = 0x9146;
pub const DEBUG_SEVERITY_LOW: u32 = 0x9148;
pub const DEBUG_SEVERITY_LOW_ARB: u32 = 0x9148;
pub const DEBUG_SEVERITY_MEDIUM: u32 = 0x9147;
pub const DEBUG_SEVERITY_MEDIUM_ARB: u32 = 0x9147;
pub const DEBUG_SEVERITY_NOTIFICATION: u32 = 0x826B;
pub const DEBUG_SOURCE_API: u32 = 0x8246;
pub const DEBUG_SOURCE_API_ARB: u32 = 0x8246;
pub const DEBUG_SOURCE_APPLICATION: u32 = 0x824A;
pub const DEBUG_SOURCE_APPLICATION_ARB: u32 = 0x824A;
pub const DEBUG_SOURCE_OTHER: u32 = 0x824B;
pub const DEBUG_SOURCE_OTHER_ARB: u32 = 0x824B;
pub const DEBUG_SOURCE_SHADER_COMPILER: u32 = 0x8248;
pub const DEBUG_SOURCE_SHADER_COMPILER_ARB: u32 = 0x8248;
pub const DEBUG_SOURCE_THIRD_PARTY: u32 = 0x8249;
pub const DEBUG_SOURCE_THIRD_PARTY_ARB: u32 = 0x8249;
pub const DEBUG_SOURCE_WINDOW_SYSTEM: u32 = 0x8247;
pub const DEBUG_SOURCE_WINDOW_SYSTEM_ARB: u32 = 0x8247;
pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: u32 = 0x824D;
pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR_ARB: u32 = 0x824D;
pub const DEBUG_TYPE_ERROR: u32 = 0x824C;
pub const DEBUG_TYPE_ERROR_ARB: u32 = 0x824C;
pub const DEBUG_TYPE_MARKER: u32 = 0x8268;
pub const DEBUG_TYPE_OTHER: u32 = 0x8251;
pub const DEBUG_TYPE_OTHER_ARB: u32 = 0x8251;
pub const DEBUG_TYPE_PERFORMANCE: u32 = 0x8250;
pub const DEBUG_TYPE_PERFORMANCE_ARB: u32 = 0x8250;
pub const DEBUG_TYPE_POP_GROUP: u32 = 0x826A;
pub const DEBUG_TYPE_PORTABILITY: u32 = 0x824F;
pub const DEBUG_TYPE_PORTABILITY_ARB: u32 = 0x824F;
pub const DEBUG_TYPE_PUSH_GROUP: u32 = 0x8269;
pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: u32 = 0x824E;
pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR_ARB: u32 = 0x824E;
pub const DECR: u32 = 0x1E03;
pub const DECR_WRAP: u32 = 0x8508;
pub const DELETE_STATUS: u32 = 0x8B80;
pub const DEPTH: u32 = 0x1801;
pub const DEPTH24_STENCIL8: u32 = 0x88F0;
pub const DEPTH32F_STENCIL8: u32 = 0x8CAD;
pub const DEPTH_ATTACHMENT: u32 = 0x8D00;
pub const DEPTH_BUFFER_BIT: u32 = 0x00000100;
pub const DEPTH_CLAMP: u32 = 0x864F;
pub const DEPTH_CLEAR_VALUE: u32 = 0x0B73;
pub const DEPTH_COMPONENT: u32 = 0x1902;
pub const DEPTH_COMPONENT16: u32 = 0x81A5;
pub const DEPTH_COMPONENT24: u32 = 0x81A6;
pub const DEPTH_COMPONENT32: u32 = 0x81A7;
pub const DEPTH_COMPONENT32F: u32 = 0x8CAC;
pub const DEPTH_COMPONENTS: u32 = 0x8284;
pub const DEPTH_FUNC: u32 = 0x0B74;
pub const DEPTH_RANGE: u32 = 0x0B70;
pub const DEPTH_RENDERABLE: u32 = 0x8287;
pub const DEPTH_STENCIL: u32 = 0x84F9;
pub const DEPTH_STENCIL_ATTACHMENT: u32 = 0x821A;
pub const DEPTH_STENCIL_TEXTURE_MODE: u32 = 0x90EA;
pub const DEPTH_TEST: u32 = 0x0B71;
pub const DEPTH_WRITEMASK: u32 = 0x0B72;
pub const DISPATCH_INDIRECT_BUFFER: u32 = 0x90EE;
pub const DISPATCH_INDIRECT_BUFFER_BINDING: u32 = 0x90EF;
pub const DITHER: u32 = 0x0BD0;
pub const DONT_CARE: u32 = 0x1100;
pub const DOUBLE: u32 = 0x140A;
pub const DOUBLEBUFFER: u32 = 0x0C32;
pub const DOUBLE_MAT2: u32 = 0x8F46;
#[allow(non_upper_case_globals)]
pub const DOUBLE_MAT2x3: u32 = 0x8F49;
#[allow(non_upper_case_globals)]
pub const DOUBLE_MAT2x4: u32 = 0x8F4A;
pub const DOUBLE_MAT3: u32 = 0x8F47;
#[allow(non_upper_case_globals)]
pub const DOUBLE_MAT3x2: u32 = 0x8F4B;
#[allow(non_upper_case_globals)]
pub const DOUBLE_MAT3x4: u32 = 0x8F4C;
pub const DOUBLE_MAT4: u32 = 0x8F48;
#[allow(non_upper_case_globals)]
pub const DOUBLE_MAT4x2: u32 = 0x8F4D;
#[allow(non_upper_case_globals)]
pub const DOUBLE_MAT4x3: u32 = 0x8F4E;
pub const DOUBLE_VEC2: u32 = 0x8FFC;
pub const DOUBLE_VEC3: u32 = 0x8FFD;
pub const DOUBLE_VEC4: u32 = 0x8FFE;
pub const DRAW_BUFFER: u32 = 0x0C01;
pub const DRAW_BUFFER0: u32 = 0x8825;
pub const DRAW_BUFFER1: u32 = 0x8826;
pub const DRAW_BUFFER10: u32 = 0x882F;
pub const DRAW_BUFFER11: u32 = 0x8830;
pub const DRAW_BUFFER12: u32 = 0x8831;
pub const DRAW_BUFFER13: u32 = 0x8832;
pub const DRAW_BUFFER14: u32 = 0x8833;
pub const DRAW_BUFFER15: u32 = 0x8834;
pub const DRAW_BUFFER2: u32 = 0x8827;
pub const DRAW_BUFFER3: u32 = 0x8828;
pub const DRAW_BUFFER4: u32 = 0x8829;
pub const DRAW_BUFFER5: u32 = 0x882A;
pub const DRAW_BUFFER6: u32 = 0x882B;
pub const DRAW_BUFFER7: u32 = 0x882C;
pub const DRAW_BUFFER8: u32 = 0x882D;
pub const DRAW_BUFFER9: u32 = 0x882E;
pub const DRAW_FRAMEBUFFER: u32 = 0x8CA9;
pub const DRAW_FRAMEBUFFER_BINDING: u32 = 0x8CA6;
pub const DRAW_INDIRECT_BUFFER: u32 = 0x8F3F;
pub const DRAW_INDIRECT_BUFFER_BINDING: u32 = 0x8F43;
pub const DST_ALPHA: u32 = 0x0304;
pub const DST_COLOR: u32 = 0x0306;
pub const DYNAMIC_COPY: u32 = 0x88EA;
pub const DYNAMIC_DRAW: u32 = 0x88E8;
pub const DYNAMIC_READ: u32 = 0x88E9;
pub const DYNAMIC_STORAGE_BIT: u32 = 0x0100;
pub const DYNAMIC_STORAGE_BIT_EXT: u32 = 0x0100;
pub const ELEMENT_ARRAY_BARRIER_BIT: u32 = 0x00000002;
pub const ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
pub const ELEMENT_ARRAY_BUFFER_BINDING: u32 = 0x8895;
pub const EQUAL: u32 = 0x0202;
pub const EQUIV: u32 = 0x1509;
pub const EXTENSIONS: u32 = 0x1F03;
pub const FALSE: u32 = 0;
pub const FASTEST: u32 = 0x1101;
pub const FILL: u32 = 0x1B02;
pub const FILTER: u32 = 0x829A;
pub const FIRST_VERTEX_CONVENTION: u32 = 0x8E4D;
pub const FIXED: u32 = 0x140C;
pub const FIXED_ONLY: u32 = 0x891D;
pub const FLOAT: u32 = 0x1406;
pub const FLOAT_32_UNSIGNED_INT_24_8_REV: u32 = 0x8DAD;
pub const FLOAT_MAT2: u32 = 0x8B5A;
#[allow(non_upper_case_globals)]
pub const FLOAT_MAT2x3: u32 = 0x8B65;
#[allow(non_upper_case_globals)]
pub const FLOAT_MAT2x4: u32 = 0x8B66;
pub const FLOAT_MAT3: u32 = 0x8B5B;
#[allow(non_upper_case_globals)]
pub const FLOAT_MAT3x2: u32 = 0x8B67;
#[allow(non_upper_case_globals)]
pub const FLOAT_MAT3x4: u32 = 0x8B68;
pub const FLOAT_MAT4: u32 = 0x8B5C;
#[allow(non_upper_case_globals)]
pub const FLOAT_MAT4x2: u32 = 0x8B69;
#[allow(non_upper_case_globals)]
pub const FLOAT_MAT4x3: u32 = 0x8B6A;
pub const FLOAT_VEC2: u32 = 0x8B50;
pub const FLOAT_VEC3: u32 = 0x8B51;
pub const FLOAT_VEC4: u32 = 0x8B52;
pub const FRACTIONAL_EVEN: u32 = 0x8E7C;
pub const FRACTIONAL_ODD: u32 = 0x8E7B;
pub const FRAGMENT_INTERPOLATION_OFFSET_BITS: u32 = 0x8E5D;
pub const FRAGMENT_SHADER: u32 = 0x8B30;
pub const FRAGMENT_SHADER_BIT: u32 = 0x00000002;
pub const FRAGMENT_SHADER_DERIVATIVE_HINT: u32 = 0x8B8B;
pub const FRAGMENT_SHADER_INVOCATIONS: u32 = 0x82F4;
pub const FRAGMENT_SUBROUTINE: u32 = 0x92EC;
pub const FRAGMENT_SUBROUTINE_UNIFORM: u32 = 0x92F2;
pub const FRAGMENT_TEXTURE: u32 = 0x829F;
pub const FRAMEBUFFER: u32 = 0x8D40;
pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: u32 = 0x8215;
pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: u32 = 0x8214;
pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: u32 = 0x8210;
pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: u32 = 0x8211;
pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: u32 = 0x8216;
pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: u32 = 0x8213;
pub const FRAMEBUFFER_ATTACHMENT_LAYERED: u32 = 0x8DA7;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: u32 = 0x8CD1;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: u32 = 0x8CD0;
pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: u32 = 0x8212;
pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: u32 = 0x8217;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: u32 = 0x8CD3;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: u32 = 0x8CD4;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: u32 = 0x8CD2;
pub const FRAMEBUFFER_BARRIER_BIT: u32 = 0x00000400;
pub const FRAMEBUFFER_BINDING: u32 = 0x8CA6;
pub const FRAMEBUFFER_BLEND: u32 = 0x828B;
pub const FRAMEBUFFER_COMPLETE: u32 = 0x8CD5;
pub const FRAMEBUFFER_DEFAULT: u32 = 0x8218;
pub const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: u32 = 0x9314;
pub const FRAMEBUFFER_DEFAULT_HEIGHT: u32 = 0x9311;
pub const FRAMEBUFFER_DEFAULT_LAYERS: u32 = 0x9312;
pub const FRAMEBUFFER_DEFAULT_SAMPLES: u32 = 0x9313;
pub const FRAMEBUFFER_DEFAULT_WIDTH: u32 = 0x9310;
pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: u32 = 0x8CD6;
pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: u32 = 0x8CDB;
pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: u32 = 0x8DA8;
pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: u32 = 0x8CD7;
pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: u32 = 0x8D56;
pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: u32 = 0x8CDC;
pub const FRAMEBUFFER_RENDERABLE: u32 = 0x8289;
pub const FRAMEBUFFER_RENDERABLE_LAYERED: u32 = 0x828A;
pub const FRAMEBUFFER_SRGB: u32 = 0x8DB9;
pub const FRAMEBUFFER_UNDEFINED: u32 = 0x8219;
pub const FRAMEBUFFER_UNSUPPORTED: u32 = 0x8CDD;
pub const FRONT: u32 = 0x0404;
pub const FRONT_AND_BACK: u32 = 0x0408;
pub const FRONT_FACE: u32 = 0x0B46;
pub const FRONT_LEFT: u32 = 0x0400;
pub const FRONT_RIGHT: u32 = 0x0401;
pub const FULL_SUPPORT: u32 = 0x82B7;
pub const FUNC_ADD: u32 = 0x8006;
pub const FUNC_REVERSE_SUBTRACT: u32 = 0x800B;
pub const FUNC_SUBTRACT: u32 = 0x800A;
pub const GEOMETRY_INPUT_TYPE: u32 = 0x8917;
pub const GEOMETRY_OUTPUT_TYPE: u32 = 0x8918;
pub const GEOMETRY_SHADER: u32 = 0x8DD9;
pub const GEOMETRY_SHADER_BIT: u32 = 0x00000004;
pub const GEOMETRY_SHADER_INVOCATIONS: u32 = 0x887F;
pub const GEOMETRY_SHADER_PRIMITIVES_EMITTED: u32 = 0x82F3;
pub const GEOMETRY_SUBROUTINE: u32 = 0x92EB;
pub const GEOMETRY_SUBROUTINE_UNIFORM: u32 = 0x92F1;
pub const GEOMETRY_TEXTURE: u32 = 0x829E;
pub const GEOMETRY_VERTICES_OUT: u32 = 0x8916;
pub const GEQUAL: u32 = 0x0206;
pub const GET_TEXTURE_IMAGE_FORMAT: u32 = 0x8291;
pub const GET_TEXTURE_IMAGE_TYPE: u32 = 0x8292;
pub const GREATER: u32 = 0x0204;
pub const GREEN: u32 = 0x1904;
pub const GREEN_INTEGER: u32 = 0x8D95;
pub const GUILTY_CONTEXT_RESET: u32 = 0x8253;
pub const HALF_FLOAT: u32 = 0x140B;
pub const HIGH_FLOAT: u32 = 0x8DF2;
pub const HIGH_INT: u32 = 0x8DF5;
pub const IMAGE_1D: u32 = 0x904C;
pub const IMAGE_1D_ARRAY: u32 = 0x9052;
pub const IMAGE_2D: u32 = 0x904D;
pub const IMAGE_2D_ARRAY: u32 = 0x9053;
pub const IMAGE_2D_MULTISAMPLE: u32 = 0x9055;
pub const IMAGE_2D_MULTISAMPLE_ARRAY: u32 = 0x9056;
pub const IMAGE_2D_RECT: u32 = 0x904F;
pub const IMAGE_3D: u32 = 0x904E;
pub const IMAGE_BINDING_ACCESS: u32 = 0x8F3E;
pub const IMAGE_BINDING_FORMAT: u32 = 0x906E;
pub const IMAGE_BINDING_LAYER: u32 = 0x8F3D;
pub const IMAGE_BINDING_LAYERED: u32 = 0x8F3C;
pub const IMAGE_BINDING_LEVEL: u32 = 0x8F3B;
pub const IMAGE_BINDING_NAME: u32 = 0x8F3A;
pub const IMAGE_BUFFER: u32 = 0x9051;
pub const IMAGE_CLASS_10_10_10_2: u32 = 0x82C3;
pub const IMAGE_CLASS_11_11_10: u32 = 0x82C2;
pub const IMAGE_CLASS_1_X_16: u32 = 0x82BE;
pub const IMAGE_CLASS_1_X_32: u32 = 0x82BB;
pub const IMAGE_CLASS_1_X_8: u32 = 0x82C1;
pub const IMAGE_CLASS_2_X_16: u32 = 0x82BD;
pub const IMAGE_CLASS_2_X_32: u32 = 0x82BA;
pub const IMAGE_CLASS_2_X_8: u32 = 0x82C0;
pub const IMAGE_CLASS_4_X_16: u32 = 0x82BC;
pub const IMAGE_CLASS_4_X_32: u32 = 0x82B9;
pub const IMAGE_CLASS_4_X_8: u32 = 0x82BF;
pub const IMAGE_COMPATIBILITY_CLASS: u32 = 0x82A8;
pub const IMAGE_CUBE: u32 = 0x9050;
pub const IMAGE_CUBE_MAP_ARRAY: u32 = 0x9054;
pub const IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: u32 = 0x90C9;
pub const IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: u32 = 0x90C8;
pub const IMAGE_FORMAT_COMPATIBILITY_TYPE: u32 = 0x90C7;
pub const IMAGE_PIXEL_FORMAT: u32 = 0x82A9;
pub const IMAGE_PIXEL_TYPE: u32 = 0x82AA;
pub const IMAGE_TEXEL_SIZE: u32 = 0x82A7;
pub const IMPLEMENTATION_COLOR_READ_FORMAT: u32 = 0x8B9B;
pub const IMPLEMENTATION_COLOR_READ_TYPE: u32 = 0x8B9A;
pub const INCR: u32 = 0x1E02;
pub const INCR_WRAP: u32 = 0x8507;
pub const INFO_LOG_LENGTH: u32 = 0x8B84;
pub const INNOCENT_CONTEXT_RESET: u32 = 0x8254;
pub const INT: u32 = 0x1404;
pub const INTERLEAVED_ATTRIBS: u32 = 0x8C8C;
pub const INTERNALFORMAT_ALPHA_SIZE: u32 = 0x8274;
pub const INTERNALFORMAT_ALPHA_TYPE: u32 = 0x827B;
pub const INTERNALFORMAT_BLUE_SIZE: u32 = 0x8273;
pub const INTERNALFORMAT_BLUE_TYPE: u32 = 0x827A;
pub const INTERNALFORMAT_DEPTH_SIZE: u32 = 0x8275;
pub const INTERNALFORMAT_DEPTH_TYPE: u32 = 0x827C;
pub const INTERNALFORMAT_GREEN_SIZE: u32 = 0x8272;
pub const INTERNALFORMAT_GREEN_TYPE: u32 = 0x8279;
pub const INTERNALFORMAT_PREFERRED: u32 = 0x8270;
pub const INTERNALFORMAT_RED_SIZE: u32 = 0x8271;
pub const INTERNALFORMAT_RED_TYPE: u32 = 0x8278;
pub const INTERNALFORMAT_SHARED_SIZE: u32 = 0x8277;
pub const INTERNALFORMAT_STENCIL_SIZE: u32 = 0x8276;
pub const INTERNALFORMAT_STENCIL_TYPE: u32 = 0x827D;
pub const INTERNALFORMAT_SUPPORTED: u32 = 0x826F;
pub const INT_2_10_10_10_REV: u32 = 0x8D9F;
pub const INT_IMAGE_1D: u32 = 0x9057;
pub const INT_IMAGE_1D_ARRAY: u32 = 0x905D;
pub const INT_IMAGE_2D: u32 = 0x9058;
pub const INT_IMAGE_2D_ARRAY: u32 = 0x905E;
pub const INT_IMAGE_2D_MULTISAMPLE: u32 = 0x9060;
pub const INT_IMAGE_2D_MULTISAMPLE_ARRAY: u32 = 0x9061;
pub const INT_IMAGE_2D_RECT: u32 = 0x905A;
pub const INT_IMAGE_3D: u32 = 0x9059;
pub const INT_IMAGE_BUFFER: u32 = 0x905C;
pub const INT_IMAGE_CUBE: u32 = 0x905B;
pub const INT_IMAGE_CUBE_MAP_ARRAY: u32 = 0x905F;
pub const INT_SAMPLER_1D: u32 = 0x8DC9;
pub const INT_SAMPLER_1D_ARRAY: u32 = 0x8DCE;
pub const INT_SAMPLER_2D: u32 = 0x8DCA;
pub const INT_SAMPLER_2D_ARRAY: u32 = 0x8DCF;
pub const INT_SAMPLER_2D_MULTISAMPLE: u32 = 0x9109;
pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: u32 = 0x910C;
pub const INT_SAMPLER_2D_RECT: u32 = 0x8DCD;
pub const INT_SAMPLER_3D: u32 = 0x8DCB;
pub const INT_SAMPLER_BUFFER: u32 = 0x8DD0;
pub const INT_SAMPLER_CUBE: u32 = 0x8DCC;
pub const INT_SAMPLER_CUBE_MAP_ARRAY: u32 = 0x900E;
pub const INT_VEC2: u32 = 0x8B53;
pub const INT_VEC3: u32 = 0x8B54;
pub const INT_VEC4: u32 = 0x8B55;
pub const INVALID_ENUM: u32 = 0x0500;
pub const INVALID_FRAMEBUFFER_OPERATION: u32 = 0x0506;
pub const INVALID_INDEX: u32 = 0xFFFFFFFF;
pub const INVALID_OPERATION: u32 = 0x0502;
pub const INVALID_VALUE: u32 = 0x0501;
pub const INVERT: u32 = 0x150A;
pub const ISOLINES: u32 = 0x8E7A;
pub const IS_PER_PATCH: u32 = 0x92E7;
pub const IS_ROW_MAJOR: u32 = 0x9300;
pub const KEEP: u32 = 0x1E00;
pub const LAST_VERTEX_CONVENTION: u32 = 0x8E4E;
pub const LAYER_PROVOKING_VERTEX: u32 = 0x825E;
pub const LEFT: u32 = 0x0406;
pub const LEQUAL: u32 = 0x0203;
pub const LESS: u32 = 0x0201;
pub const LINE: u32 = 0x1B01;
pub const LINEAR: u32 = 0x2601;
pub const LINEAR_MIPMAP_LINEAR: u32 = 0x2703;
pub const LINEAR_MIPMAP_NEAREST: u32 = 0x2701;
pub const LINES: u32 = 0x0001;
pub const LINES_ADJACENCY: u32 = 0x000A;
pub const LINE_LOOP: u32 = 0x0002;
pub const LINE_SMOOTH: u32 = 0x0B20;
pub const LINE_SMOOTH_HINT: u32 = 0x0C52;
pub const LINE_STRIP: u32 = 0x0003;
pub const LINE_STRIP_ADJACENCY: u32 = 0x000B;
pub const LINE_WIDTH: u32 = 0x0B21;
pub const LINE_WIDTH_GRANULARITY: u32 = 0x0B23;
pub const LINE_WIDTH_RANGE: u32 = 0x0B22;
pub const LINK_STATUS: u32 = 0x8B82;
pub const LOCATION: u32 = 0x930E;
pub const LOCATION_COMPONENT: u32 = 0x934A;
pub const LOCATION_INDEX: u32 = 0x930F;
pub const LOGIC_OP_MODE: u32 = 0x0BF0;
pub const LOSE_CONTEXT_ON_RESET: u32 = 0x8252;
pub const LOWER_LEFT: u32 = 0x8CA1;
pub const LOW_FLOAT: u32 = 0x8DF0;
pub const LOW_INT: u32 = 0x8DF3;
pub const MAJOR_VERSION: u32 = 0x821B;
pub const MANUAL_GENERATE_MIPMAP: u32 = 0x8294;
pub const MAP_COHERENT_BIT: u32 = 0x0080;
pub const MAP_COHERENT_BIT_EXT: u32 = 0x0080;
pub const MAP_FLUSH_EXPLICIT_BIT: u32 = 0x0010;
pub const MAP_INVALIDATE_BUFFER_BIT: u32 = 0x0008;
pub const MAP_INVALIDATE_RANGE_BIT: u32 = 0x0004;
pub const MAP_PERSISTENT_BIT: u32 = 0x0040;
pub const MAP_PERSISTENT_BIT_EXT: u32 = 0x0040;
pub const MAP_READ_BIT: u32 = 0x0001;
pub const MAP_UNSYNCHRONIZED_BIT: u32 = 0x0020;
pub const MAP_WRITE_BIT: u32 = 0x0002;
pub const MATRIX_STRIDE: u32 = 0x92FF;
pub const MAX: u32 = 0x8008;
pub const MAX_3D_TEXTURE_SIZE: u32 = 0x8073;
pub const MAX_ARRAY_TEXTURE_LAYERS: u32 = 0x88FF;
pub const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: u32 = 0x92DC;
pub const MAX_ATOMIC_COUNTER_BUFFER_SIZE: u32 = 0x92D8;
pub const MAX_CLIP_DISTANCES: u32 = 0x0D32;
pub const MAX_COLOR_ATTACHMENTS: u32 = 0x8CDF;
pub const MAX_COLOR_TEXTURE_SAMPLES: u32 = 0x910E;
pub const MAX_COMBINED_ATOMIC_COUNTERS: u32 = 0x92D7;
pub const MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: u32 = 0x92D1;
pub const MAX_COMBINED_CLIP_AND_CULL_DISTANCES: u32 = 0x82FA;
pub const MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: u32 = 0x8266;
pub const MAX_COMBINED_DIMENSIONS: u32 = 0x8282;
pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: u32 = 0x8A33;
pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: u32 = 0x8A32;
pub const MAX_COMBINED_IMAGE_UNIFORMS: u32 = 0x90CF;
pub const MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: u32 = 0x8F39;
pub const MAX_COMBINED_SHADER_OUTPUT_RESOURCES: u32 = 0x8F39;
pub const MAX_COMBINED_SHADER_STORAGE_BLOCKS: u32 = 0x90DC;
pub const MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: u32 = 0x8E1E;
pub const MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: u32 = 0x8E1F;
pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 0x8B4D;
pub const MAX_COMBINED_UNIFORM_BLOCKS: u32 = 0x8A2E;
pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: u32 = 0x8A31;
pub const MAX_COMPUTE_ATOMIC_COUNTERS: u32 = 0x8265;
pub const MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: u32 = 0x8264;
pub const MAX_COMPUTE_IMAGE_UNIFORMS: u32 = 0x91BD;
pub const MAX_COMPUTE_SHADER_STORAGE_BLOCKS: u32 = 0x90DB;
pub const MAX_COMPUTE_SHARED_MEMORY_SIZE: u32 = 0x8262;
pub const MAX_COMPUTE_TEXTURE_IMAGE_UNITS: u32 = 0x91BC;
pub const MAX_COMPUTE_UNIFORM_BLOCKS: u32 = 0x91BB;
pub const MAX_COMPUTE_UNIFORM_COMPONENTS: u32 = 0x8263;
pub const MAX_COMPUTE_WORK_GROUP_COUNT: u32 = 0x91BE;
pub const MAX_COMPUTE_WORK_GROUP_INVOCATIONS: u32 = 0x90EB;
pub const MAX_COMPUTE_WORK_GROUP_SIZE: u32 = 0x91BF;
pub const MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 0x851C;
pub const MAX_CULL_DISTANCES: u32 = 0x82F9;
pub const MAX_DEBUG_GROUP_STACK_DEPTH: u32 = 0x826C;
pub const MAX_DEBUG_LOGGED_MESSAGES: u32 = 0x9144;
pub const MAX_DEBUG_LOGGED_MESSAGES_ARB: u32 = 0x9144;
pub const MAX_DEBUG_MESSAGE_LENGTH: u32 = 0x9143;
pub const MAX_DEBUG_MESSAGE_LENGTH_ARB: u32 = 0x9143;
pub const MAX_DEPTH: u32 = 0x8280;
pub const MAX_DEPTH_TEXTURE_SAMPLES: u32 = 0x910F;
pub const MAX_DRAW_BUFFERS: u32 = 0x8824;
pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: u32 = 0x88FC;
pub const MAX_ELEMENTS_INDICES: u32 = 0x80E9;
pub const MAX_ELEMENTS_VERTICES: u32 = 0x80E8;
pub const MAX_ELEMENT_INDEX: u32 = 0x8D6B;
pub const MAX_FRAGMENT_ATOMIC_COUNTERS: u32 = 0x92D6;
pub const MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: u32 = 0x92D0;
pub const MAX_FRAGMENT_IMAGE_UNIFORMS: u32 = 0x90CE;
pub const MAX_FRAGMENT_INPUT_COMPONENTS: u32 = 0x9125;
pub const MAX_FRAGMENT_INTERPOLATION_OFFSET: u32 = 0x8E5C;
pub const MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: u32 = 0x90DA;
pub const MAX_FRAGMENT_UNIFORM_BLOCKS: u32 = 0x8A2D;
pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: u32 = 0x8B49;
pub const MAX_FRAGMENT_UNIFORM_VECTORS: u32 = 0x8DFD;
pub const MAX_FRAMEBUFFER_HEIGHT: u32 = 0x9316;
pub const MAX_FRAMEBUFFER_LAYERS: u32 = 0x9317;
pub const MAX_FRAMEBUFFER_SAMPLES: u32 = 0x9318;
pub const MAX_FRAMEBUFFER_WIDTH: u32 = 0x9315;
pub const MAX_GEOMETRY_ATOMIC_COUNTERS: u32 = 0x92D5;
pub const MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: u32 = 0x92CF;
pub const MAX_GEOMETRY_IMAGE_UNIFORMS: u32 = 0x90CD;
pub const MAX_GEOMETRY_INPUT_COMPONENTS: u32 = 0x9123;
pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: u32 = 0x9124;
pub const MAX_GEOMETRY_OUTPUT_VERTICES: u32 = 0x8DE0;
pub const MAX_GEOMETRY_SHADER_INVOCATIONS: u32 = 0x8E5A;
pub const MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: u32 = 0x90D7;
pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: u32 = 0x8C29;
pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: u32 = 0x8DE1;
pub const MAX_GEOMETRY_UNIFORM_BLOCKS: u32 = 0x8A2C;
pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: u32 = 0x8DDF;
pub const MAX_HEIGHT: u32 = 0x827F;
pub const MAX_IMAGE_SAMPLES: u32 = 0x906D;
pub const MAX_IMAGE_UNITS: u32 = 0x8F38;
pub const MAX_INTEGER_SAMPLES: u32 = 0x9110;
pub const MAX_LABEL_LENGTH: u32 = 0x82E8;
pub const MAX_LAYERS: u32 = 0x8281;
pub const MAX_NAME_LENGTH: u32 = 0x92F6;
pub const MAX_NUM_ACTIVE_VARIABLES: u32 = 0x92F7;
pub const MAX_NUM_COMPATIBLE_SUBROUTINES: u32 = 0x92F8;
pub const MAX_PATCH_VERTICES: u32 = 0x8E7D;
pub const MAX_PROGRAM_TEXEL_OFFSET: u32 = 0x8905;
pub const MAX_PROGRAM_TEXTURE_GATHER_OFFSET: u32 = 0x8E5F;
pub const MAX_RECTANGLE_TEXTURE_SIZE: u32 = 0x84F8;
pub const MAX_RENDERBUFFER_SIZE: u32 = 0x84E8;
pub const MAX_SAMPLES: u32 = 0x8D57;
pub const MAX_SAMPLE_MASK_WORDS: u32 = 0x8E59;
pub const MAX_SERVER_WAIT_TIMEOUT: u32 = 0x9111;
pub const MAX_SHADER_STORAGE_BLOCK_SIZE: u32 = 0x90DE;
pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: u32 = 0x90DD;
pub const MAX_SUBROUTINES: u32 = 0x8DE7;
pub const MAX_SUBROUTINE_UNIFORM_LOCATIONS: u32 = 0x8DE8;
pub const MAX_TESS_CONTROL_ATOMIC_COUNTERS: u32 = 0x92D3;
pub const MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: u32 = 0x92CD;
pub const MAX_TESS_CONTROL_IMAGE_UNIFORMS: u32 = 0x90CB;
pub const MAX_TESS_CONTROL_INPUT_COMPONENTS: u32 = 0x886C;
pub const MAX_TESS_CONTROL_OUTPUT_COMPONENTS: u32 = 0x8E83;
pub const MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: u32 = 0x90D8;
pub const MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: u32 = 0x8E81;
pub const MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: u32 = 0x8E85;
pub const MAX_TESS_CONTROL_UNIFORM_BLOCKS: u32 = 0x8E89;
pub const MAX_TESS_CONTROL_UNIFORM_COMPONENTS: u32 = 0x8E7F;
pub const MAX_TESS_EVALUATION_ATOMIC_COUNTERS: u32 = 0x92D4;
pub const MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: u32 = 0x92CE;
pub const MAX_TESS_EVALUATION_IMAGE_UNIFORMS: u32 = 0x90CC;
pub const MAX_TESS_EVALUATION_INPUT_COMPONENTS: u32 = 0x886D;
pub const MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: u32 = 0x8E86;
pub const MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: u32 = 0x90D9;
pub const MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: u32 = 0x8E82;
pub const MAX_TESS_EVALUATION_UNIFORM_BLOCKS: u32 = 0x8E8A;
pub const MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: u32 = 0x8E80;
pub const MAX_TESS_GEN_LEVEL: u32 = 0x8E7E;
pub const MAX_TESS_PATCH_COMPONENTS: u32 = 0x8E84;
pub const MAX_TEXTURE_BUFFER_SIZE: u32 = 0x8C2B;
pub const MAX_TEXTURE_IMAGE_UNITS: u32 = 0x8872;
pub const MAX_TEXTURE_LOD_BIAS: u32 = 0x84FD;
pub const MAX_TEXTURE_MAX_ANISOTROPY: u32 = 0x84FF;
pub const MAX_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FF;
pub const MAX_TEXTURE_SIZE: u32 = 0x0D33;
pub const MAX_TRANSFORM_FEEDBACK_BUFFERS: u32 = 0x8E70;
pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: u32 = 0x8C8A;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: u32 = 0x8C8B;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: u32 = 0x8C80;
pub const MAX_UNIFORM_BLOCK_SIZE: u32 = 0x8A30;
pub const MAX_UNIFORM_BUFFER_BINDINGS: u32 = 0x8A2F;
pub const MAX_UNIFORM_LOCATIONS: u32 = 0x826E;
pub const MAX_VARYING_COMPONENTS: u32 = 0x8B4B;
pub const MAX_VARYING_FLOATS: u32 = 0x8B4B;
pub const MAX_VARYING_VECTORS: u32 = 0x8DFC;
pub const MAX_VERTEX_ATOMIC_COUNTERS: u32 = 0x92D2;
pub const MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: u32 = 0x92CC;
pub const MAX_VERTEX_ATTRIBS: u32 = 0x8869;
pub const MAX_VERTEX_ATTRIB_BINDINGS: u32 = 0x82DA;
pub const MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: u32 = 0x82D9;
pub const MAX_VERTEX_ATTRIB_STRIDE: u32 = 0x82E5;
pub const MAX_VERTEX_IMAGE_UNIFORMS: u32 = 0x90CA;
pub const MAX_VERTEX_OUTPUT_COMPONENTS: u32 = 0x9122;
pub const MAX_VERTEX_SHADER_STORAGE_BLOCKS: u32 = 0x90D6;
pub const MAX_VERTEX_STREAMS: u32 = 0x8E71;
pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: u32 = 0x8B4C;
pub const MAX_VERTEX_UNIFORM_BLOCKS: u32 = 0x8A2B;
pub const MAX_VERTEX_UNIFORM_COMPONENTS: u32 = 0x8B4A;
pub const MAX_VERTEX_UNIFORM_VECTORS: u32 = 0x8DFB;
pub const MAX_VIEWPORTS: u32 = 0x825B;
pub const MAX_VIEWPORT_DIMS: u32 = 0x0D3A;
pub const MAX_WIDTH: u32 = 0x827E;
pub const MEDIUM_FLOAT: u32 = 0x8DF1;
pub const MEDIUM_INT: u32 = 0x8DF4;
pub const MIN: u32 = 0x8007;
pub const MINOR_VERSION: u32 = 0x821C;
pub const MIN_FRAGMENT_INTERPOLATION_OFFSET: u32 = 0x8E5B;
pub const MIN_MAP_BUFFER_ALIGNMENT: u32 = 0x90BC;
pub const MIN_PROGRAM_TEXEL_OFFSET: u32 = 0x8904;
pub const MIN_PROGRAM_TEXTURE_GATHER_OFFSET: u32 = 0x8E5E;
pub const MIN_SAMPLE_SHADING_VALUE: u32 = 0x8C37;
pub const MIPMAP: u32 = 0x8293;
pub const MIRRORED_REPEAT: u32 = 0x8370;
pub const MIRROR_CLAMP_TO_EDGE: u32 = 0x8743;
pub const MULTISAMPLE: u32 = 0x809D;
pub const NAME_LENGTH: u32 = 0x92F9;
pub const NAND: u32 = 0x150E;
pub const NEAREST: u32 = 0x2600;
pub const NEAREST_MIPMAP_LINEAR: u32 = 0x2702;
pub const NEAREST_MIPMAP_NEAREST: u32 = 0x2700;
pub const NEGATIVE_ONE_TO_ONE: u32 = 0x935E;
pub const NEVER: u32 = 0x0200;
pub const NICEST: u32 = 0x1102;
pub const NONE: u32 = 0;
pub const NOOP: u32 = 0x1505;
pub const NOR: u32 = 0x1508;
pub const NOTEQUAL: u32 = 0x0205;
pub const NO_ERROR: u32 = 0;
pub const NO_RESET_NOTIFICATION: u32 = 0x8261;
pub const NUM_ACTIVE_VARIABLES: u32 = 0x9304;
pub const NUM_COMPATIBLE_SUBROUTINES: u32 = 0x8E4A;
pub const NUM_COMPRESSED_TEXTURE_FORMATS: u32 = 0x86A2;
pub const NUM_EXTENSIONS: u32 = 0x821D;
pub const NUM_PROGRAM_BINARY_FORMATS: u32 = 0x87FE;
pub const NUM_SAMPLE_COUNTS: u32 = 0x9380;
pub const NUM_SHADER_BINARY_FORMATS: u32 = 0x8DF9;
pub const NUM_SHADING_LANGUAGE_VERSIONS: u32 = 0x82E9;
pub const NUM_SPIR_V_EXTENSIONS: u32 = 0x9554;
pub const OBJECT_TYPE: u32 = 0x9112;
pub const OFFSET: u32 = 0x92FC;
pub const ONE: u32 = 1;
pub const ONE_MINUS_CONSTANT_ALPHA: u32 = 0x8004;
pub const ONE_MINUS_CONSTANT_COLOR: u32 = 0x8002;
pub const ONE_MINUS_DST_ALPHA: u32 = 0x0305;
pub const ONE_MINUS_DST_COLOR: u32 = 0x0307;
pub const ONE_MINUS_SRC1_ALPHA: u32 = 0x88FB;
pub const ONE_MINUS_SRC1_COLOR: u32 = 0x88FA;
pub const ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
pub const ONE_MINUS_SRC_COLOR: u32 = 0x0301;
pub const OR: u32 = 0x1507;
pub const OR_INVERTED: u32 = 0x150D;
pub const OR_REVERSE: u32 = 0x150B;
pub const OUT_OF_MEMORY: u32 = 0x0505;
pub const PACK_ALIGNMENT: u32 = 0x0D05;
pub const PACK_COMPRESSED_BLOCK_DEPTH: u32 = 0x912D;
pub const PACK_COMPRESSED_BLOCK_HEIGHT: u32 = 0x912C;
pub const PACK_COMPRESSED_BLOCK_SIZE: u32 = 0x912E;
pub const PACK_COMPRESSED_BLOCK_WIDTH: u32 = 0x912B;
pub const PACK_IMAGE_HEIGHT: u32 = 0x806C;
pub const PACK_LSB_FIRST: u32 = 0x0D01;
pub const PACK_ROW_LENGTH: u32 = 0x0D02;
pub const PACK_SKIP_IMAGES: u32 = 0x806B;
pub const PACK_SKIP_PIXELS: u32 = 0x0D04;
pub const PACK_SKIP_ROWS: u32 = 0x0D03;
pub const PACK_SWAP_BYTES: u32 = 0x0D00;
pub const PARAMETER_BUFFER: u32 = 0x80EE;
pub const PARAMETER_BUFFER_BINDING: u32 = 0x80EF;
pub const PATCHES: u32 = 0x000E;
pub const PATCH_DEFAULT_INNER_LEVEL: u32 = 0x8E73;
pub const PATCH_DEFAULT_OUTER_LEVEL: u32 = 0x8E74;
pub const PATCH_VERTICES: u32 = 0x8E72;
pub const PIXEL_BUFFER_BARRIER_BIT: u32 = 0x00000080;
pub const PIXEL_PACK_BUFFER: u32 = 0x88EB;
pub const PIXEL_PACK_BUFFER_BINDING: u32 = 0x88ED;
pub const PIXEL_UNPACK_BUFFER: u32 = 0x88EC;
pub const PIXEL_UNPACK_BUFFER_BINDING: u32 = 0x88EF;
pub const POINT: u32 = 0x1B00;
pub const POINTS: u32 = 0x0000;
pub const POINT_FADE_THRESHOLD_SIZE: u32 = 0x8128;
pub const POINT_SIZE: u32 = 0x0B11;
pub const POINT_SIZE_GRANULARITY: u32 = 0x0B13;
pub const POINT_SIZE_RANGE: u32 = 0x0B12;
pub const POINT_SPRITE_COORD_ORIGIN: u32 = 0x8CA0;
pub const POLYGON_MODE: u32 = 0x0B40;
pub const POLYGON_OFFSET_CLAMP: u32 = 0x8E1B;
pub const POLYGON_OFFSET_FACTOR: u32 = 0x8038;
pub const POLYGON_OFFSET_FILL: u32 = 0x8037;
pub const POLYGON_OFFSET_LINE: u32 = 0x2A02;
pub const POLYGON_OFFSET_POINT: u32 = 0x2A01;
pub const POLYGON_OFFSET_UNITS: u32 = 0x2A00;
pub const POLYGON_SMOOTH: u32 = 0x0B41;
pub const POLYGON_SMOOTH_HINT: u32 = 0x0C53;
pub const PRIMITIVES_GENERATED: u32 = 0x8C87;
pub const PRIMITIVES_SUBMITTED: u32 = 0x82EF;
pub const PRIMITIVE_RESTART: u32 = 0x8F9D;
pub const PRIMITIVE_RESTART_FIXED_INDEX: u32 = 0x8D69;
pub const PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: u32 = 0x8221;
pub const PRIMITIVE_RESTART_INDEX: u32 = 0x8F9E;
pub const PROGRAM: u32 = 0x82E2;
pub const PROGRAM_BINARY_FORMATS: u32 = 0x87FF;
pub const PROGRAM_BINARY_LENGTH: u32 = 0x8741;
pub const PROGRAM_BINARY_RETRIEVABLE_HINT: u32 = 0x8257;
pub const PROGRAM_INPUT: u32 = 0x92E3;
pub const PROGRAM_OUTPUT: u32 = 0x92E4;
pub const PROGRAM_PIPELINE: u32 = 0x82E4;
pub const PROGRAM_PIPELINE_BINDING: u32 = 0x825A;
pub const PROGRAM_POINT_SIZE: u32 = 0x8642;
pub const PROGRAM_SEPARABLE: u32 = 0x8258;
pub const PROVOKING_VERTEX: u32 = 0x8E4F;
pub const PROXY_TEXTURE_1D: u32 = 0x8063;
pub const PROXY_TEXTURE_1D_ARRAY: u32 = 0x8C19;
pub const PROXY_TEXTURE_2D: u32 = 0x8064;
pub const PROXY_TEXTURE_2D_ARRAY: u32 = 0x8C1B;
pub const PROXY_TEXTURE_2D_MULTISAMPLE: u32 = 0x9101;
pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: u32 = 0x9103;
pub const PROXY_TEXTURE_3D: u32 = 0x8070;
pub const PROXY_TEXTURE_CUBE_MAP: u32 = 0x851B;
pub const PROXY_TEXTURE_CUBE_MAP_ARRAY: u32 = 0x900B;
pub const PROXY_TEXTURE_RECTANGLE: u32 = 0x84F7;
pub const QUADS: u32 = 0x0007;
pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: u32 = 0x8E4C;
pub const QUERY: u32 = 0x82E3;
pub const QUERY_BUFFER: u32 = 0x9192;
pub const QUERY_BUFFER_BARRIER_BIT: u32 = 0x00008000;
pub const QUERY_BUFFER_BINDING: u32 = 0x9193;
pub const QUERY_BY_REGION_NO_WAIT: u32 = 0x8E16;
pub const QUERY_BY_REGION_NO_WAIT_INVERTED: u32 = 0x8E1A;
pub const QUERY_BY_REGION_WAIT: u32 = 0x8E15;
pub const QUERY_BY_REGION_WAIT_INVERTED: u32 = 0x8E19;
pub const QUERY_COUNTER_BITS: u32 = 0x8864;
pub const QUERY_NO_WAIT: u32 = 0x8E14;
pub const QUERY_NO_WAIT_INVERTED: u32 = 0x8E18;
pub const QUERY_RESULT: u32 = 0x8866;
pub const QUERY_RESULT_AVAILABLE: u32 = 0x8867;
pub const QUERY_RESULT_NO_WAIT: u32 = 0x9194;
pub const QUERY_TARGET: u32 = 0x82EA;
pub const QUERY_WAIT: u32 = 0x8E13;
pub const QUERY_WAIT_INVERTED: u32 = 0x8E17;
pub const R11F_G11F_B10F: u32 = 0x8C3A;
pub const R16: u32 = 0x822A;
pub const R16F: u32 = 0x822D;
pub const R16I: u32 = 0x8233;
pub const R16UI: u32 = 0x8234;
pub const R16_SNORM: u32 = 0x8F98;
pub const R32F: u32 = 0x822E;
pub const R32I: u32 = 0x8235;
pub const R32UI: u32 = 0x8236;
pub const R3_G3_B2: u32 = 0x2A10;
pub const R8: u32 = 0x8229;
pub const R8I: u32 = 0x8231;
pub const R8UI: u32 = 0x8232;
pub const R8_SNORM: u32 = 0x8F94;
pub const RASTERIZER_DISCARD: u32 = 0x8C89;
pub const READ_BUFFER: u32 = 0x0C02;
pub const READ_FRAMEBUFFER: u32 = 0x8CA8;
pub const READ_FRAMEBUFFER_BINDING: u32 = 0x8CAA;
pub const READ_ONLY: u32 = 0x88B8;
pub const READ_PIXELS: u32 = 0x828C;
pub const READ_PIXELS_FORMAT: u32 = 0x828D;
pub const READ_PIXELS_TYPE: u32 = 0x828E;
pub const READ_WRITE: u32 = 0x88BA;
pub const RED: u32 = 0x1903;
pub const RED_INTEGER: u32 = 0x8D94;
pub const REFERENCED_BY_COMPUTE_SHADER: u32 = 0x930B;
pub const REFERENCED_BY_FRAGMENT_SHADER: u32 = 0x930A;
pub const REFERENCED_BY_GEOMETRY_SHADER: u32 = 0x9309;
pub const REFERENCED_BY_TESS_CONTROL_SHADER: u32 = 0x9307;
pub const REFERENCED_BY_TESS_EVALUATION_SHADER: u32 = 0x9308;
pub const REFERENCED_BY_VERTEX_SHADER: u32 = 0x9306;
pub const RENDERBUFFER: u32 = 0x8D41;
pub const RENDERBUFFER_ALPHA_SIZE: u32 = 0x8D53;
pub const RENDERBUFFER_BINDING: u32 = 0x8CA7;
pub const RENDERBUFFER_BLUE_SIZE: u32 = 0x8D52;
pub const RENDERBUFFER_DEPTH_SIZE: u32 = 0x8D54;
pub const RENDERBUFFER_GREEN_SIZE: u32 = 0x8D51;
pub const RENDERBUFFER_HEIGHT: u32 = 0x8D43;
pub const RENDERBUFFER_INTERNAL_FORMAT: u32 = 0x8D44;
pub const RENDERBUFFER_RED_SIZE: u32 = 0x8D50;
pub const RENDERBUFFER_SAMPLES: u32 = 0x8CAB;
pub const RENDERBUFFER_STENCIL_SIZE: u32 = 0x8D55;
pub const RENDERBUFFER_WIDTH: u32 = 0x8D42;
pub const RENDERER: u32 = 0x1F01;
pub const REPEAT: u32 = 0x2901;
pub const REPLACE: u32 = 0x1E01;
pub const RESET_NOTIFICATION_STRATEGY: u32 = 0x8256;
pub const RG: u32 = 0x8227;
pub const RG16: u32 = 0x822C;
pub const RG16F: u32 = 0x822F;
pub const RG16I: u32 = 0x8239;
pub const RG16UI: u32 = 0x823A;
pub const RG16_SNORM: u32 = 0x8F99;
pub const RG32F: u32 = 0x8230;
pub const RG32I: u32 = 0x823B;
pub const RG32UI: u32 = 0x823C;
pub const RG8: u32 = 0x822B;
pub const RG8I: u32 = 0x8237;
pub const RG8UI: u32 = 0x8238;
pub const RG8_SNORM: u32 = 0x8F95;
pub const RGB: u32 = 0x1907;
pub const RGB10: u32 = 0x8052;
pub const RGB10_A2: u32 = 0x8059;
pub const RGB10_A2UI: u32 = 0x906F;
pub const RGB12: u32 = 0x8053;
pub const RGB16: u32 = 0x8054;
pub const RGB16F: u32 = 0x881B;
pub const RGB16I: u32 = 0x8D89;
pub const RGB16UI: u32 = 0x8D77;
pub const RGB16_SNORM: u32 = 0x8F9A;
pub const RGB32F: u32 = 0x8815;
pub const RGB32I: u32 = 0x8D83;
pub const RGB32UI: u32 = 0x8D71;
pub const RGB4: u32 = 0x804F;
pub const RGB5: u32 = 0x8050;
pub const RGB565: u32 = 0x8D62;
pub const RGB5_A1: u32 = 0x8057;
pub const RGB8: u32 = 0x8051;
pub const RGB8I: u32 = 0x8D8F;
pub const RGB8UI: u32 = 0x8D7D;
pub const RGB8_SNORM: u32 = 0x8F96;
pub const RGB9_E5: u32 = 0x8C3D;
pub const RGBA: u32 = 0x1908;
pub const RGBA12: u32 = 0x805A;
pub const RGBA16: u32 = 0x805B;
pub const RGBA16F: u32 = 0x881A;
pub const RGBA16I: u32 = 0x8D88;
pub const RGBA16UI: u32 = 0x8D76;
pub const RGBA16_SNORM: u32 = 0x8F9B;
pub const RGBA2: u32 = 0x8055;
pub const RGBA32F: u32 = 0x8814;
pub const RGBA32I: u32 = 0x8D82;
pub const RGBA32UI: u32 = 0x8D70;
pub const RGBA4: u32 = 0x8056;
pub const RGBA8: u32 = 0x8058;
pub const RGBA8I: u32 = 0x8D8E;
pub const RGBA8UI: u32 = 0x8D7C;
pub const RGBA8_SNORM: u32 = 0x8F97;
pub const RGBA_INTEGER: u32 = 0x8D99;
pub const RGB_INTEGER: u32 = 0x8D98;
pub const RG_INTEGER: u32 = 0x8228;
pub const RIGHT: u32 = 0x0407;
pub const SAMPLER: u32 = 0x82E6;
pub const SAMPLER_1D: u32 = 0x8B5D;
pub const SAMPLER_1D_ARRAY: u32 = 0x8DC0;
pub const SAMPLER_1D_ARRAY_SHADOW: u32 = 0x8DC3;
pub const SAMPLER_1D_SHADOW: u32 = 0x8B61;
pub const SAMPLER_2D: u32 = 0x8B5E;
pub const SAMPLER_2D_ARRAY: u32 = 0x8DC1;
pub const SAMPLER_2D_ARRAY_SHADOW: u32 = 0x8DC4;
pub const SAMPLER_2D_MULTISAMPLE: u32 = 0x9108;
pub const SAMPLER_2D_MULTISAMPLE_ARRAY: u32 = 0x910B;
pub const SAMPLER_2D_RECT: u32 = 0x8B63;
pub const SAMPLER_2D_RECT_SHADOW: u32 = 0x8B64;
pub const SAMPLER_2D_SHADOW: u32 = 0x8B62;
pub const SAMPLER_3D: u32 = 0x8B5F;
pub const SAMPLER_BINDING: u32 = 0x8919;
pub const SAMPLER_BUFFER: u32 = 0x8DC2;
pub const SAMPLER_CUBE: u32 = 0x8B60;
pub const SAMPLER_CUBE_MAP_ARRAY: u32 = 0x900C;
pub const SAMPLER_CUBE_MAP_ARRAY_SHADOW: u32 = 0x900D;
pub const SAMPLER_CUBE_SHADOW: u32 = 0x8DC5;
pub const SAMPLES: u32 = 0x80A9;
pub const SAMPLES_PASSED: u32 = 0x8914;
pub const SAMPLE_ALPHA_TO_COVERAGE: u32 = 0x809E;
pub const SAMPLE_ALPHA_TO_ONE: u32 = 0x809F;
pub const SAMPLE_BUFFERS: u32 = 0x80A8;
pub const SAMPLE_COVERAGE: u32 = 0x80A0;
pub const SAMPLE_COVERAGE_INVERT: u32 = 0x80AB;
pub const SAMPLE_COVERAGE_VALUE: u32 = 0x80AA;
pub const SAMPLE_MASK: u32 = 0x8E51;
pub const SAMPLE_MASK_VALUE: u32 = 0x8E52;
pub const SAMPLE_POSITION: u32 = 0x8E50;
pub const SAMPLE_SHADING: u32 = 0x8C36;
pub const SCISSOR_BOX: u32 = 0x0C10;
pub const SCISSOR_TEST: u32 = 0x0C11;
pub const SEPARATE_ATTRIBS: u32 = 0x8C8D;
pub const SET: u32 = 0x150F;
pub const SHADER: u32 = 0x82E1;
pub const SHADER_BINARY_FORMATS: u32 = 0x8DF8;
pub const SHADER_BINARY_FORMAT_SPIR_V: u32 = 0x9551;
pub const SHADER_COMPILER: u32 = 0x8DFA;
pub const SHADER_IMAGE_ACCESS_BARRIER_BIT: u32 = 0x00000020;
pub const SHADER_IMAGE_ATOMIC: u32 = 0x82A6;
pub const SHADER_IMAGE_LOAD: u32 = 0x82A4;
pub const SHADER_IMAGE_STORE: u32 = 0x82A5;
pub const SHADER_SOURCE_LENGTH: u32 = 0x8B88;
pub const SHADER_STORAGE_BARRIER_BIT: u32 = 0x00002000;
pub const SHADER_STORAGE_BLOCK: u32 = 0x92E6;
pub const SHADER_STORAGE_BUFFER: u32 = 0x90D2;
pub const SHADER_STORAGE_BUFFER_BINDING: u32 = 0x90D3;
pub const SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: u32 = 0x90DF;
pub const SHADER_STORAGE_BUFFER_SIZE: u32 = 0x90D5;
pub const SHADER_STORAGE_BUFFER_START: u32 = 0x90D4;
pub const SHADER_TYPE: u32 = 0x8B4F;
pub const SHADING_LANGUAGE_VERSION: u32 = 0x8B8C;
pub const SHORT: u32 = 0x1402;
pub const SIGNALED: u32 = 0x9119;
pub const SIGNED_NORMALIZED: u32 = 0x8F9C;
pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: u32 = 0x82AC;
pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: u32 = 0x82AE;
pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: u32 = 0x82AD;
pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: u32 = 0x82AF;
pub const SMOOTH_LINE_WIDTH_GRANULARITY: u32 = 0x0B23;
pub const SMOOTH_LINE_WIDTH_RANGE: u32 = 0x0B22;
pub const SMOOTH_POINT_SIZE_GRANULARITY: u32 = 0x0B13;
pub const SMOOTH_POINT_SIZE_RANGE: u32 = 0x0B12;
pub const SPIR_V_BINARY: u32 = 0x9552;
pub const SPIR_V_EXTENSIONS: u32 = 0x9553;
pub const SRC1_ALPHA: u32 = 0x8589;
pub const SRC1_COLOR: u32 = 0x88F9;
pub const SRC_ALPHA: u32 = 0x0302;
pub const SRC_ALPHA_SATURATE: u32 = 0x0308;
pub const SRC_COLOR: u32 = 0x0300;
pub const SRGB: u32 = 0x8C40;
pub const SRGB8: u32 = 0x8C41;
pub const SRGB8_ALPHA8: u32 = 0x8C43;
pub const SRGB_ALPHA: u32 = 0x8C42;
pub const SRGB_READ: u32 = 0x8297;
pub const SRGB_WRITE: u32 = 0x8298;
pub const STACK_OVERFLOW: u32 = 0x0503;
pub const STACK_UNDERFLOW: u32 = 0x0504;
pub const STATIC_COPY: u32 = 0x88E6;
pub const STATIC_DRAW: u32 = 0x88E4;
pub const STATIC_READ: u32 = 0x88E5;
pub const STENCIL: u32 = 0x1802;
pub const STENCIL_ATTACHMENT: u32 = 0x8D20;
pub const STENCIL_BACK_FAIL: u32 = 0x8801;
pub const STENCIL_BACK_FUNC: u32 = 0x8800;
pub const STENCIL_BACK_PASS_DEPTH_FAIL: u32 = 0x8802;
pub const STENCIL_BACK_PASS_DEPTH_PASS: u32 = 0x8803;
pub const STENCIL_BACK_REF: u32 = 0x8CA3;
pub const STENCIL_BACK_VALUE_MASK: u32 = 0x8CA4;
pub const STENCIL_BACK_WRITEMASK: u32 = 0x8CA5;
pub const STENCIL_BUFFER_BIT: u32 = 0x00000400;
pub const STENCIL_CLEAR_VALUE: u32 = 0x0B91;
pub const STENCIL_COMPONENTS: u32 = 0x8285;
pub const STENCIL_FAIL: u32 = 0x0B94;
pub const STENCIL_FUNC: u32 = 0x0B92;
pub const STENCIL_INDEX: u32 = 0x1901;
pub const STENCIL_INDEX1: u32 = 0x8D46;
pub const STENCIL_INDEX16: u32 = 0x8D49;
pub const STENCIL_INDEX4: u32 = 0x8D47;
pub const STENCIL_INDEX8: u32 = 0x8D48;
pub const STENCIL_PASS_DEPTH_FAIL: u32 = 0x0B95;
pub const STENCIL_PASS_DEPTH_PASS: u32 = 0x0B96;
pub const STENCIL_REF: u32 = 0x0B97;
pub const STENCIL_RENDERABLE: u32 = 0x8288;
pub const STENCIL_TEST: u32 = 0x0B90;
pub const STENCIL_VALUE_MASK: u32 = 0x0B93;
pub const STENCIL_WRITEMASK: u32 = 0x0B98;
pub const STEREO: u32 = 0x0C33;
pub const STREAM_COPY: u32 = 0x88E2;
pub const STREAM_DRAW: u32 = 0x88E0;
pub const STREAM_READ: u32 = 0x88E1;
pub const SUBPIXEL_BITS: u32 = 0x0D50;
pub const SYNC_CONDITION: u32 = 0x9113;
pub const SYNC_FENCE: u32 = 0x9116;
pub const SYNC_FLAGS: u32 = 0x9115;
pub const SYNC_FLUSH_COMMANDS_BIT: u32 = 0x00000001;
pub const SYNC_GPU_COMMANDS_COMPLETE: u32 = 0x9117;
pub const SYNC_STATUS: u32 = 0x9114;
pub const TESS_CONTROL_OUTPUT_VERTICES: u32 = 0x8E75;
pub const TESS_CONTROL_SHADER: u32 = 0x8E88;
pub const TESS_CONTROL_SHADER_BIT: u32 = 0x00000008;
pub const TESS_CONTROL_SHADER_PATCHES: u32 = 0x82F1;
pub const TESS_CONTROL_SUBROUTINE: u32 = 0x92E9;
pub const TESS_CONTROL_SUBROUTINE_UNIFORM: u32 = 0x92EF;
pub const TESS_CONTROL_TEXTURE: u32 = 0x829C;
pub const TESS_EVALUATION_SHADER: u32 = 0x8E87;
pub const TESS_EVALUATION_SHADER_BIT: u32 = 0x00000010;
pub const TESS_EVALUATION_SHADER_INVOCATIONS: u32 = 0x82F2;
pub const TESS_EVALUATION_SUBROUTINE: u32 = 0x92EA;
pub const TESS_EVALUATION_SUBROUTINE_UNIFORM: u32 = 0x92F0;
pub const TESS_EVALUATION_TEXTURE: u32 = 0x829D;
pub const TESS_GEN_MODE: u32 = 0x8E76;
pub const TESS_GEN_POINT_MODE: u32 = 0x8E79;
pub const TESS_GEN_SPACING: u32 = 0x8E77;
pub const TESS_GEN_VERTEX_ORDER: u32 = 0x8E78;
pub const TEXTURE: u32 = 0x1702;
pub const TEXTURE0: u32 = 0x84C0;
pub const TEXTURE1: u32 = 0x84C1;
pub const TEXTURE10: u32 = 0x84CA;
pub const TEXTURE11: u32 = 0x84CB;
pub const TEXTURE12: u32 = 0x84CC;
pub const TEXTURE13: u32 = 0x84CD;
pub const TEXTURE14: u32 = 0x84CE;
pub const TEXTURE15: u32 = 0x84CF;
pub const TEXTURE16: u32 = 0x84D0;
pub const TEXTURE17: u32 = 0x84D1;
pub const TEXTURE18: u32 = 0x84D2;
pub const TEXTURE19: u32 = 0x84D3;
pub const TEXTURE2: u32 = 0x84C2;
pub const TEXTURE20: u32 = 0x84D4;
pub const TEXTURE21: u32 = 0x84D5;
pub const TEXTURE22: u32 = 0x84D6;
pub const TEXTURE23: u32 = 0x84D7;
pub const TEXTURE24: u32 = 0x84D8;
pub const TEXTURE25: u32 = 0x84D9;
pub const TEXTURE26: u32 = 0x84DA;
pub const TEXTURE27: u32 = 0x84DB;
pub const TEXTURE28: u32 = 0x84DC;
pub const TEXTURE29: u32 = 0x84DD;
pub const TEXTURE3: u32 = 0x84C3;
pub const TEXTURE30: u32 = 0x84DE;
pub const TEXTURE31: u32 = 0x84DF;
pub const TEXTURE4: u32 = 0x84C4;
pub const TEXTURE5: u32 = 0x84C5;
pub const TEXTURE6: u32 = 0x84C6;
pub const TEXTURE7: u32 = 0x84C7;
pub const TEXTURE8: u32 = 0x84C8;
pub const TEXTURE9: u32 = 0x84C9;
pub const TEXTURE_1D: u32 = 0x0DE0;
pub const TEXTURE_1D_ARRAY: u32 = 0x8C18;
pub const TEXTURE_2D: u32 = 0x0DE1;
pub const TEXTURE_2D_ARRAY: u32 = 0x8C1A;
pub const TEXTURE_2D_MULTISAMPLE: u32 = 0x9100;
pub const TEXTURE_2D_MULTISAMPLE_ARRAY: u32 = 0x9102;
pub const TEXTURE_3D: u32 = 0x806F;
pub const TEXTURE_ALPHA_SIZE: u32 = 0x805F;
pub const TEXTURE_ALPHA_TYPE: u32 = 0x8C13;
pub const TEXTURE_BASE_LEVEL: u32 = 0x813C;
pub const TEXTURE_BINDING_1D: u32 = 0x8068;
pub const TEXTURE_BINDING_1D_ARRAY: u32 = 0x8C1C;
pub const TEXTURE_BINDING_2D: u32 = 0x8069;
pub const TEXTURE_BINDING_2D_ARRAY: u32 = 0x8C1D;
pub const TEXTURE_BINDING_2D_MULTISAMPLE: u32 = 0x9104;
pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: u32 = 0x9105;
pub const TEXTURE_BINDING_3D: u32 = 0x806A;
pub const TEXTURE_BINDING_BUFFER: u32 = 0x8C2C;
pub const TEXTURE_BINDING_CUBE_MAP: u32 = 0x8514;
pub const TEXTURE_BINDING_CUBE_MAP_ARRAY: u32 = 0x900A;
pub const TEXTURE_BINDING_RECTANGLE: u32 = 0x84F6;
pub const TEXTURE_BLUE_SIZE: u32 = 0x805E;
pub const TEXTURE_BLUE_TYPE: u32 = 0x8C12;
pub const TEXTURE_BORDER_COLOR: u32 = 0x1004;
pub const TEXTURE_BUFFER: u32 = 0x8C2A;
pub const TEXTURE_BUFFER_BINDING: u32 = 0x8C2A;
pub const TEXTURE_BUFFER_DATA_STORE_BINDING: u32 = 0x8C2D;
pub const TEXTURE_BUFFER_OFFSET: u32 = 0x919D;
pub const TEXTURE_BUFFER_OFFSET_ALIGNMENT: u32 = 0x919F;
pub const TEXTURE_BUFFER_SIZE: u32 = 0x919E;
pub const TEXTURE_COMPARE_FUNC: u32 = 0x884D;
pub const TEXTURE_COMPARE_MODE: u32 = 0x884C;
pub const TEXTURE_COMPRESSED: u32 = 0x86A1;
pub const TEXTURE_COMPRESSED_BLOCK_HEIGHT: u32 = 0x82B2;
pub const TEXTURE_COMPRESSED_BLOCK_SIZE: u32 = 0x82B3;
pub const TEXTURE_COMPRESSED_BLOCK_WIDTH: u32 = 0x82B1;
pub const TEXTURE_COMPRESSED_IMAGE_SIZE: u32 = 0x86A0;
pub const TEXTURE_COMPRESSION_HINT: u32 = 0x84EF;
pub const TEXTURE_CUBE_MAP: u32 = 0x8513;
pub const TEXTURE_CUBE_MAP_ARRAY: u32 = 0x9009;
pub const TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 0x8516;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 0x8518;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 0x851A;
pub const TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 0x8515;
pub const TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 0x8517;
pub const TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 0x8519;
pub const TEXTURE_CUBE_MAP_SEAMLESS: u32 = 0x884F;
pub const TEXTURE_DEPTH: u32 = 0x8071;
pub const TEXTURE_DEPTH_SIZE: u32 = 0x884A;
pub const TEXTURE_DEPTH_TYPE: u32 = 0x8C16;
pub const TEXTURE_FETCH_BARRIER_BIT: u32 = 0x00000008;
pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: u32 = 0x9107;
pub const TEXTURE_GATHER: u32 = 0x82A2;
pub const TEXTURE_GATHER_SHADOW: u32 = 0x82A3;
pub const TEXTURE_GREEN_SIZE: u32 = 0x805D;
pub const TEXTURE_GREEN_TYPE: u32 = 0x8C11;
pub const TEXTURE_HEIGHT: u32 = 0x1001;
pub const TEXTURE_IMAGE_FORMAT: u32 = 0x828F;
pub const TEXTURE_IMAGE_TYPE: u32 = 0x8290;
pub const TEXTURE_IMMUTABLE_FORMAT: u32 = 0x912F;
pub const TEXTURE_IMMUTABLE_LEVELS: u32 = 0x82DF;
pub const TEXTURE_INTERNAL_FORMAT: u32 = 0x1003;
pub const TEXTURE_LOD_BIAS: u32 = 0x8501;
pub const TEXTURE_MAG_FILTER: u32 = 0x2800;
pub const TEXTURE_MAX_ANISOTROPY: u32 = 0x84FE;
pub const TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FE;
pub const TEXTURE_MAX_LEVEL: u32 = 0x813D;
pub const TEXTURE_MAX_LOD: u32 = 0x813B;
pub const TEXTURE_MIN_FILTER: u32 = 0x2801;
pub const TEXTURE_MIN_LOD: u32 = 0x813A;
pub const TEXTURE_RECTANGLE: u32 = 0x84F5;
pub const TEXTURE_RED_SIZE: u32 = 0x805C;
pub const TEXTURE_RED_TYPE: u32 = 0x8C10;
pub const TEXTURE_SAMPLES: u32 = 0x9106;
pub const TEXTURE_SHADOW: u32 = 0x82A1;
pub const TEXTURE_SHARED_SIZE: u32 = 0x8C3F;
pub const TEXTURE_STENCIL_SIZE: u32 = 0x88F1;
pub const TEXTURE_SWIZZLE_A: u32 = 0x8E45;
pub const TEXTURE_SWIZZLE_B: u32 = 0x8E44;
pub const TEXTURE_SWIZZLE_G: u32 = 0x8E43;
pub const TEXTURE_SWIZZLE_R: u32 = 0x8E42;
pub const TEXTURE_SWIZZLE_RGBA: u32 = 0x8E46;
pub const TEXTURE_TARGET: u32 = 0x1006;
pub const TEXTURE_UPDATE_BARRIER_BIT: u32 = 0x00000100;
pub const TEXTURE_VIEW: u32 = 0x82B5;
pub const TEXTURE_VIEW_MIN_LAYER: u32 = 0x82DD;
pub const TEXTURE_VIEW_MIN_LEVEL: u32 = 0x82DB;
pub const TEXTURE_VIEW_NUM_LAYERS: u32 = 0x82DE;
pub const TEXTURE_VIEW_NUM_LEVELS: u32 = 0x82DC;
pub const TEXTURE_WIDTH: u32 = 0x1000;
pub const TEXTURE_WRAP_R: u32 = 0x8072;
pub const TEXTURE_WRAP_S: u32 = 0x2802;
pub const TEXTURE_WRAP_T: u32 = 0x2803;
pub const TIMEOUT_EXPIRED: u32 = 0x911B;
pub const TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF;
pub const TIMESTAMP: u32 = 0x8E28;
pub const TIME_ELAPSED: u32 = 0x88BF;
pub const TOP_LEVEL_ARRAY_SIZE: u32 = 0x930C;
pub const TOP_LEVEL_ARRAY_STRIDE: u32 = 0x930D;
pub const TRANSFORM_FEEDBACK: u32 = 0x8E22;
pub const TRANSFORM_FEEDBACK_ACTIVE: u32 = 0x8E24;
pub const TRANSFORM_FEEDBACK_BARRIER_BIT: u32 = 0x00000800;
pub const TRANSFORM_FEEDBACK_BINDING: u32 = 0x8E25;
pub const TRANSFORM_FEEDBACK_BUFFER: u32 = 0x8C8E;
pub const TRANSFORM_FEEDBACK_BUFFER_ACTIVE: u32 = 0x8E24;
pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: u32 = 0x8C8F;
pub const TRANSFORM_FEEDBACK_BUFFER_INDEX: u32 = 0x934B;
pub const TRANSFORM_FEEDBACK_BUFFER_MODE: u32 = 0x8C7F;
pub const TRANSFORM_FEEDBACK_BUFFER_PAUSED: u32 = 0x8E23;
pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: u32 = 0x8C85;
pub const TRANSFORM_FEEDBACK_BUFFER_START: u32 = 0x8C84;
pub const TRANSFORM_FEEDBACK_BUFFER_STRIDE: u32 = 0x934C;
pub const TRANSFORM_FEEDBACK_OVERFLOW: u32 = 0x82EC;
pub const TRANSFORM_FEEDBACK_PAUSED: u32 = 0x8E23;
pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: u32 = 0x8C88;
pub const TRANSFORM_FEEDBACK_STREAM_OVERFLOW: u32 = 0x82ED;
pub const TRANSFORM_FEEDBACK_VARYING: u32 = 0x92F4;
pub const TRANSFORM_FEEDBACK_VARYINGS: u32 = 0x8C83;
pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: u32 = 0x8C76;
pub const TRIANGLES: u32 = 0x0004;
pub const TRIANGLES_ADJACENCY: u32 = 0x000C;
pub const TRIANGLE_FAN: u32 = 0x0006;
pub const TRIANGLE_STRIP: u32 = 0x0005;
pub const TRIANGLE_STRIP_ADJACENCY: u32 = 0x000D;
pub const TRUE: u32 = 1;
pub const TYPE: u32 = 0x92FA;
pub const UNDEFINED_VERTEX: u32 = 0x8260;
pub const UNIFORM: u32 = 0x92E1;
pub const UNIFORM_ARRAY_STRIDE: u32 = 0x8A3C;
pub const UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: u32 = 0x92DA;
pub const UNIFORM_BARRIER_BIT: u32 = 0x00000004;
pub const UNIFORM_BLOCK: u32 = 0x92E2;
pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: u32 = 0x8A42;
pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: u32 = 0x8A43;
pub const UNIFORM_BLOCK_BINDING: u32 = 0x8A3F;
pub const UNIFORM_BLOCK_DATA_SIZE: u32 = 0x8A40;
pub const UNIFORM_BLOCK_INDEX: u32 = 0x8A3A;
pub const UNIFORM_BLOCK_NAME_LENGTH: u32 = 0x8A41;
pub const UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: u32 = 0x90EC;
pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: u32 = 0x8A46;
pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: u32 = 0x8A45;
pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: u32 = 0x84F0;
pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: u32 = 0x84F1;
pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: u32 = 0x8A44;
pub const UNIFORM_BUFFER: u32 = 0x8A11;
pub const UNIFORM_BUFFER_BINDING: u32 = 0x8A28;
pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: u32 = 0x8A34;
pub const UNIFORM_BUFFER_SIZE: u32 = 0x8A2A;
pub const UNIFORM_BUFFER_START: u32 = 0x8A29;
pub const UNIFORM_IS_ROW_MAJOR: u32 = 0x8A3E;
pub const UNIFORM_MATRIX_STRIDE: u32 = 0x8A3D;
pub const UNIFORM_NAME_LENGTH: u32 = 0x8A39;
pub const UNIFORM_OFFSET: u32 = 0x8A3B;
pub const UNIFORM_SIZE: u32 = 0x8A38;
pub const UNIFORM_TYPE: u32 = 0x8A37;
pub const UNKNOWN_CONTEXT_RESET: u32 = 0x8255;
pub const UNPACK_ALIGNMENT: u32 = 0x0CF5;
pub const UNPACK_COMPRESSED_BLOCK_DEPTH: u32 = 0x9129;
pub const UNPACK_COMPRESSED_BLOCK_HEIGHT: u32 = 0x9128;
pub const UNPACK_COMPRESSED_BLOCK_SIZE: u32 = 0x912A;
pub const UNPACK_COMPRESSED_BLOCK_WIDTH: u32 = 0x9127;
pub const UNPACK_IMAGE_HEIGHT: u32 = 0x806E;
pub const UNPACK_LSB_FIRST: u32 = 0x0CF1;
pub const UNPACK_ROW_LENGTH: u32 = 0x0CF2;
pub const UNPACK_SKIP_IMAGES: u32 = 0x806D;
pub const UNPACK_SKIP_PIXELS: u32 = 0x0CF4;
pub const UNPACK_SKIP_ROWS: u32 = 0x0CF3;
pub const UNPACK_SWAP_BYTES: u32 = 0x0CF0;
pub const UNSIGNALED: u32 = 0x9118;
pub const UNSIGNED_BYTE: u32 = 0x1401;
pub const UNSIGNED_BYTE_2_3_3_REV: u32 = 0x8362;
pub const UNSIGNED_BYTE_3_3_2: u32 = 0x8032;
pub const UNSIGNED_INT: u32 = 0x1405;
pub const UNSIGNED_INT64_ARB: u32 = 0x140F;
pub const UNSIGNED_INT_10F_11F_11F_REV: u32 = 0x8C3B;
pub const UNSIGNED_INT_10_10_10_2: u32 = 0x8036;
pub const UNSIGNED_INT_24_8: u32 = 0x84FA;
pub const UNSIGNED_INT_2_10_10_10_REV: u32 = 0x8368;
pub const UNSIGNED_INT_5_9_9_9_REV: u32 = 0x8C3E;
pub const UNSIGNED_INT_8_8_8_8: u32 = 0x8035;
pub const UNSIGNED_INT_8_8_8_8_REV: u32 = 0x8367;
pub const UNSIGNED_INT_ATOMIC_COUNTER: u32 = 0x92DB;
pub const UNSIGNED_INT_IMAGE_1D: u32 = 0x9062;
pub const UNSIGNED_INT_IMAGE_1D_ARRAY: u32 = 0x9068;
pub const UNSIGNED_INT_IMAGE_2D: u32 = 0x9063;
pub const UNSIGNED_INT_IMAGE_2D_ARRAY: u32 = 0x9069;
pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: u32 = 0x906B;
pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: u32 = 0x906C;
pub const UNSIGNED_INT_IMAGE_2D_RECT: u32 = 0x9065;
pub const UNSIGNED_INT_IMAGE_3D: u32 = 0x9064;
pub const UNSIGNED_INT_IMAGE_BUFFER: u32 = 0x9067;
pub const UNSIGNED_INT_IMAGE_CUBE: u32 = 0x9066;
pub const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: u32 = 0x906A;
pub const UNSIGNED_INT_SAMPLER_1D: u32 = 0x8DD1;
pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: u32 = 0x8DD6;
pub const UNSIGNED_INT_SAMPLER_2D: u32 = 0x8DD2;
pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: u32 = 0x8DD7;
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: u32 = 0x910A;
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: u32 = 0x910D;
pub const UNSIGNED_INT_SAMPLER_2D_RECT: u32 = 0x8DD5;
pub const UNSIGNED_INT_SAMPLER_3D: u32 = 0x8DD3;
pub const UNSIGNED_INT_SAMPLER_BUFFER: u32 = 0x8DD8;
pub const UNSIGNED_INT_SAMPLER_CUBE: u32 = 0x8DD4;
pub const UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: u32 = 0x900F;
pub const UNSIGNED_INT_VEC2: u32 = 0x8DC6;
pub const UNSIGNED_INT_VEC3: u32 = 0x8DC7;
pub const UNSIGNED_INT_VEC4: u32 = 0x8DC8;
pub const UNSIGNED_NORMALIZED: u32 = 0x8C17;
pub const UNSIGNED_SHORT: u32 = 0x1403;
pub const UNSIGNED_SHORT_1_5_5_5_REV: u32 = 0x8366;
pub const UNSIGNED_SHORT_4_4_4_4: u32 = 0x8033;
pub const UNSIGNED_SHORT_4_4_4_4_REV: u32 = 0x8365;
pub const UNSIGNED_SHORT_5_5_5_1: u32 = 0x8034;
pub const UNSIGNED_SHORT_5_6_5: u32 = 0x8363;
pub const UNSIGNED_SHORT_5_6_5_REV: u32 = 0x8364;
pub const UPPER_LEFT: u32 = 0x8CA2;
pub const VALIDATE_STATUS: u32 = 0x8B83;
pub const VENDOR: u32 = 0x1F00;
pub const VERSION: u32 = 0x1F02;
pub const VERTEX_ARRAY: u32 = 0x8074;
pub const VERTEX_ARRAY_BINDING: u32 = 0x85B5;
pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT: u32 = 0x00000001;
pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: u32 = 0x889F;
pub const VERTEX_ATTRIB_ARRAY_DIVISOR: u32 = 0x88FE;
pub const VERTEX_ATTRIB_ARRAY_DIVISOR_ARB: u32 = 0x88FE;
pub const VERTEX_ATTRIB_ARRAY_ENABLED: u32 = 0x8622;
pub const VERTEX_ATTRIB_ARRAY_INTEGER: u32 = 0x88FD;
pub const VERTEX_ATTRIB_ARRAY_LONG: u32 = 0x874E;
pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: u32 = 0x886A;
pub const VERTEX_ATTRIB_ARRAY_POINTER: u32 = 0x8645;
pub const VERTEX_ATTRIB_ARRAY_SIZE: u32 = 0x8623;
pub const VERTEX_ATTRIB_ARRAY_STRIDE: u32 = 0x8624;
pub const VERTEX_ATTRIB_ARRAY_TYPE: u32 = 0x8625;
pub const VERTEX_ATTRIB_BINDING: u32 = 0x82D4;
pub const VERTEX_ATTRIB_RELATIVE_OFFSET: u32 = 0x82D5;
pub const VERTEX_BINDING_BUFFER: u32 = 0x8F4F;
pub const VERTEX_BINDING_DIVISOR: u32 = 0x82D6;
pub const VERTEX_BINDING_OFFSET: u32 = 0x82D7;
pub const VERTEX_BINDING_STRIDE: u32 = 0x82D8;
pub const VERTEX_PROGRAM_POINT_SIZE: u32 = 0x8642;
pub const VERTEX_SHADER: u32 = 0x8B31;
pub const VERTEX_SHADER_BIT: u32 = 0x00000001;
pub const VERTEX_SHADER_INVOCATIONS: u32 = 0x82F0;
pub const VERTEX_SUBROUTINE: u32 = 0x92E8;
pub const VERTEX_SUBROUTINE_UNIFORM: u32 = 0x92EE;
pub const VERTEX_TEXTURE: u32 = 0x829B;
pub const VERTICES_SUBMITTED: u32 = 0x82EE;
pub const VIEWPORT: u32 = 0x0BA2;
pub const VIEWPORT_BOUNDS_RANGE: u32 = 0x825D;
pub const VIEWPORT_INDEX_PROVOKING_VERTEX: u32 = 0x825F;
pub const VIEWPORT_SUBPIXEL_BITS: u32 = 0x825C;
pub const VIEW_CLASS_128_BITS: u32 = 0x82C4;
pub const VIEW_CLASS_16_BITS: u32 = 0x82CA;
pub const VIEW_CLASS_24_BITS: u32 = 0x82C9;
pub const VIEW_CLASS_32_BITS: u32 = 0x82C8;
pub const VIEW_CLASS_48_BITS: u32 = 0x82C7;
pub const VIEW_CLASS_64_BITS: u32 = 0x82C6;
pub const VIEW_CLASS_8_BITS: u32 = 0x82CB;
pub const VIEW_CLASS_96_BITS: u32 = 0x82C5;
pub const VIEW_CLASS_BPTC_FLOAT: u32 = 0x82D3;
pub const VIEW_CLASS_BPTC_UNORM: u32 = 0x82D2;
pub const VIEW_CLASS_RGTC1_RED: u32 = 0x82D0;
pub const VIEW_CLASS_RGTC2_RG: u32 = 0x82D1;
pub const VIEW_CLASS_S3TC_DXT1_RGB: u32 = 0x82CC;
pub const VIEW_CLASS_S3TC_DXT1_RGBA: u32 = 0x82CD;
pub const VIEW_CLASS_S3TC_DXT3_RGBA: u32 = 0x82CE;
pub const VIEW_CLASS_S3TC_DXT5_RGBA: u32 = 0x82CF;
pub const VIEW_COMPATIBILITY_CLASS: u32 = 0x82B6;
pub const WAIT_FAILED: u32 = 0x911D;
pub const WRITE_ONLY: u32 = 0x88B9;
pub const XOR: u32 = 0x1506;
pub const ZERO: u32 = 0;
pub const ZERO_TO_ONE: u32 = 0x935F;
