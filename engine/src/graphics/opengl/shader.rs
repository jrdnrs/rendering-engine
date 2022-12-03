use std::collections::HashMap;

use colored::*;
use log::{debug, error, trace};

use crate::{
    graphics::{
        graphics::ApiEnum,
        shader::{Program, ShaderData, ShaderDataType},
    },
    platform::rustgl as gl,
};

pub fn shader_data_element_count(type_: &ShaderDataType) -> usize {
    match type_ {
        ShaderDataType::Uint1 => 1,
        ShaderDataType::Uint2 => 2,
        ShaderDataType::Uint3 => 3,
        ShaderDataType::Uint4 => 4,
        ShaderDataType::Int1 => 1,
        ShaderDataType::Int2 => 2,
        ShaderDataType::Int3 => 3,
        ShaderDataType::Int4 => 4,
        ShaderDataType::Float1 => 1,
        ShaderDataType::Float2 => 2,
        ShaderDataType::Float3 => 3,
        ShaderDataType::Float4 => 4,
        ShaderDataType::Mat2f => 4,
        ShaderDataType::Mat3f => 9,
        ShaderDataType::Mat4f => 16,
    }
}

pub fn shader_data_size_bytes(type_: &ShaderDataType) -> usize {
    match type_ {
        ShaderDataType::Uint1 => 1 * 4,
        ShaderDataType::Uint2 => 2 * 4,
        ShaderDataType::Uint3 => 3 * 4,
        ShaderDataType::Uint4 => 4 * 4,
        ShaderDataType::Int1 => 1 * 4,
        ShaderDataType::Int2 => 2 * 4,
        ShaderDataType::Int3 => 3 * 4,
        ShaderDataType::Int4 => 4 * 4,
        ShaderDataType::Float1 => 1 * 4,
        ShaderDataType::Float2 => 2 * 4,
        ShaderDataType::Float3 => 3 * 4,
        ShaderDataType::Float4 => 4 * 4,
        ShaderDataType::Mat2f => 4 * 4,
        ShaderDataType::Mat3f => 9 * 4,
        ShaderDataType::Mat4f => 16 * 4,
    }
}

pub fn shader_data_gl_type(type_: &ShaderDataType) -> ApiEnum {
    match type_ {
        ShaderDataType::Uint1
        | ShaderDataType::Uint2
        | ShaderDataType::Uint3
        | ShaderDataType::Uint4 => gl::UNSIGNED_INT,
        ShaderDataType::Int1
        | ShaderDataType::Int2
        | ShaderDataType::Int3
        | ShaderDataType::Int4 => gl::INT,
        ShaderDataType::Float1
        | ShaderDataType::Float2
        | ShaderDataType::Float3
        | ShaderDataType::Float4
        | ShaderDataType::Mat2f
        | ShaderDataType::Mat3f
        | ShaderDataType::Mat4f => gl::FLOAT,
    }
}

pub struct ShaderBuilder {
    entrypoint: &'static str,
    includes: Vec<String>,
    read_string: String,
    write_string: String,
}

impl ShaderBuilder {
    pub fn build(entrypoint: &'static str) -> Result<Vec<(u32, String)>, String> {
        let shaders_source = match std::fs::read_to_string(entrypoint) {
            Ok(ss) => ss,
            Err(error) => return Err(error.to_string()),
        };

        let mut sb = ShaderBuilder {
            entrypoint,
            includes: Vec::new(),
            read_string: String::new(),
            write_string: String::new(),
        };
        let mut shaders_final = Vec::new();

        for shader_source in shaders_source.split("#shader ") {
            let (shader_type, shader_source_no_type) = match shader_source.split_once("\r") {
                Some(("vertex", s)) => (gl::VERTEX_SHADER, s),
                Some(("fragment", s)) => (gl::FRAGMENT_SHADER, s),
                Some(("geometry", s)) => (gl::GEOMETRY_SHADER, s),
                Some(("compute", s)) => (gl::COMPUTE_SHADER, s),
                Some((st, _)) => {
                    return Err(format!("Unknown Shader Type Declared '{st}'")
                        .red()
                        .to_string());
                }
                _ => continue,
            };

            shaders_final.push((shader_type, sb.resolve_includes(shader_source_no_type)));
        }

        Ok(shaders_final)
    }

