use super::PipelineStage;
use crate::{
    components::Renderable,
    graphics::{
        self,
        state::{Orientation, RasteriserState},
    },
    math::Mat4f,
    memory_manager::memory_manager::{
        DrawElementsIndirectCommand, InstanceData, MemoryManager, DRAW_COMMAND_SIZE,
    },
    renderer::{command::DrawCommands, state::RendererState},
    resource_manager::resource_manager::{
        FramebufferID, MaterialID, MeshID, ResourceIDTrait, ResourcesManager, ShaderProgramID,
    },
};

pub struct DebugStage {
    target: FramebufferID,
    wireframe_shader_id: ShaderProgramID,
    vertices_shader_id: ShaderProgramID,
    renderable_indices: Vec<usize>,
    command_queue: DrawCommands,
    pending_indirect_command_count: u32,
}

impl DebugStage {
    pub fn new(target: FramebufferID, resources_manager: &mut ResourcesManager) -> Self {
        let wireframe_shader_id = resources_manager.load_shader("res/shaders/debug_wireframe.glsl");
        let vertices_shader_id = resources_manager.load_shader("res/shaders/debug_vertices.glsl");

        Self {
            target,
            renderable_indices: Vec::new(),
            command_queue: DrawCommands::new(hash),
            pending_indirect_command_count: 0,
            wireframe_shader_id,
            vertices_shader_id,
        }
    }
}

impl PipelineStage for DebugStage {
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
        rasteriser_state: &mut RasteriserState,
        renderables: &[Renderable],
    ) {
        self.command_queue
            .update_keys(renderables, &self.renderable_indices);
        self.command_queue.sort_indices();

        let mut instance_count = 0;

        let r = Renderable {
            mesh_id: MeshID::new(0xFFFF),
            material_id: MaterialID::new(0xFFFF),
            shader_id: ShaderProgramID::new(0xFFFF),
            transform: Mat4f::identity(),
            pipeline_stages: 0,
        };

        for i in 0..self.command_queue.indices.len() {
            let renderable = &renderables[self.renderable_indices[self.command_queue.indices[i]]];
            let next_renderable = if i == self.command_queue.indices.len() - 1 {
                &r
            } else {
                &renderables[self.renderable_indices[self.command_queue.indices[i + 1]]]
            };

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
                    self.command_queue.indices[(i - (instance_count - 1) as usize)..=i].iter()
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

        rasteriser_state.set_polygon_mode(Orientation::FrontAndBack, false);

        renderer_state.set_shader_program(self.wireframe_shader_id, resources_manager);

        graphics::submit_draw_call(
            graphics::DrawMode::Triangles,
            graphics::DataType::Uint32,
            (memory_manager.get_indirect_command_index() - self.pending_indirect_command_count)
                * DRAW_COMMAND_SIZE,
            self.pending_indirect_command_count,
        );

        rasteriser_state.set_polygon_mode(Orientation::FrontAndBack, true);

        renderer_state.set_shader_program(self.vertices_shader_id, resources_manager);

        graphics::submit_draw_call(
            graphics::DrawMode::Points,
            graphics::DataType::Uint32,
            (memory_manager.get_indirect_command_index() - self.pending_indirect_command_count)
                * DRAW_COMMAND_SIZE,
            self.pending_indirect_command_count,
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
