use crate::graphics::{Vertex,VertexPosition};



pub struct Turtle{
 position:VertexPosition,
 direction:VertexPosition
}


impl Turtle{

    pub fn new()->Self{
        Turtle{
            position: VertexPosition::new([0.0,0.0,0.0]),
            direction:VertexPosition::new([0.0,1.0,0.0])
        }
    }

    pub fn interpret(&mut self, _commands:&str)->Vec<Vertex>{
        let mut result=Vec::new();
        result.push(Vertex{pos:VertexPosition::new([0.0,0.0,0.0])});
        result.push(Vertex{pos:VertexPosition::new([0.0,1.0,0.0])});
        result
    }
}
