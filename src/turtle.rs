use crate::graphics::{Vertex, VertexPosition};
use crate::lsys::{ActualParam, Element, LString, ParamList};
extern crate nalgebra_glm as glm;
use glm::Vec3;
use num_traits::identities::Zero;

#[derive(Debug)]
pub struct Turtle {
    position: Vec3,
    heading: Vec3,
    up: Vec3,
    right: Vec3,
    model: Vec<Vertex>,
}

impl Default for Turtle {
    fn default() -> Self {
        Self::new()
    }
}

impl Turtle {
    pub fn new() -> Self {
        Turtle {
            position: Vec3::zero(),
            heading: Vec3::y(),
            up: Vec3::z(),
            right: Vec3::x(),
            model: Vec::new(),
        }
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

    pub fn draw(&mut self, distance: f32) {
        self.push_position();
        self.mov(distance);
        self.push_position();
    }

    fn push_position(&mut self) {
        self.model.push(Vertex {
            pos: VertexPosition::new([self.position.x, self.position.y, self.position.z]),
        });
    }

    pub fn interpret(&mut self, lstring: &LString) -> &Vec<Vertex> {
        for element in lstring {
            self.interpret_element(element);
        }
        &self.model
    }

    fn interpret_element(&mut self, element: &Element<ActualParam>) {
        match (element.symbol, &element.params) {
            ('F', ParamList::Empty) => self.draw(0.01),
            ('F', ParamList::A1([x])) => self.draw(*x),
            ('+', ParamList::Empty) => self.turn(90.0),
            ('+', ParamList::A1([x])) => self.turn(*x),
            ('-', ParamList::Empty) => self.turn(-90.0),
            ('-', ParamList::A1([x])) => self.turn(-*x),
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
        turtle.mov(0.5);
        println!("{:?}", turtle);
        assert_relative_eq!(turtle.position, Vec3::new(0.0, 0.5, 0.0));
    }

    #[test]
    fn test_turn() {
        let mut turtle = Turtle::new();
        turtle.turn(90.0);
        turtle.mov(0.5);
        println!("{:?}", turtle);
        assert_relative_eq!(turtle.position, Vec3::new(0.5, 0.0, 0.0));
    }

    #[test]
    fn test_pitch() {
        let mut turtle = Turtle::new();
        turtle.pitch(90.0);
        turtle.mov(0.5);
        println!("{:?}", turtle);
        assert_relative_eq!(turtle.position, Vec3::new(0.0, 0.0, 0.5));
    }

    #[test]
    fn test_roll() {
        let mut turtle = Turtle::new();
        turtle.roll(90.0);
        turtle.mov(0.5);
        println!("{:?}", turtle);
        assert_relative_eq!(turtle.position, Vec3::new(0.0, 0.5, 0.0));
    }

    #[test]
    fn test_draw() {
        let mut turtle = Turtle::new();
        turtle.draw(0.5);
        println!("{:?}", turtle);
        assert_relative_eq!(turtle.position, Vec3::new(0.0, 0.5, 0.0));
        assert_eq!(turtle.model.len(), 2);
    }
}
