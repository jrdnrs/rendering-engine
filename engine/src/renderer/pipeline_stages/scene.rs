use std::collections::{HashMap, HashSet};

use glow::{self as gl, HasContext};
use nohash_hasher::BuildNoHashHasher;

use super::PipelineStage;
use crate::{
    components::Renderable,
    math::math::Mat4f,
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

pub struct SceneStage<'a> {
    gl: &'a gl::Context,
    target: FramebufferID,
    renderables: Vec<Renderable>,
    command_queue: DrawCommands,
    pending_indirect_command_count: u32,

    meshes_per_shader: HashMap<u32, HashSet<u32, BuildNoHashHasher<u32>>, BuildNoHashHasher<u32>>,
    instances_per_mesh: HashMap<u32, u32, BuildNoHashHasher<u32>>,
}

impl<'a> SceneStage<'a> {
    pub fn new(gl: &'a gl::Context, target: FramebufferID) -> Self {
        Self {
            gl,
            target,
            renderables: Vec::new(),
            command_queue: DrawCommands::new(hash),
            pending_indirect_command_count: 0,

            meshes_per_shader: HashMap::with_hasher(BuildNoHashHasher::default()),
            instances_per_mesh: HashMap::with_hasher(BuildNoHashHasher::default()),
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
        self.renderables.push(renderable.clone());

        if let Some(mesh_set) = self
            .meshes_per_shader
            .get_mut(&renderable.shader_id.index())
        {
            mesh_set.insert(renderable.mesh_id.index());
        } else {
            self.meshes_per_shader
                .insert(renderable.shader_id.index(), HashSet::with_hasher(BuildNoHashHasher::default()));
        }

        let renderable_hash = hash(renderable);
        if let Some(count) = self.instances_per_mesh.get_mut(&renderable_hash) {
            *count += 1;
        } else {
            self.instances_per_mesh.insert(renderable_hash, 1);
        }
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

        let mut index_index = 0;

        while index_index < self.command_queue.indices.len() {
            let renderable_index = self.command_queue.indices[index_index];
            let renderable = &self.renderables[renderable_index];

            renderer_state.set_shader_program(renderable.shader_id, resources_manager);

            for _ in 0..self.meshes_per_shader[&renderable.shader_id.index()].len() {
                let renderable_index = self.command_queue.indices[index_index];
                let renderable = &self.renderables[renderable_index];
                let instances = self.instances_per_mesh[&hash(renderable)];

                memory_manager.reserve_instance_space(instances);
                upload_draw_data(memory_manager, resources_manager, renderable, instances);
                self.pending_indirect_command_count += 1;

                for _ in 0..instances {
                    let renderable_index = self.command_queue.indices[index_index];
                    let renderable = &self.renderables[renderable_index];

                    memory_manager.push_instance_data(&InstanceData {
                        material_index: renderable.material_id.index(),
                        transform: renderable.transform.transpose(),
                    });

                    index_index += 1;
                }
            }

            make_draw_call(self.gl, memory_manager, self.pending_indirect_command_count);
        }

        self.renderables.clear();
        for mesh_set in self.meshes_per_shader.values_mut() {
            mesh_set.clear();
        }
        self.instances_per_mesh.clear();
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

fn make_draw_call(gl: &gl::Context, memory_manager: &mut MemoryManager, command_count: u32) {
    unsafe {
        gl.multi_draw_elements_indirect_offset(
            gl::TRIANGLES,
            gl::UNSIGNED_INT,
            ((memory_manager.get_indirect_command_index() - command_count) * DRAW_COMMAND_SIZE)
                as i32,
            command_count as i32,
            0,
        );
    }
}
