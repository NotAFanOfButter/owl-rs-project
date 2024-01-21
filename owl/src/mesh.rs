use crate::{prelude::*, ox, VertexArray, Program, OwlError};

pub struct Mesh<'a,E: ToByteVec> {
    pub start: usize,
    pub count: usize,
    pub vertex_array: &'a VertexArray<E>
}

pub use ox::DrawMode;
impl<'a, E: ToByteVec> Mesh<'a, E> {
    pub fn draw(&self, mode: DrawMode, shader_program: &Program) -> Result<(),OwlError> {
        shader_program.use_self().map_err(|e| e.with_context("drawing mesh"))?;
        self.vertex_array.bind();
        match self.vertex_array.elements {
            Some(ref e) => {
                ox::draw_elements(mode, self.count, e.inner_type, self.start)
                    .with_message("drawing mesh, element draw failed")
            },
            None => {
                ox::draw_arrays(mode, self.start, self.count)
                    .with_message("drawing mesh, array draw failed")
            },
        }
    }
}
