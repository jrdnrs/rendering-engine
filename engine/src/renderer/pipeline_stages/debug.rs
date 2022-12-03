use std::collections::{HashMap, HashSet};

use glow::{self as gl, HasContext};

use super::PipelineStage;
use crate::{
    components::Renderable,
    math::Mat4f,
    memory_manager::memory_manager::{
        DrawElementsIndirectCommand, InstanceData, MemoryManager, DRAW_COMMAND_SIZE,
    },
    renderer::{
        command::DrawCommands,
        state::{RasteriserState, RendererState},
    },
    resource_manager::resource_manager::{
        FramebufferID, MaterialID, MeshID, ResourceIDTrait, ResourcesManager, ShaderProgramID,
    },
};

pub struct DebugStage<'a> {
    gl: &'a gl::Context,
    target: FramebufferID,
    wireframe_shader_id: ShaderProgramID,
    vertices_shader_id: ShaderProgramID,
    renderable_indices: Vec<usize>,
    command_queue: DrawCommands,
    pending_indirect_command_count: u32,
}

impl<'a> DebugStage<'a> {
    pub fn new(
        gl: &'a gl::Context,
        target: FramebufferID,
        resources_manager: &mut ResourcesManager,
    ) -> Self {
        let wireframe_shader_id = resources_manager.load_shader("res/shaders/debug_wireframe.glsl");
        let vertices_shader_id = resources_manager.load_shader("res/shaders/debug_vertices.glsl");

        Self {
            gl,
            target,
            renderable_indices: Vec::new(),
            command_queue: DrawCommands::new(hash),
            pending_indirect_command_count: 0,
            wireframe_shader_id,
            vertices_shader_id,
        }
    }
}

impl<'a> PipelineStage for DebugStage<'a> {
    fn get_target(&self) -> FramebufferID {
        self.target
    }

    fn init(
        &mut self,
        memory_manager: &mut MemoryManager,
        resource_manager: &mut ResourcesManager,
        renderer_state: &mut RendererState,
    ) {
    }

    fn submit(&mut self, renderable_index: usize) {
        self.renderable_indices.push(renderable_index);
    }

    fn execute(
        &mut self,
        memory_manager: &mut MemoryManager,
        resources_manager: &mut ResourcesManager,
        renderer_state: &mut RendererState,
        renderables: &[Renderable]
    ) {
        self.command_queue.update_keys(renderables, &self.renderable_indices);
        self.command_queue.sort_indices();

        // HACK: added a dummy renderable to the end to otherwise the last renderable would be cut off
        // when iterating through renderables with renderable and next_renderable. zip stops when one returns None
        // self.renderables.push(Renderable {
        //     mesh_id: MeshID::new(0xFFFF),
        //     material_id: MaterialID::new(0xFFFF),
        //     shader_id: ShaderProgramID::new(0xFFFF),
        //     transform: Mat4f::identity(),
        //     pipeline_stages: 0,
        // });
        // self.command_queue.indices.push(self.renderables.len() - 1);

        let mut instance_count = 0;

        for (i, (index, next_index)) in self
            .command_queue
            .indices
            .iter()
            .zip(self.command_queue.indices[1..].iter())
            .enumerate()
        {
            let renderable = &renderables[self.renderable_indices[*index]];
            let next_renderable = &renderables[self.renderable_indices[*next_index]];

            instance_count += 1;

            if renderable.mesh_id != next_renderable.mesh_id {
                memory_manager.reserve_instance_space(instance_count);
                upload_draw_data(
                    memory_manager,
                    resources_manager,
                    renderable,
                    instance_count,
                );
                self.pending_indirect_command_count += 1;

                for instance_index in
                    self.command_queue.indices[(i - (instance_count - 1) as usize)..(i + 1)].iter()
                {
                    let renderable = &renderables[self.renderable_indices[*instance_index]];

                    memory_manager.push_instance_data(&InstanceData {
                        material_index: renderable.material_id.index(),
                        transform: renderable.transform,
                    });
                }

                instance_count = 0;
            }
        }
        
        unsafe {
            self.gl.polygon_mode(gl::FRONT_AND_BACK, gl::LINE);
        }
        renderer_state.set_shader_program(self.wireframe_shader_id, resources_manager);
        make_draw_call(
            self.gl,
            memory_manager,
            self.pending_indirect_command_count,
            gl::TRIANGLES,
        );

        unsafe {
            self.gl.polygon_mode(gl::FRONT_AND_BACK, gl::FILL);
        }

        renderer_state.set_shader_program(self.vertices_shader_id, resources_manager);
        make_draw_call(
            self.gl,
            memory_manager,
            self.pending_indirect_command_count,
            gl::POINTS,
        );

        self.pending_indirect_command_count = 0;
        self.renderable_indices.clear();
    }
}

fn hash(r: &Renderable) -> u32 {
    r.mesh_id.index()
}

fn upload_draw_data(
    memory_manager: &mut MemoryManager,
    resources_manager: &mut ResourcesManager,
    renderable: &Renderable,
    instance_count: u32,
) {
    let mesh = resources_manager.borrow_mesh(&renderable.mesh_id).unwrap();

    memory_manager.reserve_vertex_space(mesh.vertices.len() as u32);
    memory_manager.reserve_index_space(mesh.indices.len() as u32);
    let indirect_command = DrawElementsIndirectCommand {
        count: mesh.indices.len() as u32,
        instance_count,
        first_index: memory_manager.get_index_index(),
        base_vertex: memory_manager.get_vertex_index(),
        base_instance: memory_manager.get_instance_index(),
    };
    memory_manager.push_indirect_command(indirect_command);
    memory_manager.push_vertex_slice(&mesh.vertices);
    memory_manager.push_index_slice(&mesh.indices);
}

fn make_draw_call(
    gl: &gl::Context,
    memory_manager: &mut MemoryManager,
    command_count: u32,
    mode: u32,
) {
    unsafe {
        gl.multi_draw_elements_indirect_offset(
            mode,
            gl::UNSIGNED_INT,
            ((memory_manager.get_indirect_command_index() - command_count) * DRAW_COMMAND_SIZE)
                as i32,
            command_count as i32,
            0,
        );
    }
}