    fn resolve_includes(&mut self, shader_source: &str) -> String {
        self.includes.clear();

        self.read_string = shader_source.to_owned();
        self.write_string.clear();

        let mut finished = false;

        while !finished {
            let mut iter = self.read_string.lines().peekable();
            while let Some(line) = iter.next() {
                if line.trim().starts_with("#include ") {
                    let path = line.split('"').nth(1).unwrap();
                    let include_directive = format!(r#"#include "{path}""#);

                    // if we've seen this path before, skip
                    if self.includes.iter().any(|s| s == path) {
                        self.write_string = self.read_string.replace(&include_directive, "");
                        break;
                    } else {
                        self.includes.push(path.to_owned())
                    }

                    // replace directive with the file if found
                    if let Ok(included_source) = std::fs::read_to_string(path) {
                        self.write_string = self
                            .read_string
                            .replace(&include_directive, &included_source);
                        break;
                    } else {
                        error!(
                            "Unable to include '{}' in shader '{}'",
                            path, self.entrypoint
                        );
                        self.write_string = self.read_string.replace(&include_directive, "");
                    }
                }

                if iter.peek().is_none() {
                    finished = true;
                }
            }

            // swap the iterated string with the modified one
            std::mem::swap(&mut self.read_string, &mut self.write_string);
        }

        self.write_string.clone()
    }
}

impl Program {
    pub fn new(&self) -> Program {
        let gl_prog = unsafe { gl::create_program().unwrap() };

        Program {
            handle: gl_prog.0,
            shader_handles: Vec::new(),
            uniform_loc_cache: HashMap::new(),
            shaders_path: "",
        }
    }

    pub fn reload_shaders(&self, program: &mut Program) {
        program.shader_handles.clear();
        program.uniform_loc_cache.clear();
        unsafe { gl::delete_program(gl::GlProgram(program.handle)) };

        let gl_prog = unsafe { gl::create_program().unwrap() };
        program.handle = gl_prog.0;
        self.add_shaders(program, program.shaders_path);
    }

    /// Accepts a single string containing multiple shader definitions, each prefaced by <br>
    /// #shader *<shader_type>* <br>
    /// where *<shader_type>* can be vertex, fragment, and so on.
    pub fn add_shaders(&self, program: &mut Program, shaders_path: &'static str) {
        program.shaders_path = shaders_path;

        match ShaderBuilder::build(shaders_path) {
            Ok(shaders) => {
                for (shader_type, shader_source) in shaders.iter() {
                    self.add_shader(program, *shader_type, shader_source);
                }
            }
            Err(error) => error!("{}", error),
        }

        for shader in program.shader_handles.iter() {
            unsafe { gl::detach_shader(gl::GlProgram(program.handle), gl::GlShader(*shader)) }
        }
    }

    /// Attempts to compile and link the shader to this program
    fn add_shader(&self, program: &mut Program, shader_type: u32, shader_source: &str) {
        unsafe {
            // This 'NativeShader' type is a u32 that represents the pointer to our new shader object
            let shader = gl::create_shader(shader_type).unwrap();

            // We associate the shader object with a source code string
            gl::shader_source(shader, shader_source);

            trace!(
                "{}",
                format!(
                    "Adding {} Shader from '{}'",
                    if shader_type == gl::VERTEX_SHADER {
                        "Vertex"
                    } else if shader_type == gl::FRAGMENT_SHADER {
                        "Fragment"
                    } else if shader_type == gl::GEOMETRY_SHADER {
                        "Geometry"
                    } else if shader_type == gl::COMPUTE_SHADER {
                        "Compute"
                    } else {
                        "Unknown" // TODO: Add more types
                    },
                    program.shaders_path,
                )
                .blue()
            );
            self.compile_shader(shader);
            self.link_shader(program, shader);

            program.shader_handles.push(shader.0);
        }
    }

    fn compile_shader(&self, shader: gl::GlShader) {
        unsafe {
            // Compiles the source code strings that have been stored in the shader object
            gl::compile_shader(shader);
        }

        self.print_shader_compile_status(shader);
        self.print_shader_info_log(shader);
    }

