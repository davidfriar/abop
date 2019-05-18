
#[derive (Debug)]
pub struct LSystem<'a>{
    pub current:&'a str
}

impl<'a> LSystem<'a> {

    pub fn new(axiom:&'a str)->Self{
        LSystem{current:axiom}
    }

    pub fn generate(&self)->(){
    }
}
