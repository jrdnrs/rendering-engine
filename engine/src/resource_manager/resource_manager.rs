use std::{collections::VecDeque, marker::PhantomData};

use glow::{self as gl, HasContext};

use super::{
    framebuffer::{Framebuffer, FramebufferConfig},
    gl_image::GlImage,
    model::*,
    shader::*,
    texture::*,
};

pub struct ResourcesManager<'a> {
    gl: &'a gl::Context,

    pub mesh_manager: ResourceManager<Mesh, MeshID>,
    pub material_manager: ResourceManager<Material, MaterialID>,
    pub shader_program_manager: ResourceManager<Program<'a>, ShaderProgramID>,
    pub texture_manager: ResourceManager<Texture<'a>, TextureID>,
    pub framebuffer_manager: ResourceManager<Framebuffer<'a>, FramebufferID>,

    // temp
    pub resize_framebuffers: Vec<FramebufferID>,
}

impl<'a> ResourcesManager<'a> {
    pub fn new(gl: &'a gl::Context) -> Self {
        ResourcesManager {
            gl,
            mesh_manager: ResourceManager::new(),
            material_manager: ResourceManager::new(),
            shader_program_manager: ResourceManager::new(),
            texture_manager: ResourceManager::new(),
            framebuffer_manager: ResourceManager::new(),

            resize_framebuffers: Vec::new(),
        }

        // TODO: load default texture and material
    }

    pub fn load_texture(
        &mut self,
        path: &'static str,
        config: &TextureConfig,
    ) -> Result<TextureID, String> {
        match GlImage::from_path(path) {
            Ok(img) => {
                let texture = Texture::new_2d(self.gl, img, config);
                let id = self.texture_manager.load(texture);
                Ok(id)
            }
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn load_skybox_textures(
        &mut self,
        paths: [&'static str; 6],
        config: &TextureConfig,
    ) -> Result<TextureID, String> {
        let mut images: [GlImage; 6] = Default::default();

        for (i, path) in paths.iter().enumerate() {
            match GlImage::from_path(path) {
                Ok(img) => images[i] = img,
                Err(err) => return Err(err.to_string()),
            }
        }

        let texture = Texture::new_cubemap(self.gl, images, config);
        let id = self.texture_manager.load(texture);
        Ok(id)
    }

    fn remove_texture(&mut self, texture_id: TextureID) {
        // problem: texture handles are uploaded as a Material into ssbo
        // solution: we make the texture non resident, and reupload all materials but now point dead texture ids to
        // a default texture instead? this seems slow, better idea? I guess we kind of have to do this, i just don't
        // like that we iterate through all materials, when we want only ones with this texture. If textures don't
        // change that often, maybe it's fine
    }

    pub fn load_shader(&mut self, path: &'static str) -> ShaderProgramID {
        let mut program = Program::new(self.gl);
        program.add_shaders(path);
        self.shader_program_manager.load(program)
    }

    fn remove_shader(&mut self, shader_id: ShaderProgramID) {

        // problem: what if a renderable references a dead shader?
        // solution: I think we just don't render it?

        // problem: shader bucket exists
        // solution: remove shader bucket, but that will offset the index!!!
    }

    pub fn load_mesh(&mut self, mesh: Mesh) -> MeshID {
        self.mesh_manager.load(mesh)
    }

    fn remove_mesh(&mut self, mesh_id: MeshID) {
        // problem: what if a renderable references a dead mesh?
        // solution: I think we just don't render it?

        // problem: each shader bucket will have a vec for this mesh
        // solution: remove mesh buckets, but that will offset the index!!!
    }

    pub fn load_material(&mut self, material: Material) -> MaterialID {
        self.material_manager.load(material)
    }

    fn remove_material(&mut self, material_id: MaterialID) {
        // problem: this material has been uploaded to ssbo
        // solution: just leave it there, we'll make sure not to reference it when rendering future renderables
        // and the id will be recycled when a new material is added

        // problem: if doing the above, a material that takes the place of an old one will appear visible to renderables
        // that were referencing that place but were expecting an older material
        // solution: the id we give out has a version and we check this as well

        // problem: what do we do when a renderable references a material that is dead?
        // solution: we replace material id when rendering with id of the default material
    }

    pub fn load_framebuffer(
        &mut self,
        config: &FramebufferConfig,
        auto_resize: bool,
    ) -> FramebufferID {
        let framebuffer = Framebuffer::new(&self.gl, config);
        let id = self.framebuffer_manager.load(framebuffer);
        if auto_resize {
            self.resize_framebuffers.push(id);
        }
        id
    }

    pub fn borrow_mesh(&self, mesh_id: &MeshID) -> Option<&Mesh> {
        self.mesh_manager.borrow(mesh_id)
    }

    pub fn borrow_material(&self, material_id: &MaterialID) -> Option<&Material> {
        self.material_manager.borrow(material_id)
    }

    pub fn borrow_shader_program(&self, shader_id: &ShaderProgramID) -> Option<&Program> {
        self.shader_program_manager.borrow(shader_id)
    }

    pub fn borrow_texture(&self, texture_id: &TextureID) -> Option<&Texture> {
        self.texture_manager.borrow(texture_id)
    }

    pub fn borrow_framebuffer(&self, framebuffer_id: &FramebufferID) -> Option<&Framebuffer> {
        self.framebuffer_manager.borrow(framebuffer_id)
    }

    pub fn borrow_mut_mesh(&mut self, mesh_id: &MeshID) -> Option<&mut Mesh> {
        self.mesh_manager.borrow_mut(mesh_id)
    }

    pub fn borrow_mut_material(&mut self, material_id: &MaterialID) -> Option<&mut Material> {
        self.material_manager.borrow_mut(material_id)
    }

    pub fn borrow_mut_shader_program(
        &mut self,
        shader_id: &ShaderProgramID,
    ) -> Option<&mut Program<'a>> {
        self.shader_program_manager.borrow_mut(shader_id)
    }

    pub fn borrow_mut_texture(&mut self, texture_id: &TextureID) -> Option<&mut Texture<'a>> {
        self.texture_manager.borrow_mut(texture_id)
    }

    pub fn borrow_mut_framebuffer(
        &mut self,
        framebuffer_id: &FramebufferID,
    ) -> Option<&mut Framebuffer<'a>> {
        self.framebuffer_manager.borrow_mut(framebuffer_id)
    }
}

