extern crate luminance;
extern crate luminance_derive;
extern crate luminance_glfw;

use luminance::context::GraphicsContext;
use luminance::framebuffer::Framebuffer;
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::{Mode, TessBuilder};
use luminance_derive::{Semantics, Vertex};
use luminance_glfw::event::{Action, Key, WindowEvent};
use luminance_glfw::surface::{GlfwSurface, Surface, WindowDim, WindowOpt};

const VS: &str = include_str!("vs.glsl");
const FS: &str = include_str!("fs.glsl");

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantics {
    // reference vertex positions with the co variable in vertex shaders
    #[sem(name = "co", repr = "[f32; 3]", type_name = "VertexPosition")]
    Position,
}

#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    pub pos: VertexPosition,
}

pub fn run(model: &Vec<Vertex>) {
    let mut surface = GlfwSurface::new(
        WindowDim::Windowed(960, 540),
        "LSystem",
        WindowOpt::default(),
    )
    .expect("GLFW surface creation");

    let (program, _) =
        Program::<Semantics, (), ()>::from_strings(None, VS, None, FS).expect("program creation");

    let mut back_buffer = Framebuffer::back_buffer(surface.size());

    let model = TessBuilder::new(&mut surface)
        .add_vertices(model)
        .set_mode(Mode::Line)
        .build()
        .unwrap();

    'app: loop {
        for event in surface.poll_events() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'app
                }
                WindowEvent::FramebufferSize(width, height) => {
                    back_buffer = Framebuffer::back_buffer([width as u32, height as u32]);
                }
                _ => (),
            }
        }

        surface
            .pipeline_builder()
            .pipeline(&back_buffer, [0., 0., 0., 0.], |_, shd_gate| {
                shd_gate.shade(&program, |rdr_gate, _| {
                    rdr_gate.render(RenderState::default(), |tess_gate| {
                        tess_gate.render(&mut surface, (&model).into());
                    });
                });
            });

        surface.swap_buffers();
    }
}
