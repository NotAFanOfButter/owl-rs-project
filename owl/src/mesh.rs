use crate::{prelude::*, ox, VertexArray, Program, OwlError};

pub struct Mesh<'a,E: ToByteVec> {
    pub start: usize,
    pub count: usize,
    pub vertex_array: &'a VertexArray<E>
}

pub use ox::DrawMode;
impl<'a, E: ToByteVec> Mesh<'a, E> {
    /// # Errors
    ///
    /// This function will return an error if any buffers used for data are being mapped,
    /// or an incompatible geometry shader is used within `shader_program`.
    pub fn draw(&self, mode: DrawMode, shader_program: &Program) -> Result<(),OwlError> {
        shader_program.use_self().with_context("drawing mesh")?;
        self.vertex_array.bind();
        match self.vertex_array.elements {
            Some(ref e) => {
                ox::draw_elements(mode, self.count, e.inner_type, self.start).map_err(|e|
                    match e {
                        ox::OxError::BaseError(crate::OriginalError::InvalidOperation) => 
                            e.with_message("either one of the buffers used is being mapped, or
                                the geometry shader's input primitive is incompatible with the
                                draw mode"),
                        _ => e.with_message("no other errors should be produced")
                    }
                    .with_context("drawing mesh, element draw failed")
                )
            },
            None => {
                ox::draw_arrays(mode, self.start, self.count).map_err(|e|
                    match e {
                        ox::OxError::BaseError(crate::OriginalError::InvalidOperation) => 
                            e.with_message("either one of the buffers used is being mapped, or
                                the geometry shader's input primitive is incompatible with the
                                draw mode"),
                        _ => e.with_message("no other errors should be produced")
                    }
                    .with_context("drawing mesh, element draw failed")
                )
            },
        }
    }
}