    fn link_shader(&self, program: &Program, shader: gl::GlShader) {
        unsafe {
            // We associate the shader object with a source code string
            gl::attach_shader(gl::GlProgram(program.handle), shader);

            // This uses the attached shader objects to create a single executable to run on the GPU
            gl::link_program(gl::GlProgram(program.handle));

            // If a shader object to be deleted is attached to a program object, it will be flagged for deletion, but
            // it will not be deleted until it is no longer attached to any program object
            gl::delete_shader(shader);
        }

        self.print_program_link_status(program);
        self.print_program_info_log(program);
    }

    /// Prints the information log for the specified shader object. <br>
    /// The information log for a shader object is modified when the shader is compiled.
    fn print_shader_info_log(&self, shader: gl::GlShader) {
        let msg = unsafe { gl::get_shader_info_log(shader) };
        let msg = msg.trim();

        debug!(
            "{}{}{}",
            "Program Info Log:".cyan(),
            if msg.is_empty() { "" } else { "/n" },
            msg
        );
    }

    fn print_shader_compile_status(&self, shader: gl::GlShader) {
        debug!("{}{}", "Shader Compile Status: ".cyan(), unsafe {
            gl::get_shader_compile_status(shader)
        })
    }

    fn print_program_info_log(&self, program: &Program) {
        let msg = unsafe { gl::get_program_info_log(gl::GlProgram(program.handle)) };
        let msg = msg.trim();

        debug!(
            "{}{}{}",
            "Program Info Log:".cyan(),
            if msg.is_empty() { "" } else { "/n" },
            msg
        );
    }

    fn print_program_link_status(&self, program: &Program) {
        debug!("{}{}", "Program Link Status: ".cyan(), unsafe {
            gl::get_program_link_status(gl::GlProgram(program.handle))
        });
    }

    pub fn bind(&self, program: &Program) {
        unsafe {
            gl::use_program(Some(gl::GlProgram(program.handle)));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::use_program(None);
        }
    }

    pub fn delete(&self, program: &Program) {
        self.unbind();
        unsafe {
            gl::delete_program(gl::GlProgram(program.handle));
        }
    }

    pub fn set_uniform(&self, program: &mut Program, name: String, type_: ShaderData) {
        let loc = if let Some(location) = program.uniform_loc_cache.get(&name) {
            gl::GlUniformLocation(*location)
        } else if let Some(location) =
            unsafe { gl::get_uniform_location(gl::GlProgram(program.handle), &name) }
        {
            program.uniform_loc_cache.insert(name, location.0);
            location
        } else {
            // error!(
            //     "Attempted to set uniform '{}', but it cannot be found in the current program",
            //     name
            // );
            return;
        };

        match type_ {
            ShaderData::Uint1(x) => unsafe { gl::uniform_1_u32(Some(&loc), x) },
            ShaderData::Uint2(x, y) => unsafe { gl::uniform_2_u32(Some(&loc), x, y) },
            ShaderData::Uint3(x, y, z) => unsafe { gl::uniform_3_u32(Some(&loc), x, y, z) },
            ShaderData::Uint4(x, y, z, w) => unsafe { gl::uniform_4_u32(Some(&loc), x, y, z, w) },
            ShaderData::Int1(x) => unsafe { gl::uniform_1_i32(Some(&loc), x) },
            ShaderData::Int2(x, y) => unsafe { gl::uniform_2_i32(Some(&loc), x, y) },
            ShaderData::Int3(x, y, z) => unsafe { gl::uniform_3_i32(Some(&loc), x, y, z) },
            ShaderData::Int4(x, y, z, w) => unsafe { gl::uniform_4_i32(Some(&loc), x, y, z, w) },
            ShaderData::Float1(x) => unsafe { gl::uniform_1_f32(Some(&loc), x) },
            ShaderData::Float2(x, y) => unsafe { gl::uniform_2_f32(Some(&loc), x, y) },
            ShaderData::Float3(x, y, z) => unsafe { gl::uniform_3_f32(Some(&loc), x, y, z) },
            ShaderData::Float4(x, y, z, w) => unsafe { gl::uniform_4_f32(Some(&loc), x, y, z, w) },
            ShaderData::Mat2f() => todo!(),
            ShaderData::Mat3f() => todo!(),
            ShaderData::Mat4f(v) => unsafe {
                gl::uniform_matrix_4_f32_slice(Some(&loc), false, v.as_slice())
            },
        }
    }
}
