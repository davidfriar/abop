use crate::config::get_config;
use crate::graphics::{Vertex, VertexColor, VertexNormal, VertexPosition};
use crate::lsys::{ActualParam, Element, LString};
extern crate nalgebra_glm as glm;
use glm::{Mat4, Vec3, Vec4};

lazy_static! {
    static ref DEFAULT_DISTANCE: f32 = get_config("turtle.default.distance");
    static ref DEFAULT_SIZE: f32 = get_config("turtle.default.size");
    static ref DEFAULT_ANGLE: f32 = get_config("turtle.default.angle");
    static ref DEFAULT_COLOR: [f32; 3] = get_config("turtle.default.color");
    static ref DEFAULT_ROTATION_STEP: i8 = get_config("turtle.default.rotation.step");
    static ref DEFAULT_SHAPE_SEGMENTS: i8 = get_config("turtle.default.shape.segments");
}

#[derive(Debug)]
pub struct Turtle {
    state: TurtleState,
    stack: Vec<TurtleState>,
}

#[derive(Debug, Clone)]
pub struct TurtleState {
    transform: Mat4,
    color: Vec3,
    size: Option<f32>,
    shape: Vec<ShapeVertex>,
}

#[derive(Debug, Clone)]
struct ShapeVertex {
    pos: Vec4,
    normal: Vec4,
}

type DrawingOutput = Option<Vec<Vertex>>;

impl Turtle {
    pub fn new() -> Self {
        Turtle {
            state: TurtleState::new(),
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, lstring: &LString) -> Vec<Vertex> {
        lstring
            .into_iter()
            .flat_map(|element| self.interpret_element(element).unwrap_or(Vec::new()))
            .collect()
    }

    fn interpret_element(&mut self, element: &Element<ActualParam>) -> DrawingOutput {
        match (element.symbol, element.params.values()) {
            ('F', []) => self.state.draw(*DEFAULT_DISTANCE, None),
            ('F', [x]) => self.state.draw(*x, None),
            ('F', [x, y]) => self.state.draw(*x, Some(*y)),
            ('f', []) => self.state.mov(*DEFAULT_DISTANCE),
            ('f', [x]) => self.state.mov(*x),
            ('+', []) => self.state.turn(*DEFAULT_ANGLE),
            ('+', [x]) => self.state.turn(*x),
            ('-', []) => self.state.turn(-*DEFAULT_ANGLE),
            ('-', [x]) => self.state.turn(-*x),
            ('/', []) => self.state.roll(*DEFAULT_ANGLE),
            ('/', [x]) => self.state.roll(*x),
            ('\\', []) => self.state.roll(-*DEFAULT_ANGLE),
            ('\\', [x]) => self.state.roll(-*x),
            ('^', []) => self.state.pitch(*DEFAULT_ANGLE),
            ('^', [x]) => self.state.pitch(*x),
            ('&', []) => self.state.pitch(-*DEFAULT_ANGLE),
            ('&', [x]) => self.state.pitch(-*x),
            ('`', [x, y, z]) => self.state.color(*x, *y, *z),
            ('[', []) => {
                self.push_state();
                None
            }
            (']', []) => {
                self.pop_state();
                None
            }
            _ => None,
        }
    }

    fn push_state(&mut self) {
        self.stack.push(self.state.clone());
    }

    fn pop_state(&mut self) {
        if let Some(state) = self.stack.pop() {
            self.state = state;
        }
    }
}

impl TurtleState {
    fn new() -> Self {
        TurtleState {
            transform: glm::identity(),
            color: glm::make_vec3(&*DEFAULT_COLOR),
            size: None,
            shape: Self::default_shape(),
        }
    }

    fn default_shape() -> Vec<ShapeVertex> {
        let n = *DEFAULT_SHAPE_SEGMENTS;
        let x = Vec3::x();
        let y = Vec3::y();
        (0..n)
            .map(|i| glm::rotate_vec3(&x, (i as f32 * 360.0 / n as f32).to_radians(), &y))
            .map(|v| ShapeVertex {
                pos: Vec4::new(v.x, v.y, v.z, 1.0),
                normal: Vec4::new(v.x, v.y, v.z, 1.0),
            })
            .collect()
    }
    fn position(&self) -> Vec3 {
        glm::vec4_to_vec3(&glm::column(&self.transform, 3))
    }

    fn right(&self) -> Vec3 {
        glm::vec4_to_vec3(&glm::column(&self.transform, 0))
    }

    fn heading(&self) -> Vec3 {
        glm::vec4_to_vec3(&glm::column(&self.transform, 1))
    }

    fn up(&self) -> Vec3 {
        glm::vec4_to_vec3(&glm::column(&self.transform, 2))
    }

    pub fn mov(&mut self, distance: f32) -> DrawingOutput {
        self.transform = glm::translation(&(self.heading() * distance)) * self.transform;
        None
    }

    pub fn turn(&mut self, angle: f32) -> DrawingOutput {
        self.rot(-angle, &self.up())
    }

    pub fn pitch(&mut self, angle: f32) -> DrawingOutput {
        self.rot(angle, &self.right())
    }

    pub fn roll(&mut self, angle: f32) -> DrawingOutput {
        self.rot(angle, &self.heading())
    }

    fn draw(&mut self, distance: f32, new_size: Option<f32>) -> DrawingOutput {
        if self.size.is_none() {
            self.size = new_size;
        }
        let shape1 = self.transformed_shape();
        self.mov(distance);
        if new_size.is_some() {
            self.size = new_size;
        }
        let shape2 = self.transformed_shape();
        Some(self.tube(shape1, shape2))
    }

