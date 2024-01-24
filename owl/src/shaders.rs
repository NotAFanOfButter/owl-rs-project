use std::ffi::CString;

use crate::{IntegralVertexFormat, FloatVertexFormat, AttributePointer, OwlError, VertexArray};
use crate::prelude::*;
use crate::ox;

/// Corresponds to a glsl type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    UVec2,
    UVec3,
    UVec4,
    Mat2,
    Mat3,
    Mat4
}
impl std::fmt::Display for AttributeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Bool => "bool",
            Self::Int => "int",
            Self::Float => "float",
            Self::Vec2 => "vec2",
            Self::Vec3 => "vec3",
            Self::Vec4 => "vec4",
            Self::BVec2 => "bvec2",
            Self::BVec3 => "bvec3",
            Self::BVec4 => "bvec4",
            Self::IVec2 => "ivec2",
            Self::IVec3 => "ivec3",
            Self::IVec4 => "ivec4",
            Self::UVec2 => "uvec2",
            Self::UVec3 => "uvec3",
            Self::UVec4 => "uvec4",
            Self::Mat2 => "mat2",
            Self::Mat3 => "mat3",
            Self::Mat4 => "mat4",
        };    
        write!(f, "{s}")
    }
}

/// Corresponds to a floating point (at least natively) glsl type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FloatAttributeType {
    Bool,
    Float,
    Vec2,
    Vec3,
    Vec4,
    BVec2,
    BVec3,
    BVec4,
    Mat2,
    Mat3,
    Mat4
}
impl From<FloatAttributeType> for AttributeType {
    fn from(value: FloatAttributeType) -> Self {
        match value {
            FloatAttributeType::Bool => Self::Bool,
            FloatAttributeType::Float => Self::Float,
            FloatAttributeType::Vec2 => Self::Vec2,
            FloatAttributeType::Vec3 => Self::Vec3,
            FloatAttributeType::Vec4 => Self::Vec4,
            FloatAttributeType::BVec2 => Self::BVec2,
            FloatAttributeType::BVec3 => Self::BVec3,
            FloatAttributeType::BVec4 => Self::BVec4,
            FloatAttributeType::Mat2 => Self::Mat2,
            FloatAttributeType::Mat3 => Self::Mat3,
            FloatAttributeType::Mat4 => Self::Mat4,
        }
    }
}

/// Corresponds to an integral glsl type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IntegralAttributeType {
    Int,
    IVec2,
    IVec3,
    IVec4,
    UVec2,
    UVec3,
    UVec4,
}
impl From<IntegralAttributeType> for AttributeType {
    fn from(value: IntegralAttributeType) -> Self {
        match value {
            IntegralAttributeType::Int => Self::Int,
            IntegralAttributeType::IVec2 => Self::IVec2,
            IntegralAttributeType::IVec3 => Self::IVec3,
            IntegralAttributeType::IVec4 => Self::IVec4,
            IntegralAttributeType::UVec2 => Self::UVec2,
            IntegralAttributeType::UVec3 => Self::UVec3 ,
            IntegralAttributeType::UVec4 => Self::UVec4,
        }
    }
}

/// Generic attribute with a name and glsl type,
/// used as part of an input, uniform, or pipe
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub name: String,
    pub glsl_type: AttributeType,
}

pub enum InputAttribute {
    Integral { name: String, glsl_type: IntegralAttributeType, data_format: IntegralVertexFormat },
    Float { name: String, glsl_type: FloatAttributeType, data_format: FloatVertexFormat },
}
impl From<InputAttribute> for Attribute {
    fn from(value: InputAttribute) -> Self {
        match value {
            InputAttribute::Integral { name, glsl_type, .. } =>
                Self { name, glsl_type: glsl_type.into() },
            InputAttribute::Float { name, glsl_type, .. } =>
                Self { name, glsl_type: glsl_type.into() },
        }
    }
}

