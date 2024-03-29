use std::time;

use ecs::World;
use glutin::{
    event::{DeviceEvent, Event, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
};
use log::debug;

use crate::{
    components,
    context::Context,
    input::input::Input,
    math::*,
    memory_manager::uniform_layouts::{DirectionalLight, PointLight, SpotLight},
    renderer::{pipeline_stages, renderer::Renderer},
    resource_manager::{
        model::Material,
        prefabs::{self, sphere, unit_cube_mesh},
    }, graphics::{texture::{TextureFilter, TextureConfig, TextureWrap}, self},
};

pub struct Engine<'a> {
    context: Context,
    renderer: Renderer<'a>,
    input: Input,
    world: World,
}

impl Engine<'_> {
    pub fn new(context: Context) -> Self {
        Engine {
            context,
            renderer: Renderer::new(),
            input: Input::new(),
            world: World::new(),
        }
    }

    fn process_input(&mut self) {
        let delta_time = self.context.last_frame_delta.as_secs_f32();
        let sensitivity = 10.0 * delta_time;
        let move_speed = 5.0 * delta_time;

        if self.input.is_key_down(VirtualKeyCode::Escape) {
            self.context
                .window_context
                .window()
                .set_cursor_grab(false)
                .unwrap();
            self.context
                .window_context
                .window()
                .set_cursor_visible(true);
            self.input.mouse.grabbed = false;
        }

        if self.input.mouse.is_button_pressed(MouseButton::Left) {
            self.context
                .window_context
                .window()
                .set_cursor_grab(true)
                .unwrap();
            self.context
                .window_context
                .window()
                .set_cursor_visible(false);
            self.input.mouse.grabbed = true;
        }

        // only process movements if we have grabbed cursor
        if self.input.mouse.grabbed {
            if self.input.mouse.moved {
                let x = self.input.mouse.delta_x as f32 * sensitivity;
                let y = self.input.mouse.delta_y as f32 * sensitivity;

                self.renderer.camera.yaw += x;
                self.renderer.camera.pitch += y;

                self.renderer.camera.pitch = self.renderer.camera.pitch.clamp(-89.0, 89.0);

                self.renderer.camera.direction = Vec3f::new(
                    self.renderer.camera.yaw.to_radians().cos()
                        * self.renderer.camera.pitch.to_radians().cos(),
                    self.renderer.camera.pitch.to_radians().sin(),
                    self.renderer.camera.yaw.to_radians().sin()
                        * self.renderer.camera.pitch.to_radians().cos(),
                )
                .normalise();

                // debug!(
                //     "x: {}, y: {}, z: {}",
                //     self.renderer.camera.direction.x,
                //     self.renderer.camera.direction.y,
                //     self.renderer.camera.direction.z
                // )
            }

            if self.input.is_key_down(VirtualKeyCode::F2) {
                debug!("Reloading shaders");
                for p in self
                    .renderer
                    .resources_manager
                    .shader_program_manager
                    .resources
                    .iter_mut()
                {
                    p.reload_shaders();
                }
            }

            if self.input.is_key_down(VirtualKeyCode::F3) {
                if self
                    .renderer
                    .renderer_pipeline
                    .is_enabled(pipeline_stages::STAGE_DEBUG)
                {
                    debug!("Debug render stage: OFF");
                    self.renderer
                        .renderer_pipeline
                        .disable_stages(pipeline_stages::STAGE_DEBUG)
                } else {
                    debug!("Debug render stage: ON");
                    self.renderer
                        .renderer_pipeline
                        .enable_stages(pipeline_stages::STAGE_DEBUG)
                }
            }

            if self.input.is_key_down(VirtualKeyCode::W) {
                self.renderer.camera.position -= self.renderer.camera.direction * move_speed;
            }

            if self.input.is_key_down(VirtualKeyCode::A) {
                self.renderer.camera.position += self
                    .renderer
                    .camera
                    .direction
                    .cross(Vec3f::new(0.0, 1.0, 0.0))
                    .normalise()
                    * move_speed;
            }

            if self.input.is_key_down(VirtualKeyCode::S) {
                self.renderer.camera.position += self.renderer.camera.direction * move_speed;
            }

            if self.input.is_key_down(VirtualKeyCode::D) {
                self.renderer.camera.position -= self
                    .renderer
                    .camera
                    .direction
                    .cross(Vec3f::new(0.0, 1.0, 0.0))
                    .normalise()
                    * move_speed;
            }

            if self.input.is_key_down(VirtualKeyCode::Space) {
                self.renderer.camera.position += Vec3f::new(0.0, 1.0, 0.0) * move_speed;
            }

            if self.input.is_key_down(VirtualKeyCode::LShift) {
                self.renderer.camera.position -= Vec3f::new(0.0, 1.0, 0.0) * move_speed;
            }
        }
    }

    /// This runs once before rendering occurs
    fn setup(&mut self) {
        graphics::set_clear_color(0.4, 0.5, 0.9, 1.0);
        let window_size = self.context.window_context.window().inner_size();
        self.renderer
            .set_viewport(window_size.width, window_size.height);

        let lamp_texture_id = self
            .renderer
            .load_texture(
                "res/textures/lamp.dds",
                &TextureConfig {
                    min_filter: TextureFilter::Linear,
                    mag_filter: TextureFilter::Nearest,
                    mipmap: true,
                    srgb: true,
                    ..Default::default()
                },
            )
            .unwrap();
        let ground_texture_id = self
            .renderer
            .load_texture(
                "res/textures/mossy_cobblestone.dds",
                &TextureConfig {
                    min_filter: TextureFilter::Linear,
                    mag_filter: TextureFilter::Nearest,
                    mipmap: true,
                    srgb: true,
                    ..Default::default()
                },
            )
            .unwrap();
        let wood_texture_id = self
            .renderer
            .load_texture(
                "res/textures/oak_planks.dds",
                &TextureConfig {
                    min_filter: TextureFilter::Linear,
                    mag_filter: TextureFilter::Nearest,
                    mipmap: true,
                    srgb: true,
                    ..Default::default()
                },
            )
            .unwrap();

        let lamp_specular_texture_id = self
            .renderer
            .load_texture(
                "res/textures/lamp_s.dds",
                &TextureConfig {
                    min_filter: TextureFilter::Linear,
                    mag_filter: TextureFilter::Nearest,
                    mipmap: false,
                    srgb: false,
                    ..Default::default()
                },
            )
            .unwrap();
        let wood_specular_texture_id = self
            .renderer
            .load_texture(
                "res/textures/oak_planks_s.dds",
                &TextureConfig {
                    min_filter: TextureFilter::Linear,
                    mag_filter: TextureFilter::Nearest,
                    mipmap: false,
                    srgb: false,
                    ..Default::default()
                },
            )
            .unwrap();

        let lamp_normal_texture_id = self
            .renderer
            .load_texture(
                "res/textures/lamp_n.dds",
                &TextureConfig {
                    min_filter: TextureFilter::Linear,
                    mag_filter: TextureFilter::Nearest,
                    mipmap: false,
                    srgb: false,
                    ..Default::default()
                },
            )
            .unwrap();
        let ground_normal_texture_id = self
            .renderer
            .load_texture(
                "res/textures/mossy_cobblestone_n.dds",
                &TextureConfig {
                    min_filter: TextureFilter::Linear,
                    mag_filter: TextureFilter::Nearest,
                    mipmap: false,
                    srgb: false,
                    ..Default::default()
                },
            )
            .unwrap();
        let wood_normal_texture_id = self
            .renderer
            .load_texture(
                "res/textures/oak_planks_n.dds",
                &TextureConfig {
                    min_filter: TextureFilter::Linear,
                    mag_filter: TextureFilter::Nearest,
                    mipmap: false,
                    srgb: false,
                    ..Default::default()
                },
            )
            .unwrap();

        let basic_shader_id = self.renderer.load_shader("res/shaders/basic.glsl");
        let light_shader_id = self.renderer.load_shader("res/shaders/lighting.glsl");
        let skybox_shader_id = self.renderer.load_shader("res/shaders/skybox.glsl");

        let ground_material_id = self.renderer.load_material(Material {
            shininess: 16.0,
            diffuse_texture_id: Some(ground_texture_id),
            specular_texture_id: None,
            normal_texture_id: Some(ground_normal_texture_id),
        });

        let wood_material_id = self.renderer.load_material(Material {
            shininess: 16.0,
            diffuse_texture_id: Some(wood_texture_id),
            specular_texture_id: Some(wood_specular_texture_id),
            normal_texture_id: Some(wood_normal_texture_id),
        });

        let blank_material_id = self.renderer.load_material(Material {
            shininess: 16.0,
            diffuse_texture_id: None,
            specular_texture_id: None,
            normal_texture_id: None,
        });

        let lamp_material_id = self.renderer.load_material(Material {
            shininess: 128.0,
            diffuse_texture_id: Some(lamp_texture_id),
            specular_texture_id: Some(lamp_specular_texture_id),
            normal_texture_id: Some(lamp_normal_texture_id),
        });

        let cube_model_id = self
            .renderer
            .load_mesh(unit_cube_mesh(Vec4f::new(0.95, 0.85, 0.65, 0.85)));

        let sphere_model_id = self.renderer.load_mesh(sphere(12));

        self.world
            .register_component::<components::SpotLightBlock>();
        self.world
            .register_component::<components::PointLightBlock>();
        self.world.register_component::<components::DirLightBlock>();
        self.world.register_component::<components::Block>();
        self.world.register_component::<components::Renderable>();

        let skybox_texture_id = self
            .renderer
            .load_skybox_textures(
                [
                    "res/textures/skybox/CoriolisNight/px.dds",
                    "res/textures/skybox/CoriolisNight/nx.dds",
                    "res/textures/skybox/CoriolisNight/py.dds",
                    "res/textures/skybox/CoriolisNight/ny.dds",
                    "res/textures/skybox/CoriolisNight/pz.dds",
                    "res/textures/skybox/CoriolisNight/nz.dds",
                ],
                &TextureConfig {
                    wrap: TextureWrap::ClampToEdge,
                    mag_filter: TextureFilter::Linear,
                    min_filter: TextureFilter::Linear,
                    mipmap: false,
                    srgb: true,
                },
            )
            .unwrap();

        let skybox_material_id = self.renderer.load_material(Material {
            shininess: 0.0,
            diffuse_texture_id: Some(skybox_texture_id),
            specular_texture_id: None,
            normal_texture_id: None,
        });

        let skybox = self.world.create_entity();
        let skybox_component = components::Renderable {
            mesh_id: cube_model_id,
            material_id: skybox_material_id,
            shader_id: skybox_shader_id,
            transform: Mat4f::identity(),
            pipeline_stages: pipeline_stages::STAGE_SKY,
        };
        _ = self.world.set_component(&skybox, skybox_component);

        let cubes = 50;
        let mut position = Vec3f::new(0.0, 0.0, 5.0);
        for _ in 0..cubes {
            position.z -= cubes as f32;
            position.x += 1.0;

            for _ in 0..cubes {
                // position.y -= cubes as f32;
                position.z += 1.0;

                // for _ in 0..cubes {
                //     position.y += 1.0;

                let block = self.world.create_entity();
                _ = self.world.set_component(&block, components::Block {});
                _ = self.world.set_component(
                    &block,
                    components::Renderable {
                        mesh_id: cube_model_id,
                        material_id: ground_material_id,
                        shader_id: light_shader_id,
                        transform: Mat4f::translate(position.x, position.y, position.z),
                        pipeline_stages: pipeline_stages::STAGE_SCENE,
                    },
                );

                // }
            }
        }

        for i in 1..6 {
            let block = self.world.create_entity();
            _ = self.world.set_component(&block, components::Block {});
            _ = self.world.set_component(
                &block,
                components::Renderable {
                    mesh_id: cube_model_id,
                    material_id: wood_material_id,
                    shader_id: light_shader_id,
                    transform: Mat4f::translate(3.0, i as f32, -5.0),
                    pipeline_stages: pipeline_stages::STAGE_SCENE | pipeline_stages::STAGE_SHADOW,
                },
            );
        }

        for i in 1..6 {
            let block = self.world.create_entity();
            _ = self.world.set_component(&block, components::Block {});
            _ = self.world.set_component(
                &block,
                components::Renderable {
                    mesh_id: cube_model_id,
                    material_id: wood_material_id,
                    shader_id: light_shader_id,
                    transform: Mat4f::translate(7.0, i as f32, -17.0),
                    pipeline_stages: pipeline_stages::STAGE_SCENE | pipeline_stages::STAGE_SHADOW,
                },
            );
        }

        for i in 1..6 {
            let block = self.world.create_entity();
            _ = self.world.set_component(&block, components::Block {});
            _ = self.world.set_component(
                &block,
                components::Renderable {
                    mesh_id: cube_model_id,
                    material_id: wood_material_id,
                    shader_id: light_shader_id,
                    transform: Mat4f::translate(24.0, i as f32, -20.0),
                    pipeline_stages: pipeline_stages::STAGE_SCENE | pipeline_stages::STAGE_SHADOW,
                },
            );
        }

        for i in 1..6 {
            let block = self.world.create_entity();
            _ = self.world.set_component(&block, components::Block {});
            _ = self.world.set_component(
                &block,
                components::Renderable {
                    mesh_id: cube_model_id,
                    material_id: wood_material_id,
                    shader_id: light_shader_id,
                    transform: Mat4f::translate(24.0, i as f32, -8.0),
                    pipeline_stages: pipeline_stages::STAGE_SCENE | pipeline_stages::STAGE_SHADOW,
                },
            );
        }

        for i in 1..6 {
            let block = self.world.create_entity();
            _ = self.world.set_component(&block, components::Block {});
            _ = self.world.set_component(
                &block,
                components::Renderable {
                    mesh_id: cube_model_id,
                    material_id: wood_material_id,
                    shader_id: light_shader_id,
                    transform: Mat4f::translate(15.0, i as f32, 0.0),
                    pipeline_stages: pipeline_stages::STAGE_SCENE | pipeline_stages::STAGE_SHADOW,
                },
            );
        }

        let sphere = self.world.create_entity();
        _ = self.world.set_component(
            &sphere,
            components::Renderable {
                mesh_id: sphere_model_id,
                material_id: ground_material_id,
                shader_id: light_shader_id,
                transform: Mat4f::translate(10.0, 3.0, -10.0),
                pipeline_stages: pipeline_stages::STAGE_SCENE
                    | pipeline_stages::STAGE_SHADOW
                    | pipeline_stages::STAGE_DEBUG,
            },
        );

        let lamp = self.world.create_entity();
        _ = self.world.set_component(
            &lamp,
            // components::PointLightBlock {
            //     attenuation: Vec3f::new(0.0028, 0.027, 1.0),
            // },
            components::DirLightBlock {
                direction: Vec3f::new(-0.5, 0.33, 0.5),
            },
        );
        _ = self.world.set_component(
            &lamp,
            components::Renderable {
                mesh_id: cube_model_id,
                material_id: lamp_material_id,
                shader_id: basic_shader_id,
                transform: Mat4f::translate(0.0, 16.0, 0.0),
                pipeline_stages: pipeline_stages::STAGE_SCENE,
            },
        );

        // let lamp = self.world.create_entity();
        // _ = self.world.set_component(
        //     &lamp,
        //     components::PointLightBlock {
        //         attenuation: Vec3f::new(0.0075, 0.045, 1.0),
        //     },
        // );
        // _ = self.world.set_component(
        //     &lamp,
        //     components::Renderable {
        //         mesh_id: cube_model_id,
        //         material_id: lamp_material_id,
        //         shader_id: basic_shader_id,
        //         transform: Mat4f::translate(24.0, 6.0, -16.0),
        //         pipeline_stages: pipeline_stages::STAGE_SCENE,
        //     },
        // );

        // let lamp = self.world.create_entity();
        // _ = self.world.set_component(
        //     &lamp,
        //     components::SpotLightBlock {
        //         attenuation: Vec3f::new(0.017, 0.07, 1.0),
        //         inner_cutoff_cos: 15f32.to_radians().cos(),
        //         outer_cutoff_cos: 55f32.to_radians().cos(),
        //         direction: Vec3f::new(-0.69, 0.23, -0.69),
        //     },
        // );
        // _ = self.world.set_component(
        //     &lamp,
        //     components::Renderable {
        //         mesh_id: cube_model_id,
        //         material_id: lamp_material_id,
        //         shader_id: basic_shader_id,
        //         transform: Mat4f::translate(4.0, 4.0, -16.0),
        //         pipeline_stages: pipeline_stages::STAGE_SCENE,
        //     },
        // );

        // let lamp = self.world.create_entity();
        // _ = self.world.set_component(
        //     &lamp,
        //     components::SpotLightBlock {
        //         attenuation: Vec3f::new(0.017, 0.07, 1.0),
        //         inner_cutoff_cos: 15f32.to_radians().cos(),
        //         outer_cutoff_cos: 55f32.to_radians().cos(),
        //         direction: Vec3f::new(-0.69, 0.23, 0.69),
        //     },
        // );
        // _ = self.world.set_component(
        //     &lamp,
        //     components::Renderable {
        //         mesh_id: cube_model_id,
        //         material_id: lamp_material_id,
        //         shader_id: basic_shader_id,
        //         transform: Mat4f::translate(4.0, 4.0, -5.0),
        //         pipeline_stages: pipeline_stages::STAGE_SCENE,
        //     },
        // );

        let axis_mesh = prefabs::axis();
        let axis_mesh_id = self.renderer.load_mesh(axis_mesh);

        let axis = self.world.create_entity();
        _ = self.world.set_component(
            &axis,
            components::Renderable {
                mesh_id: axis_mesh_id,
                material_id: lamp_material_id,
                shader_id: basic_shader_id,
                transform: Mat4f::translate(-0.5, -0.5, 0.0),
                pipeline_stages: pipeline_stages::STAGE_DEBUG,
            },
        )
    }

    /// This runs once per frame
    fn update(&mut self) {
        self.process_input();
        self.draw();
        self.input.mouse.moved = false;
        self.input.mouse.delta_x = 0.0;
        self.input.mouse.delta_y = 0.0;
        self.context.frames += 1;
    }

    fn draw(&mut self) {
        self.renderer.begin();

        for (spot_light, renderable) in self
            .world
            .get_current_view_mut()
            .iter_two_components_mut::<components::SpotLightBlock, components::Renderable>()
            .unwrap()
        {
            self.renderer.add_spot_light(SpotLight {
                ambient_col: Vec3f::new(0.91, 0.65, 0.36) * 0.15,
                diffuse_col: Vec3f::new(0.91, 0.65, 0.36) * 1.5,
                specular_col: Vec3f::new(0.5, 0.5, 0.5) * 0.15,

                attenuation: spot_light.attenuation,

                inner_cutoff: spot_light.inner_cutoff_cos,
                outer_cutoff: spot_light.outer_cutoff_cos,

                position: Vec3f::new(
                    renderable.transform[(0, 3)],
                    renderable.transform[(1, 3)],
                    renderable.transform[(2, 3)],
                ),
                direction: spot_light.direction,
                ..Default::default()
            })
        }

        for (point_light, renderable) in self
            .world
            .get_current_view_mut()
            .iter_two_components_mut::<components::PointLightBlock, components::Renderable>()
            .unwrap()
        {
            self.renderer.add_point_light(PointLight {
                ambient_col: Vec3f::new(0.91, 0.65, 0.36) * 0.15,
                diffuse_col: Vec3f::new(0.91, 0.65, 0.36) * 1.5,
                specular_col: Vec3f::new(0.5, 0.5, 0.5) * 0.15,

                attenuation: point_light.attenuation,

                position: Vec3f::new(
                    renderable.transform[(0, 3)],
                    renderable.transform[(1, 3)],
                    renderable.transform[(2, 3)],
                ),
                ..Default::default()
            })
        }

        for (dir_light, renderable) in self
            .world
            .get_current_view_mut()
            .iter_two_components_mut::<components::DirLightBlock, components::Renderable>()
            .unwrap()
        {
            self.renderer.set_directional_light(DirectionalLight {
                ambient_col: Vec3f::new(0.91, 0.65, 0.36) * 0.15,
                diffuse_col: Vec3f::new(0.91, 0.65, 0.36),
                specular_col: Vec3f::new(0.5, 0.5, 0.5) * 0.15,

                position: Vec3f::new(
                    renderable.transform[(0, 3)],
                    renderable.transform[(1, 3)],
                    renderable.transform[(2, 3)],
                ),
                direction: dir_light.direction,

                ..Default::default()
            })
        }

        for renderable in self
            .world
            .get_current_view_mut()
            .iter_components_mut()
            .unwrap()
        {
            self.renderer.draw(&renderable);
        }

        self.renderer.end();
    }

    pub fn run(mut self, mut event_loop: EventLoop<()>) {
        self.setup();

        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::LoopDestroyed => {
                    for p in self
                        .renderer
                        .resources_manager
                        .shader_program_manager
                        .resources
                        .iter()
                    {
                        p.delete();
                    }
                    return;
                }
                Event::DeviceEvent { event, .. } => match event {
                    DeviceEvent::MouseMotion { delta } => {
                        self.input.mouse.delta_x += delta.0;
                        self.input.mouse.delta_y += delta.1;
                        self.input.mouse.moved = true;
                    }
                    _ => (),
                },
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::MouseInput { state, button, .. } => {
                        self.input.mouse.handle_input(button, state);
                    }

                    WindowEvent::KeyboardInput { input, .. } => {
                        self.input.handle_input(input);
                    }

                    WindowEvent::CursorEntered { .. } => {
                        self.input.mouse.on_window = true;
                    }

                    WindowEvent::CursorLeft { .. } => {
                        self.input.mouse.on_window = false;
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        self.input.mouse.pos_x = position.x;
                        self.input.mouse.pos_y = position.y;
                    }

                    WindowEvent::Resized(ref physical_size) => {
                        self.context.being_resized = true;
                        self.context.window_context.resize(*physical_size);
                        self.renderer
                            .set_viewport(physical_size.width, physical_size.height);
                    }
                    WindowEvent::Moved(_) => {
                        self.context.being_moved = true;
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => {}
                },
                Event::MainEventsCleared if crate::VSYNC => {
                    self.context.window_context.window().request_redraw();
                }
                // Event::MainEventsCleared => {
                //     // microseconds left before target time, before we should spin
                //     // poor timings on Windows means this should be >= 1000
                //     let spin_threshold = 1000;

                //     let now = time::Instant::now();
                //     if now >= self.context.target_time {
                //         self.context.target_time = now + self.context.target_frametime;
                //         self.context.window_context.window().request_redraw();
                //     }

                //     let now = time::Instant::now();
                //     let delta = self.context.target_time - now;
                //     if delta < time::Duration::from_micros(spin_threshold) {
                //         return;
                //     }

                //     let mut sleep_time = time::Duration::from_micros(
                //         (delta.as_micros() - (delta.as_micros() % spin_threshold as u128)) as u64,
                //     );

                //     if delta > time::Duration::from_micros((1000000.0 / 144.0) as u64)
                //         && (self.context.being_moved || self.context.being_resized)
                //     {
                //         sleep_time = time::Duration::from_micros(
                //             ((1000000.0 / 144.0) - ((1000000.0 / 144.0) % spin_threshold as f64))
                //                 as u64,
                //         );
                //     }

                //     spin_sleep::sleep(sleep_time);
                // }
                Event::RedrawRequested(_) => {
                    self.update();
                  
                    self.context.window_context.swap_buffers().unwrap();

                    let now = time::Instant::now();
                    self.context.last_frame_delta = now - self.context.last_frame_time;
                    self.context.last_frame_time = time::Instant::now();
                }
                _ => (),
            }
        });
    }
}