    fn rot(&mut self, angle: f32, axis: &Vec3) -> DrawingOutput {
        let pos = &self.position();
        let to_origin = glm::translation(&(Vec3::zeros() - pos));
        let to_pos = glm::translation(pos);

        let steps = (angle as i8 / *DEFAULT_ROTATION_STEP).abs();
        let rotation = glm::rotation((angle / steps as f32).to_radians(), axis);
        Some(
            (0..steps)
                .flat_map(|_| {
                    let shape1 = self.transformed_shape();
                    self.transform = to_pos * rotation * to_origin * self.transform;
                    let shape2 = self.transformed_shape();
                    self.tube(shape1, shape2)
                })
                .collect(),
        )
    }

    pub fn color(&mut self, r: f32, g: f32, b: f32) -> DrawingOutput {
        self.color[0] = r;
        self.color[1] = g;
        self.color[2] = b;
        None
    }

    fn transformed_shape(&mut self) -> Vec<ShapeVertex> {
        let s = *self.size.get_or_insert(*DEFAULT_SIZE);
        let scaling = glm::scaling(&Vec3::new(s, s, s));
        self.shape
            .iter()
            .map(|ShapeVertex { pos, normal }| ShapeVertex {
                pos: self.transform * scaling * pos,
                normal: self.transform * normal,
            })
            .collect()
    }

    fn tube(&self, shape1: Vec<ShapeVertex>, shape2: Vec<ShapeVertex>) -> Vec<Vertex> {
        let n = shape1.len();
        (0..n)
            .flat_map(|i| {
                let next = (i + 1) % n;
                let mut t = self.triangle(&shape2[i], &shape1[i], &shape2[next]);
                t.append(&mut self.triangle(&shape2[next], &shape1[i], &shape1[next]));
                t
            })
            .collect()
    }

    fn triangle(&self, v1: &ShapeVertex, v2: &ShapeVertex, v3: &ShapeVertex) -> Vec<Vertex> {
        let color = self.color;
        vec![
            self.vertex(v1, color),
            self.vertex(v2, color),
            self.vertex(v3, color),
        ]
    }

    fn vertex(&self, v: &ShapeVertex, color: Vec3) -> Vertex {
        Vertex {
            pos: VertexPosition::new([v.pos.x, v.pos.y, v.pos.z]),
            col: VertexColor::new([color.x, color.y, color.z]),
            norm: VertexNormal::new([v.normal.x, v.normal.y, v.normal.z]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_mov() {
        let mut turtle = Turtle::new();
        turtle.state.mov(0.5);
        println!("{:?}", turtle);
        assert_relative_eq!(turtle.state.position(), Vec3::new(0.0, 0.5, 0.0));
    }

    #[test]
    fn test_turn() {
        let mut turtle = Turtle::new();
        turtle.state.turn(90.0);
        turtle.state.mov(0.5);
        assert_relative_eq!(
            turtle.state.position(),
            Vec3::new(0.5, 0.0, 0.0),
            epsilon = std::f32::EPSILON * 2.0
        );
    }

    #[test]
    fn test_move_then_turn() {
        let mut turtle = Turtle::new();
        println!(
            "start {:?} position:{}, heading:{}, up:{}, right:{}",
            turtle.state.transform.data,
            turtle.state.position(),
            turtle.state.heading(),
            turtle.state.up(),
            turtle.state.right()
        );
        turtle.state.mov(1.0);
        println!(
            "after first move {:?} position:{}, heading:{}, up:{}, right:{}",
            turtle.state.transform.data,
            turtle.state.position(),
            turtle.state.heading(),
            turtle.state.up(),
            turtle.state.right()
        );
        turtle.state.turn(90.0);
        println!(
            "after turn {:?} position:{}, heading:{}, up:{}, right:{}",
            turtle.state.transform.data,
            turtle.state.position(),
            turtle.state.heading(),
            turtle.state.up(),
            turtle.state.right()
        );
        turtle.state.mov(0.5);
        println!(
            "after second move {:?} position:{}, heading:{}, up:{}, right:{}",
            turtle.state.transform.data,
            turtle.state.position(),
            turtle.state.heading(),
            turtle.state.up(),
            turtle.state.right()
        );
        assert_relative_eq!(
            turtle.state.position(),
            Vec3::new(0.5, 1.0, 0.0),
            epsilon = std::f32::EPSILON * 2.0
        );
    }

    #[test]
    fn test_pitch() {
        let mut turtle = Turtle::new();
        turtle.state.pitch(90.0);
        turtle.state.mov(0.5);
        println!("{:?}", turtle);
        assert_relative_eq!(
            turtle.state.position(),
            Vec3::new(0.0, 0.0, 0.5),
            epsilon = std::f32::EPSILON * 2.0
        );
    }

    #[test]
    fn test_roll() {
        let mut turtle = Turtle::new();
        turtle.state.roll(90.0);
        turtle.state.mov(0.5);
        println!("{:?}", turtle);
        assert_relative_eq!(turtle.state.position(), Vec3::new(0.0, 0.5, 0.0));
    }

    #[test]
    fn test_draw() {
        let mut turtle = Turtle::new();
        turtle.state.draw(0.5, None);
        println!("{:?}", turtle);
        assert_relative_eq!(turtle.state.position(), Vec3::new(0.0, 0.5, 0.0));
    }
}
