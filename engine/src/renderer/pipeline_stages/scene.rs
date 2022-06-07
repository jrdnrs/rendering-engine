use glow::{self as gl, HasContext};

use super::PipelineStage;
use crate::{
    components::Renderable,
    math::math::Mat4f,
    memory_manager::{
        memory_manager::{DrawElementsIndirectCommand, MemoryManager, DRAW_COMMAND_SIZE},
        uniform_layouts::InstanceData,
    },
    renderer::{
        command::DrawCommands,
        state::{RasteriserState, RendererState},
    },
    resource_manager::resource_manager::{
        FramebufferID, MaterialID, MeshID, ResourceIDTrait, ResourcesManager, ShaderProgramID,
    },
};

pub struct SceneStage<'a> {
    gl: &'a gl::Context,
    target: FramebufferID,
    renderables: Vec<Renderable>,
    command_queue: DrawCommands,
    total_instance_count: u32,
    pending_indirect_command_count: u32,
}

impl<'a> SceneStage<'a> {
    pub fn new(gl: &'a gl::Context, target: FramebufferID) -> Self {
        Self {
            gl,
            target,
            renderables: Vec::new(),
            command_queue: DrawCommands::new(hash),
            total_instance_count: 0,
            pending_indirect_command_count: 0,
        }
    }
}

impl<'a> PipelineStage for SceneStage<'a> {
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

    fn submit(&mut self, renderable: &Renderable) {
        self.renderables.push(renderable.clone())
    }

    fn execute(
        &mut self,
        memory_manager: &mut MemoryManager,
        resources_manager: &mut ResourcesManager,
        renderer_state: &mut RendererState,
    ) {
        unsafe { self.gl.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }

        renderer_state.upload_camera_data(memory_manager);
        renderer_state.upload_light_data(memory_manager);

        self.command_queue.update_keys(&self.renderables);
        self.command_queue.sort_indices();

        let mut previous_renderable = Renderable {
            mesh_id: MeshID::new(0xFFFF),
            material_id: MaterialID::new(0xFFFF),
            shader_id: ShaderProgramID::new(0xFFFF),
            transform: Mat4f::identity(),
            pipeline_stages: 0,
        };
        let mut current_instance_count = 0;
        for (i, index) in self.command_queue.indices.iter().enumerate() {
            let renderable = &self.renderables[*index];

            // if mesh has changed, check if there are queued instances
            // if so, then previous mesh needs to be uploaded and submitted as draw command
            if renderable.mesh_id != previous_renderable.mesh_id && current_instance_count > 0 {
                upload_draw_data(
                    memory_manager,
                    resources_manager,
                    &previous_renderable,
                    current_instance_count,
                    self.total_instance_count,
                );
                self.pending_indirect_command_count += 1;

                self.total_instance_count += current_instance_count;
                current_instance_count = 0;
            }

            // if shader has changed, check if there are queued commands
            // if so, then submit those before changing shader
            if renderable.shader_id != previous_renderable.shader_id {
                // even if mesh is still the same, a new shader also means a new command
                if current_instance_count > 0 {
                    upload_draw_data(
                        memory_manager,
                        resources_manager,
                        &previous_renderable,
                        current_instance_count,
                        self.total_instance_count,
                    );
                    self.pending_indirect_command_count += 1;

                    self.total_instance_count += current_instance_count;
                    current_instance_count = 0;
                }

                if self.pending_indirect_command_count > 0 {
                    make_draw_call(self.gl, memory_manager, self.pending_indirect_command_count);
                    self.pending_indirect_command_count = 0;
                }

                renderer_state.set_shader_program(renderable.shader_id, resources_manager);
            }

            // upload per instance data
            let instance = InstanceData {
                transform: renderable.transform.transpose(),
                material_index: renderable.material_id.index(),
                ..Default::default()
            };
            memory_manager.set_instance_data(instance, i);
            current_instance_count += 1;

            previous_renderable = renderable.clone();
        }

        // flush everything remaining after loop has ended
        if current_instance_count > 0 {
            let renderable =
                &self.renderables[self.command_queue.indices[self.command_queue.indices.len() - 1]];

            upload_draw_data(
                memory_manager,
                resources_manager,
                renderable,
                current_instance_count,
                self.total_instance_count,
            );
            self.pending_indirect_command_count += 1;

            make_draw_call(self.gl, memory_manager, self.pending_indirect_command_count);
        }

        self.renderables.clear();
        self.total_instance_count = 0;
        self.pending_indirect_command_count = 0;
    }
}

fn hash(r: &Renderable) -> u32 {
    let shader_index = r.shader_id.index();
    let mesh_index = r.mesh_id.index();

    (shader_index << 16) | (mesh_index & 0x00FF)
}

fn upload_draw_data(
    memory_manager: &mut MemoryManager,
    resource_manager: &mut ResourcesManager,
    renderable: &Renderable,
    instance_count: u32,
    base_instance: u32,
) {
    let mesh = resource_manager.borrow_mesh(&renderable.mesh_id).unwrap();

    memory_manager.reserve_vertex_space(mesh.vertices.len() as u32);
    memory_manager.reserve_index_space(mesh.indices.len() as u32);
    let indirect_command = DrawElementsIndirectCommand {
        count: mesh.indices.len() as u32,
        instance_count,
        first_index: memory_manager.get_index_index(),
        base_vertex: memory_manager.get_vertex_index(),
        base_instance,
    };
    memory_manager.push_indirect_command(indirect_command);
    memory_manager.push_vertex_slice(&mesh.vertices);
    memory_manager.push_index_slice(&mesh.indices);
}

fn make_draw_call(gl: &gl::Context, memory_manager: &mut MemoryManager, command_count: u32) {
    unsafe {
        gl.multi_draw_elements_indirect_offset(
            gl::TRIANGLES,
            gl::UNSIGNED_INT,
            ((memory_manager.get_indirect_command_index() 
                - command_count ) * DRAW_COMMAND_SIZE) as i32,
            command_count as i32,
            0,
        );
    }
}
