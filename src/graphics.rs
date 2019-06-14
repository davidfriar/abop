extern crate luminance;
extern crate luminance_derive;
extern crate luminance_glfw;

extern crate nalgebra_glm as glm;
use glm::Vec3;
use luminance::context::GraphicsContext;
use luminance::framebuffer::Framebuffer;
use luminance::linear::{M33, M44};
use luminance::render_state::RenderState;
use luminance::shader::program::{Program, Uniform};
use luminance::tess::{Mode, Tess, TessBuilder};
use luminance::texture::{Dim2, Flat};
use luminance_derive::UniformInterface;
use luminance_derive::{Semantics, Vertex};
use luminance_glfw::event::{Action, Key, WindowEvent};
use luminance_glfw::surface::{GlfwSurface, Surface, WindowDim, WindowOpt};
use std::time::Instant;

const VS: &str = include_str!("vs.glsl");
const FS: &str = include_str!("fs.glsl");

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantics {
    #[sem(name = "pos", repr = "[f32; 3]", type_name = "VertexPosition")]
    Position,
    #[sem(name = "col", repr = "[f32; 3]", type_name = "VertexColor")]
    Color,
    #[sem(name = "norm", repr = "[f32; 3]", type_name = "VertexNormal")]
    Normal,
}

#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    pub pos: VertexPosition,
    pub col: VertexColor,
    pub norm: VertexNormal,
}

#[derive(Debug, UniformInterface)]
struct ShaderInterface {
    model_transform: Uniform<M44>,
    normal_transform: Uniform<M33>,
    mvp_transform: Uniform<M44>,
    view_pos: Uniform<[f32; 3]>,
}

struct Timer {
    instant: Instant,
}

struct Camera {
    position: Vec3,
    front: Vec3,
    up: Vec3,
}
pub struct Application {
    surface: GlfwSurface,
    program: Program<Semantics, (), ShaderInterface>,
    back_buffer: Framebuffer<Flat, Dim2, (), ()>,
    projection: glm::Mat4,
    model: glm::Mat4,
    camera: Camera,
    timer: Timer,
}

impl Application {
    const DEFAULT_WIDTH: u32 = 960;
    const DEFAULT_HEIGHT: u32 = 540;
    const ROTATION_SPEED: f32 = 0.005;

    pub fn new() -> Self {
        let surface = GlfwSurface::new(
            WindowDim::Windowed(Self::DEFAULT_WIDTH, Self::DEFAULT_HEIGHT),
            "LSystem",
            WindowOpt::default(),
        )
        .expect("GLFW surface creation failed");

        let (program, _) =
            Program::<Semantics, (), ShaderInterface>::from_strings(None, VS, None, FS)
                .expect("Shader program creation failed");

        let back_buffer = Framebuffer::back_buffer(surface.size());

        Application {
            surface,
            program,
            back_buffer,
            projection: glm::identity(),
            model: glm::identity(),
            camera: Camera::new(),
            timer: Timer::new(),
        }
    }

    pub fn run(&mut self, obj: &Vec<Vertex>) {
        let tess = self.load_object(obj);

        while self.handle_input() {
            let t = self.timer.elapsed();
            self.model = glm::rotate(
                &self.model,
                (t as f32 * Self::ROTATION_SPEED).to_radians(),
                &Vec3::new(0.0, 1.0, 0.0),
            );

            self.render(&tess);
        }
    }

    fn load_object(&mut self, obj: &Vec<Vertex>) -> Tess {
        TessBuilder::new(&mut self.surface)
            .add_vertices(obj)
            .set_mode(Mode::Triangle)
            // .set_mode(Mode::Line)
            .build()
            .unwrap()
    }

    fn handle_input(&mut self) -> bool {
        let press = |action| action == Action::Press || action == Action::Repeat;
        let events: Vec<WindowEvent> = self.surface.poll_events().collect();

        for event in events {
            match event {
                WindowEvent::Key(key, _, action, _) if press(action) => match key {
                    Key::W => self.camera.move_forward(),
                    Key::S => self.camera.move_back(),
                    Key::D => self.camera.turn_right(),
                    Key::A => self.camera.turn_left(),
                    _ => (),
                },
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    return false
                }
                WindowEvent::FramebufferSize(width, height) => {
                    self.resize_window(width, height);
                }
                _ => (),
            }
        }
        true
    }

    fn resize_window(&mut self, width: i32, height: i32) {
        self.back_buffer = Framebuffer::back_buffer([width as u32, height as u32]);
        self.projection =
            glm::perspective(45f32.to_radians(), width as f32 / height as f32, 0.1, 100.0);
    }

    fn render(&mut self, obj: &Tess) {
        let surface = &mut self.surface;
        let program = &self.program;
        let mvp_transform: glm::Mat4 = self.projection * self.camera.view() * self.model;
        let model = self.model.clone();
        let view_pos = self.camera.position.clone();
        let normal_transform = glm::mat4_to_mat3(&glm::transpose(&glm::inverse(&self.model)));

        surface
            .pipeline_builder()
            .pipeline(&self.back_buffer, [0., 0., 0., 0.], |_, shd_gate| {
                shd_gate.shade(program, |rdr_gate, iface| {
                    iface.mvp_transform.update(mvp_transform.into());
                    iface.model_transform.update(model.into());
                    iface.normal_transform.update(normal_transform.into());
                    iface.view_pos.update(view_pos.into());
                    rdr_gate.render(RenderState::default(), |tess_gate| {
                        tess_gate.render(surface, obj.into());
                    });
                });
            });

        surface.swap_buffers();
    }
}

impl Camera {
    const SPEED: f32 = 0.1;
    fn new() -> Self {
        Camera {
            position: Vec3::new(0.0, 0.0, 3.0),
            front: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
        }
    }

    fn view(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    fn move_forward(&mut self) {
        self.position += self.front * Self::SPEED;
    }

    fn move_back(&mut self) {
        self.position -= self.front * Self::SPEED;
    }
    fn turn_right(&mut self) {
        self.front = glm::rotate_vec3(&self.front, -5.0f32.to_radians(), &self.up);
    }
    fn turn_left(&mut self) {
        self.front = glm::rotate_vec3(&self.front, 5.0f32.to_radians(), &self.up);
    }
}

impl Timer {
    fn new() -> Self {
        Timer {
            instant: Instant::now(),
        }
    }

    fn elapsed(&mut self) -> u32 {
        let t = self.instant.elapsed().as_millis() as u32;
        self.instant = Instant::now();
        t
    }
}
