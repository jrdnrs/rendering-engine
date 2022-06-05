use std::time;

use ecs::World;
use glow::{self as gl, HasContext};
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
    math::math::*,
    memory_manager::uniform_layouts::Light,
    renderer::{pipeline_stages, renderer::Renderer},
    resource_manager::{model::Material, prefabs::{unit_cube_mesh, self}, texture::TextureConfig},
};

pub struct Engine<'a> {
    gl: &'a gl::Context,
    context: Context,
    renderer: Renderer<'a>,
    input: Input,
    world: World,
}

impl<'a> Engine<'a> {
    pub fn new(context: Context, gl: &'a gl::Context) -> Self {
        Engine {
            context,
            gl,
            renderer: Renderer::new(gl),
            input: Input::new(),
            world: World::new(),
        }
    }

    fn process_input(&mut self) {
        let delta_time = self.context.last_frame_delta.as_secs_f32();
        let sensitivity = 40.0 * delta_time;
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

                if self.renderer.camera.pitch > 89.0 {
                    self.renderer.camera.pitch = 89.0
                };
                if self.renderer.camera.pitch < -89.0 {
                    self.renderer.camera.pitch = -89.0
                };

                self.renderer.camera.direction = Vec3f::new(
                    self.renderer.camera.yaw.to_radians().cos()
                        * self.renderer.camera.pitch.to_radians().cos(),
                    self.renderer.camera.pitch.to_radians().sin(),
                    self.renderer.camera.yaw.to_radians().sin()
                        * self.renderer.camera.pitch.to_radians().cos(),
                )
                .normalise();

                // debug!("x: {}, y: {}, z: {}", self.camera.direction.x, self.camera.direction.y, self.camera.direction.x)
            }

