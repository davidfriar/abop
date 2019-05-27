use crate::graphics::{Vertex, VertexPosition};
use crate::lsys::LString;

pub struct Turtle {
    _position: VertexPosition,
    _direction: VertexPosition,
}

impl Default for Turtle {
    fn default() -> Self {
        Self::new()
    }
}

impl Turtle {
    pub fn new() -> Self {
        Turtle {
            _position: VertexPosition::new([0.0, 0.0, 0.0]),
            _direction: VertexPosition::new([0.0, 1.0, 0.0]),
        }
    }

    pub fn interpret(&mut self, _commands: &LString) -> Vec<Vertex> {
        let mut result = Vec::new();
        result.push(Vertex {
            pos: VertexPosition::new([0.0, 0.0, 0.0]),
        });
        result.push(Vertex {
            pos: VertexPosition::new([0.0, 1.0, 0.0]),
        });
        result
    }
}