const ENTITY_INDEX_BITS: u32 = 22;
const ENTITY_INDEX_MASK: u32 = (1 << ENTITY_INDEX_BITS) - 1;
const MINIMUM_FREE_SPACES: u32 = 0; // keep resource_id list packed, as they directly correspond to actual resources

pub trait ResourceManagerTrait {
    type ResourceType;
    type ResourceIDType;

    fn new() -> Self
    where
        Self: Sized;
    fn load(&mut self, resource: Self::ResourceType) -> Self::ResourceIDType;
    fn borrow(&self, resource_id: &Self::ResourceIDType) -> Option<&Self::ResourceType>;
    fn borrow_mut(&mut self, resource_id: &Self::ResourceIDType)
    -> Option<&mut Self::ResourceType>;
    fn alive(&self, resource_id: &Self::ResourceIDType) -> bool;
    fn remove(&mut self, resource_id: Self::ResourceIDType);
}

pub trait ResourceIDTrait {
    fn new(id: u32) -> Self
    where
        Self: Sized;
    fn index(&self) -> u32;
    fn version(&self) -> u32;
}

pub struct ResourceManager<Resource, ResourceID: ResourceIDTrait> {
    pub resources: Vec<Resource>,
    pub alive_count: u32,
    resource_versions: Vec<u32>,
    free_spaces: VecDeque<u32>,
    _id_type: PhantomData<ResourceID>,
}

impl<T, U: ResourceIDTrait> ResourceManagerTrait for ResourceManager<T, U> {
    type ResourceType = T;
    type ResourceIDType = U;

    fn new() -> Self {
        Self {
            resources: Vec::new(),
            alive_count: 0,
            resource_versions: Vec::new(),
            free_spaces: VecDeque::new(),
            _id_type: PhantomData,
        }
    }

    fn load(&mut self, resource: T) -> U {
        let id: u32;
        if self.free_spaces.len() as u32 > MINIMUM_FREE_SPACES {
            let index = self.free_spaces.pop_front().unwrap();
            id = (self.resource_versions[index as usize] << ENTITY_INDEX_BITS) | index;
            self.resources[index as usize] = resource;
        } else {
            self.resource_versions.push(0);
            id = self.resource_versions.len() as u32 - 1;
            self.resources.push(resource)
        }

        self.alive_count += 1;
        U::new(id)
    }

    fn borrow(&self, resource_id: &U) -> Option<&T> {
        if self.alive(resource_id) {
            Some(&self.resources[resource_id.index() as usize])
        } else {
            None
        }
    }

    fn borrow_mut(&mut self, resource_id: &U) -> Option<&mut T> {
        if self.alive(resource_id) {
            Some(&mut self.resources[resource_id.index() as usize])
        } else {
            None
        }
    }

    fn alive(&self, resource_id: &U) -> bool {
        self.resource_versions[resource_id.index() as usize] == resource_id.version()
    }

    fn remove(&mut self, resource_id: U) {
        if self.alive(&resource_id) {
            let index = resource_id.index();
            self.resource_versions[index as usize] += 1;
            self.free_spaces.push_back(index);
            self.alive_count -= 1;
            // we don't actually remove the resource
            // as the resourceID is now regarded dead, the renderable that holds it will be treated differently
            // eventually this resource place will be overwritten the next time one is created
        }
    }
}

// TODO: make this a procedural derive macro
macro_rules! impl_resourceID {
    ($id:ident) => {
        impl ResourceIDTrait for $id {
            fn new(id: u32) -> Self {
                Self { id }
            }

            fn index(&self) -> u32 {
                self.id & ENTITY_INDEX_MASK
            }
            fn version(&self) -> u32 {
                self.id >> ENTITY_INDEX_BITS
            }
        }
    };
}

impl_resourceID!(MeshID);
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MeshID {
    id: u32,
}

impl_resourceID!(TextureID);
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TextureID {
    id: u32,
}

impl_resourceID!(ShaderProgramID);
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ShaderProgramID {
    id: u32,
}

impl_resourceID!(MaterialID);
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MaterialID {
    id: u32,
}

impl_resourceID!(FramebufferID);
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FramebufferID {
    id: u32,
}