            if self.input.is_key_down(VirtualKeyCode::F1) {
                self.context.wireframe = !self.context.wireframe;

                if self.context.wireframe {
                    unsafe { self.gl.polygon_mode(gl::FRONT_AND_BACK, gl::LINE) }
                    debug!("Wireframe enabled")
                } else {
                    unsafe { self.gl.polygon_mode(gl::FRONT_AND_BACK, gl::FILL) }
                    debug!("Wireframe disabled")
                }
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
                    p.reload();
                }
            }

            if self.input.is_key_down(VirtualKeyCode::W) {
                self.renderer.camera.position -= self.renderer.camera.direction.scalar(move_speed);
            }

            if self.input.is_key_down(VirtualKeyCode::A) {
                self.renderer.camera.position += self
                    .renderer
                    .camera
                    .direction
                    .cross(Vec3f::new(0.0, 1.0, 0.0))
                    .normalise()
                    .scalar(move_speed);
            }

            if self.input.is_key_down(VirtualKeyCode::S) {
                self.renderer.camera.position += self.renderer.camera.direction.scalar(move_speed);
            }

            if self.input.is_key_down(VirtualKeyCode::D) {
                self.renderer.camera.position -= self
                    .renderer
                    .camera
                    .direction
                    .cross(Vec3f::new(0.0, 1.0, 0.0))
                    .normalise()
                    .scalar(move_speed);
            }

            if self.input.is_key_down(VirtualKeyCode::Space) {
                self.renderer.camera.position += Vec3f::new(0.0, 1.0, 0.0).scalar(move_speed);
            }

            if self.input.is_key_down(VirtualKeyCode::LShift) {
                self.renderer.camera.position -= Vec3f::new(0.0, 1.0, 0.0).scalar(move_speed);
            }
        }
    }

    /// This runs once before rendering occurs
    fn setup(&mut self) {
        self.renderer.set_clear_colour(0.4, 0.5, 0.9, 1.0);
        let window_size = self.context.window_context.window().inner_size();
        self.renderer
            .set_viewport(window_size.width as i32, window_size.height as i32);

        let lamp_texture_id = self
            .renderer
            .load_texture(
                "res/textures/lamp.dds",
                &TextureConfig {
                    min_filter: gl::LINEAR_MIPMAP_LINEAR,
                    srgb: true,
                    ..Default::default()
                },
            )
            .unwrap();
        let ground_texture_id = self
            .renderer
            .load_texture(
                "res/textures/cobblestone_mossy.dds",
                &TextureConfig {
                    min_filter: gl::LINEAR_MIPMAP_LINEAR,
                    srgb: true,
                    ..Default::default()
                },
            )
            .unwrap();

        let debug_shader_id = self.renderer.load_shader("res/shaders/debug.glsl");
        let basic_shader_id = self.renderer.load_shader("res/shaders/basic.glsl");
        let light_shader_id = self.renderer.load_shader("res/shaders/light.glsl");
        let skybox_shader_id = self.renderer.load_shader("res/shaders/skybox.glsl");

        let ground_material_id = self.renderer.load_material(Material {
            shininess: 16.0,
            diffuse_texture_id: ground_texture_id,
            specular_texture_id: ground_texture_id,
        });

        let lamp_material_id = self.renderer.load_material(Material {
            shininess: 32.0,
            diffuse_texture_id: lamp_texture_id,
            specular_texture_id: lamp_texture_id,
        });

        let cube_model_id = self
            .renderer
            .load_mesh(unit_cube_mesh(Vec4f::new(0.95, 0.85, 0.65, 0.85)));

        self.world.register_component::<components::LightBlock>();
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
                    wrap: gl::CLAMP_TO_EDGE,
                    mag_filter: gl::LINEAR,
                    min_filter: gl::LINEAR,
                    srgb: true,
                },
            )
            .unwrap();

        let skybox_material_id = self.renderer.load_material(Material {
            shininess: 0.0,
            diffuse_texture_id: skybox_texture_id,
            specular_texture_id: skybox_texture_id,
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

        let cubes = 25;
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

        let lamp = self.world.create_entity();
        _ = self.world.set_component(&lamp, components::LightBlock {});
        _ = self.world.set_component(
            &lamp,
            components::Renderable {
                mesh_id: cube_model_id,
                material_id: lamp_material_id,
                shader_id: basic_shader_id,
                transform: Mat4f::translate(20.0, 4.0, -16.0),
                pipeline_stages: pipeline_stages::STAGE_SCENE,
            },
        );

        let lamp = self.world.create_entity();
        _ = self.world.set_component(&lamp, components::LightBlock {});
        _ = self.world.set_component(
            &lamp,
            components::Renderable {
                mesh_id: cube_model_id,
                material_id: lamp_material_id,
                shader_id: basic_shader_id,
                transform: Mat4f::translate(4.0, 4.0, -16.0),
                pipeline_stages: pipeline_stages::STAGE_SCENE,
            },
        );

        let lamp = self.world.create_entity();
        _ = self.world.set_component(&lamp, components::LightBlock {});
        _ = self.world.set_component(
            &lamp,
            components::Renderable {
                mesh_id: cube_model_id,
                material_id: lamp_material_id,
                shader_id: basic_shader_id,
                transform: Mat4f::translate(20.0, 4.0, 0.0),
                pipeline_stages: pipeline_stages::STAGE_SCENE,
            },
        );

        let lamp = self.world.create_entity();
        _ = self.world.set_component(&lamp, components::LightBlock {});
        _ = self.world.set_component(
            &lamp,
            components::Renderable {
                mesh_id: cube_model_id,
                material_id: lamp_material_id,
                shader_id: basic_shader_id,
                transform: Mat4f::translate(4.0, 4.0, 0.0),
                pipeline_stages: pipeline_stages::STAGE_SCENE,
            },
        );

        let axis_mesh = prefabs::axis();
        let axis_mesh_id = self.renderer.load_mesh(axis_mesh);

        let axis = self.world.create_entity();
        _ = self.world.set_component(&axis, components::Renderable{
            mesh_id: axis_mesh_id,
            material_id: lamp_material_id,
            shader_id: debug_shader_id,
            transform: Mat4f::translate(0.0, 0.0, 0.0),
            pipeline_stages: pipeline_stages::STAGE_DEBUG,
        })
    }

    /// This runs once per frame
    fn update(&mut self) {
        self.process_input();
        self.draw();
        self.input.mouse.moved = false;
        self.context.frames += 1;
    }

    fn draw(&mut self) {
        self.renderer.begin();

        for (_light_block, renderable) in self
            .world
            .get_current_view_mut()
            .iter_two_components_mut::<components::LightBlock, components::Renderable>()
            .unwrap()
        {
            self.renderer.add_light(Light {
                ambient_col: Vec3f::new(0.91, 0.65, 0.36),
                ambient_strength: 0.15,
                diffuse_col: Vec3f::new(0.91, 0.65, 0.36),
                diffuse_strength: 1.5,
                specular_col: Vec3f::new(0.5, 0.5, 0.5),
                specular_strength: 0.5,
                quadratic: 0.07,
                linear: 0.14,
                constant: 1.0,
                position: Vec3f::new(
                    renderable.transform.0[3],
                    renderable.transform.0[7],
                    renderable.transform.0[11],
                ),
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
                        self.input.mouse.delta_x = delta.0;
                        self.input.mouse.delta_y = delta.1;
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
                            .set_viewport(physical_size.width as i32, physical_size.height as i32);
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
                Event::MainEventsCleared => {
                    // microseconds left before target time, before we should spin
                    // poor timings on Windows means this should be >= 1000
                    let spin_threshold = 1000;

                    let now = time::Instant::now();
                    if now >= self.context.target_time {
                        self.context.target_time = now + self.context.target_frametime;
                        self.context.window_context.window().request_redraw();
                    }

                    let now = time::Instant::now();
                    let delta = self.context.target_time - now;
                    if delta < time::Duration::from_micros(spin_threshold) {
                        return;
                    }

                    let mut sleep_time = time::Duration::from_micros(
                        (delta.as_micros() - (delta.as_micros() % spin_threshold as u128)) as u64,
                    );

                    if delta > time::Duration::from_micros((1000000.0 / 144.0) as u64)
                        && (self.context.being_moved || self.context.being_resized)
                    {
                        sleep_time = time::Duration::from_micros(
                            ((1000000.0 / 144.0) - ((1000000.0 / 144.0) % spin_threshold as f64))
                                as u64,
                        );
                    }

                    spin_sleep::sleep(sleep_time);
                }
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
