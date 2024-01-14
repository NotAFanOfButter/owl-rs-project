use crate::ArrayBuffer;
use crate::ElementBuffer;
use crate::OwlError;
use crate::ToByteVec;
use crate::oxidised_bindings as ox;

pub use ox::{ VertexFormat, DataTypeSize3, DataTypeSize4, DataTypeSizeBgra, DataTypeUnsized };

pub enum AttributeType {
    Bool,
    Int,
    Float,
    Vec2,
    Vec3,
    Vec4,
    BVec2,
    BVec3,
    BVec4,
    IVec2,
    IVec3,
    IVec4,
    Mat2,
    Mat3,
    Mat4
}
/// Generic attribute with a name and glsl type,
/// used as part of an input, uniform, or pipe
pub struct Attribute {
    pub name: String,
    pub glsl_type: AttributeType,
}

type Bytes = usize;
pub struct AttributePointer<'a, T: ToByteVec> {
    pub buffer: &'a ArrayBuffer<T>,
    pub stride: Bytes,
    pub offset: Bytes,
    pub format: VertexFormat,
}

/// An input to the shader pipeline, stored in a [VertexArray].
pub struct Input {
    index: u8,
    attribute: Attribute,
}
impl Input {
    fn new<T: ToByteVec>(index: u8, attribute: Attribute, AttributePointer { buffer, stride, offset, format }: AttributePointer<T>) -> Self {
        buffer.bind();
        // buffer bound & index checked: shouldn't ever fail
        ox::vertex_attrib_pointer(index, format, stride, offset)
            .expect("somehow: failed to create input");
        Self { index, attribute }
    }
}

pub struct VertexArray<T: ToByteVec> {
    inner: ox::VertexArray,
    max_inputs: u8,
    inputs: Vec<Input>,
    elements: Option<ElementBuffer<T>>,
}
impl<T: ToByteVec> VertexArray<T> {
    // INVARIANT: will not be deleted until it is dropped
    // fewer calls can fail, reducing error handling, but they now "expect"
    pub fn new() -> Self {
        Self {
            inner: ox::gen_vertex_array(),
            max_inputs: ox::get_uint(ox::UIntParameter::MaxVertexAttribs) as u8,
            inputs: Vec::new(),
            elements: None
        }
    }
    pub fn add_input(&mut self, attribute: Attribute, pointer: AttributePointer<T>) -> Result<(),OwlError> {
        // max inputs should never be allowed to be >= 256
        let next_index = self.inputs.len() as u8;
        if next_index > self.max_inputs {
            self.bind();
            self.inputs.push(Input::new(next_index, attribute, pointer));
            Ok(())
        } else {
            Err(OwlError::custom("maximum inputs reached"))
        }
    }
    fn bind(&self) {
        ox::bind_vertex_array(Some(self.inner))
            .expect("vertex array should not be deleted yet")
    }
    fn unbind(&self) {
        ox::bind_vertex_array(None)
            .expect("binding 0 always succeeds")
    }
}
impl<T: ToByteVec> Drop for VertexArray<T> {
    fn drop(&mut self) {
        ox::delete_vertex_array(self.inner)
    }
}