/// An input to the shader pipeline, stored in a [`VertexArray`].
#[derive(Clone, Debug, Hash)]
pub struct Input {
    index: u8,
    attribute: Attribute,
}
impl Input {
    // pub(crate) fn new<T: ToByteVec>(index: u8, attribute: Attribute, AttributePointer { buffer, stride, offset, format }: AttributePointer<T>) -> Self {
    //     buffer.bind();
    //     // buffer bound & index checked: shouldn't ever fail
    //     ox::vertex_attrib_pointer(index, format, stride, offset)
    //         .expect("buffer should be bound, and index checked");
    //     Self { index, attribute }
    // }
    pub(crate) fn new<T: ToByteVec>(index: u8, attribute: InputAttribute,
        AttributePointer { buffer, stride, offset }: AttributePointer<T>) -> Self {
        let (stride, offset) = (stride.0, offset.0);
        buffer.bind();
        match attribute {
            InputAttribute::Integral { name, glsl_type, data_format } => {
                ox::vertex_attrib_i_pointer(index, data_format, stride, offset)
                    .expect("buffer should be bound, and index checked");
                Self {
                    index, attribute: Attribute { name, glsl_type: glsl_type.into() }
                }
            },
            InputAttribute::Float { name, glsl_type, data_format } => {
                ox::vertex_attrib_pointer(index, data_format, stride, offset)
                    .expect("buffer should be bound, and index checked");
                Self {
                    index, attribute: Attribute { name, glsl_type: glsl_type.into() }
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PipeTargets {
    VertexFragment,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pipe {
    pub targets: PipeTargets,
    pub attribute: Attribute,
}

/// A vertex shader and its (nul-terminated) source
#[derive(Debug, Clone, Hash)]
struct VertexShader {
    shader: ox::Shader,
    source: CString,
}
/// A fragment shader and its (nul-terminated) source
#[derive(Debug, Clone, Hash)]
struct FragmentShader {
    shader: ox::Shader,
    source: CString,
}

/// Newtype allowing for deletion on drop
pub struct Program(ox::ShaderProgram);

impl Program {
    pub(crate) fn use_self(&self) -> Result<(),OwlError> {
        ox::use_program(self.0).with_message("using program failed")
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        ox::delete_program(self.0)
            .expect("program only deleted on drop");
    }
}

/// A representation of the shader pipeline as a whole, intended to be used as a builder, with the final stage ending in 'compile'
#[must_use]
#[derive(Debug)]
pub struct ShaderPipeline {
    version: u32,
    vertex: VertexShader,
    fragment: FragmentShader,
    inputs: Vec<Input>,
    pipes: Vec<Pipe>,
    output: Attribute
}

impl ShaderPipeline {
    pub fn new(glsl_version: u32) -> Result<Self,OwlError> {
        if ![430].contains(&glsl_version) {
            return Err(OwlError::custom("incorrect glsl version, accepted versions: 430"));
        }
        let vertex = ox::create_shader(ox::ShaderType::Vertex)
            .with_message("creating pipeline (vertex shader)")?;
        let fragment = ox::create_shader(ox::ShaderType::Fragment)
            .with_message("creating pipeline (fragment shader)")?;
        Ok(Self {
            version: glsl_version,
            vertex: VertexShader { shader: vertex, source: CString::default() },
            fragment: FragmentShader { shader: fragment, source: CString::default() },
            inputs: Vec::new(),
            output: Attribute { name: String::default(), glsl_type: AttributeType::Vec4 },
            pipes: Vec::new(),
        })
    }
    pub fn inputs_from_vertex_array<T: ToByteVec>(self, vertex_array: &VertexArray<T>) -> Self {
        Self {
            inputs: vertex_array.inputs.clone(),
            ..self
        }
    }
    pub fn vertex_body(self, source: &str) -> Result<Self,std::ffi::NulError> {
        Ok(Self {
            vertex: VertexShader { source: CString::new(source)?, ..self.vertex  },
            ..self
        })
    }
    pub fn fragment_body(self, source: &str, output: Attribute) -> Result<Self,std::ffi::NulError> {
        Ok(Self {
            fragment: FragmentShader { source: CString::new(source)?, ..self.fragment },
            output,
            ..self
        })
    }
    pub fn pipe(mut self, pipe: Pipe) -> Self {
        self.pipes.push(pipe);
        self
    }
    /// # Panics
    ///
    /// This function should never panic. If it does, this is a bug.
    pub fn compile(self) -> Result<Program,OwlError> {
        // add inputs to vertex code
        let version_prelude = format!("#version {} core\n", self.version);
        let vertex_source = {
            let input_to_glsl = |i: &Input| {
                format!("layout (location = {}) in {} {};\n", i.index, i.attribute.glsl_type, i.attribute.name)
            };
            let ins_prelude: String = self.inputs.iter().map(input_to_glsl).collect();
            let body = self.vertex.source.into_string().expect("created from &str, so valid UTF-8");
            let pipes_prelude: String = self.pipes.iter()
                .filter_map(|Pipe { targets, attribute }| {
                    match targets {
                        PipeTargets::VertexFragment => Some(
                            format!("out {} {};\n", attribute.glsl_type, attribute.name)
                        )
                    }
            }).collect();
            CString::new(version_prelude.clone() + &ins_prelude + &pipes_prelude + &body)
                .expect("created from a collection of valid UTF-8 strings, so must be valid")
        };
        // println!("{}", vertex_source.clone().into_string().unwrap());
        let fragment_source = {
            let pipes_prelude: String = self.pipes.iter()
                .filter_map(|Pipe { targets, attribute }| {
                    match targets {
                        PipeTargets::VertexFragment => Some(
                            format!("in {} {};\n", attribute.glsl_type, attribute.name)
                        )
                    }
            }).collect();
            let out_prelude = format!("out {} {};\n", self.output.glsl_type, self.output.name);
            let body = self.fragment.source.into_string().expect("created from &str, so valid UTF-8");
            CString::new(version_prelude + &pipes_prelude + &out_prelude + &body)
                .expect("created from a collection of valid UTF-8 strings, so must be valid")
        };
        // println!("{}", fragment_source.clone().into_string().unwrap());
        // compile shaders
        ox::shader_source(self.vertex.shader, &[vertex_source]).and(
            ox::shader_source(self.fragment.shader, &[fragment_source]))
            .expect("shaders not yet deleted");
        ox::compile_shader(self.vertex.shader).with_message("compiling pipeline (vertex shader)")?;
        ox::compile_shader(self.fragment.shader).with_message("compiling pipeline (fragment shader)")?;
        // link program
        let program = ox::create_program().with_message("compiling pipeline (shader program)")?;
        ox::attach_shader(program, self.vertex.shader)
            .expect("shader is neither deleted, nor already attached");
        ox::attach_shader(program, self.fragment.shader)
            .expect("shader is neither deleted, nor already attached");
        ox::link_program(program).with_message("compiling pipeline (linking program)")?;
        ox::delete_shader(self.vertex.shader)
            .expect("shader is not deleted");
        ox::delete_shader(self.fragment.shader)
            .expect("shader is not deleted");
        Ok(Program(program))
    }
}
