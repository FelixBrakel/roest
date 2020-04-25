use gl_renderer::Program;
// use specs::{Component, VecStorage};

pub struct IMeshRenderer {
    program: Program,
}

#[allow(dead_code)]
impl IMeshRenderer {
    pub fn new(program: Program) -> IMeshRenderer {
        IMeshRenderer { program }
    }

    pub fn get_program(&self) -> &Program {
        &self.program
    }
}

// impl Component for IMeshRenderer {
//     type Storage = VecStorage<IMeshRenderer>;
// }