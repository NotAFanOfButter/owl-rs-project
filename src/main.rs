use winit::{
    dpi::LogicalSize,
    event::{Event,WindowEvent},
    event_loop::{EventLoop,ControlFlow},
    window::{WindowButtons,WindowBuilder},
};
use raw_gl_context::{GlConfig,GlContext};

use owl::prelude::*;

fn main() -> Result<(), owl::OwlError> {
    //
    // Setup
    //

    let event_loop = EventLoop::new().expect("failed to create an event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800,600))
        .with_title("Opengl Square")
        .with_enabled_buttons(WindowButtons::CLOSE)
        .build(&event_loop).expect("failed to create a window");
    let context = unsafe { GlContext::create(&window,
        GlConfig { version: (4,3), ..Default::default() })
        .expect("failed to create an OpenGL context") };
    unsafe {
        context.make_current();
    }
    owl::load_proc(&context);

    //
    // Vertices
    //

    #[derive(ToByteVec)]
    struct Vertex(f32,f32,f32);
    let vertices = vec![
        Vertex(-0.5, 0.5, 0.0),
        Vertex(-0.5, -0.5, 0.0),
        Vertex(0.5, 0.5, 0.0),
        Vertex(0.5, -0.5, 0.0),
    ];
    let vertex_buffer = owl::ArrayBuffer::new(vertices, owl::BufferUsage::StaticDraw)?;
    let indices = vec![
        0, 1, 2,
    ];
    let index_buffer = owl::ElementBuffer::new(
        indices, owl::BufferUsage::StaticDraw, owl::IndexType::UnsignedInt)?;
    let mut vertex_array_object = owl::VertexArray::new()
        .with_indices(index_buffer)
        .with_input(
            owl::Attribute { name: "pos".to_owned(), glsl_type: owl::AttributeType::Vec3 },
            owl::AttributePointer { 
                buffer: &vertex_buffer,
                stride: 3 * std::mem::size_of::<f32>(),
                offset: 0,
                format: owl::VertexFormat::Size3 { normalised: false, data_type: owl::DataTypeSize3::Float }
            }
        )?;

    //
    // Shader Pipeline
    //
    let shader_program = owl::ShaderPipeline::new(430)?
        .inputs_from_vertex_array(&vertex_array_object)
        .vertex_body(r#"
            void main() {
                gl_Position = vec4(pos, 1.0);
            }
            "#).expect("no nul bytes")
        .fragment_body(r#"
            void main() {
                colour = vec4(0.0, 0.5, 0.5, 1.0);
            }
            "#,
            owl::Attribute { name: "colour".to_string(), glsl_type: owl::AttributeType::Vec4 })
            .expect("no nul bytes")
        .compile()?;
    
    let triangle = owl::Mesh { start: 0, count: 4, vertex_array: &vertex_array_object };

    event_loop.run(|event, elwt| {
        match event {
            Event::WindowEvent {event, ..} => {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::RedrawRequested => {
                        owl::colour_clear(owl::Colour(0.1, 0.1, 0.1, 1.0));
                        triangle.draw_triangles(&vertex_array_object, &shader_program);
                        context.swap_buffers();
                    },
                    _ => ()
                }
            }
            _ => ()
        }
    }).expect("Failed to run an event loop");
    Ok(())
}
