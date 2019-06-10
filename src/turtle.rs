use crate::graphics::{Vertex, VertexColor, VertexNormal, VertexPosition};
use crate::lsys::{ActualParam, Element, LString};
extern crate nalgebra_glm as glm;
use glm::Vec3;
use num_traits::identities::Zero;

#[derive(Debug)]
pub struct Turtle {
    state: TurtleState,
    model: Vec<Vertex>,
    stack: Vec<TurtleState>,
}

#[derive(Debug, Clone)]
pub struct TurtleState {
    position: Vec3,
    heading: Vec3,
    up: Vec3,
    right: Vec3,
    color: Vec3,
    size: Option<f32>,
    shape: Vec<Vec3>,
}

impl TurtleState {
    const DEFAULT_SIZE: f32 = 0.01;

    fn new() -> Self {
        TurtleState {
            position: Vec3::zero(),
            color: Vec3::new(0.6, 0.6, 0.0),
            size: None,
            shape: Self::default_shape(),
            heading: Vec3::y(),
            up: Vec3::z(),
            right: Vec3::x(),
        }
    }

    fn default_shape() -> Vec<Vec3> {
        let n = 12;
        let x = Vec3::x();
        let y = Vec3::y();
        (0..n)
            .map(|i| glm::rotate_vec3(&x, (i as f32 * 360.0 / n as f32).to_radians(), &y))
            .collect()
    }

    pub fn mov(&mut self, distance: f32) {
        self.position += self.heading * distance;
    }

    pub fn turn(&mut self, angle: f32) {
        self.heading = glm::rotate_vec3(&self.heading, -angle.to_radians(), &self.up);
        self.right = glm::rotate_vec3(&self.right, -angle.to_radians(), &self.up);
    }

    pub fn pitch(&mut self, angle: f32) {
        self.heading = glm::rotate_vec3(&self.heading, angle.to_radians(), &self.right);
        self.up = glm::rotate_vec3(&self.up, angle.to_radians(), &self.right);
    }

    pub fn roll(&mut self, angle: f32) {
        self.right = glm::rotate_vec3(&self.right, angle.to_radians(), &self.heading);
        self.up = glm::rotate_vec3(&self.up, angle.to_radians(), &self.heading);
    }
    pub fn color(&mut self, r: f32, g: f32, b: f32) {
        self.color[0] = r;
        self.color[1] = g;
        self.color[2] = b;
    }
}

impl Default for Turtle {
    fn default() -> Self {
        Self::new()
    }
}

impl Turtle {
    pub fn new() -> Self {
        Turtle {
            state: TurtleState::new(),
            model: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn push_state(&mut self) {
        self.stack.push(self.state.clone());
    }

    pub fn pop_state(&mut self) {
        match self.stack.pop() {
            Some(state) => {
                self.state = state;
            }
            _ => (),
        }
    }

    pub fn draw(&mut self, distance: f32, new_size: Option<f32>) {
        let size1 = self
            .state
            .size
            .unwrap_or(new_size.unwrap_or(TurtleState::DEFAULT_SIZE));
        let size2 = new_size.unwrap_or(size1);

        let shape: Vec<Vec3> = self
            .state
            .shape
            .iter()
            .map(|v| {
                Vec3::new(
                    glm::dot(&v, &self.state.right),
                    -glm::dot(&v, &self.state.heading),
                    glm::dot(&v, &self.state.up),
                )
            })
            .collect();
        let shape1: Vec<Vec3> = shape
            .iter()
            .map(|v| v * size1 + self.state.position)
            .collect();
        let shape2: Vec<Vec3> = shape
            .iter()
            .map(|v| v * size2 + self.state.position + self.state.heading * distance)
            .collect();

        for i in 0..shape.len() {
            let next = (i + 1) % shape.len();
            self.output_triangle(shape2[i], shape1[i], shape2[next]);
            self.output_triangle(shape2[next], shape1[i], shape1[next]);
        }
        self.state.size = Some(size2);
        self.state.mov(distance);
    }

    fn output_triangle(&mut self, v1: Vec3, v2: Vec3, v3: Vec3) {
        let normal = glm::normalize(&((v2 - v1).cross(&(v3 - v1))));
        let color = self.state.color;
        self.output_vertex(v1, normal, color);
        self.output_vertex(v2, normal, color);
        self.output_vertex(v3, normal, color);
    }

    fn output_vertex(&mut self, v: Vec3, normal: Vec3, color: Vec3) {
        self.model.push(Vertex {
            pos: VertexPosition::new([v.x, v.y, v.z]),
            col: VertexColor::new([color.x, color.y, color.z]),
            norm: VertexNormal::new([normal.x, normal.y, normal.z]),
        });
    }

    pub fn interpret(&mut self, lstring: &LString) -> &Vec<Vertex> {
        for element in lstring {
            self.interpret_element(element);
        }
        &self.model
    }

    fn interpret_element(&mut self, element: &Element<ActualParam>) {
        match (element.symbol, element.params.values()) {
            ('F', []) => self.draw(0.1, None),
            ('F', [x]) => self.draw(*x, None),
            ('F', [x, y]) => self.draw(*x, Some(*y)),
            ('+', []) => self.state.turn(90.0),
            ('+', [x]) => self.state.turn(*x),
            ('-', []) => self.state.turn(-90.0),
            ('-', [x]) => self.state.turn(-*x),
            ('/', []) => self.state.roll(90.0),
            ('/', [x]) => self.state.roll(*x),
            ('\\', []) => self.state.roll(-90.0),
            ('\\', [x]) => self.state.roll(-*x),
            ('^', []) => self.state.pitch(90.0),
            ('^', [x]) => self.state.pitch(*x),
            ('&', []) => self.state.pitch(-90.0),
            ('&', [x]) => self.state.pitch(-*x),
            ('[', []) => self.push_state(),
            (']', []) => self.pop_state(),
            ('`', [x, y, z]) => self.state.color(*x, *y, *z),
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mov() {
        let mut turtle = Turtle::new();
        turtle.state.mov(0.5);
        println!("{:?}", turtle);
        assert_relative_eq!(turtle.state.position, Vec3::new(0.0, 0.5, 0.0));
    }

    #[test]
    fn test_turn() {
        let mut turtle = Turtle::new();
        turtle.state.turn(90.0);
        turtle.state.mov(0.5);
        println!("{:?}", turtle);
        assert_relative_eq!(turtle.state.position, Vec3::new(0.5, 0.0, 0.0));
    }

    #[test]
    fn test_pitch() {
        let mut turtle = Turtle::new();
        turtle.state.pitch(90.0);
        turtle.state.mov(0.5);
        println!("{:?}", turtle);
        assert_relative_eq!(turtle.state.position, Vec3::new(0.0, 0.0, 0.5));
    }

    #[test]
    fn test_roll() {
        let mut turtle = Turtle::new();
        turtle.state.roll(90.0);
        turtle.state.mov(0.5);
        println!("{:?}", turtle);
        assert_relative_eq!(turtle.state.position, Vec3::new(0.0, 0.5, 0.0));
    }

    #[test]
    fn test_draw() {
        let mut turtle = Turtle::new();
        turtle.draw(0.5, None);
        println!("{:?}", turtle);
        assert_relative_eq!(turtle.state.position, Vec3::new(0.0, 0.5, 0.0));
        assert_eq!(turtle.model.len(), 2);
    }
}
