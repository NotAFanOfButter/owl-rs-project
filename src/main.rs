use winit::{self, event_loop};
use raw_gl_context;

use owl;

fn main() -> Result<(), owl::OwlError> {
    use winit::{
        dpi::LogicalSize,
        event::{Event,WindowEvent},
        event_loop::EventLoop,
        window,
    };
    use raw_gl_context::{GlConfig,GlContext};

    let event_loop = EventLoop::new().expect("failed to create an event loop");
    event_loop.set_control_flow(event_loop::ControlFlow::Poll);
    let window = window::WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800,600))
        .with_title("Opengl Square")
        .with_enabled_buttons(winit::window::WindowButtons::CLOSE)
        .build(&event_loop).expect("failed to create a window");
    let context = unsafe { GlContext::create(&window,
        GlConfig { version: (4,3), ..Default::default() })
        .expect("failed to create an OpenGL context") };
    unsafe {
        context.make_current();
    }
    owl::load_proc(&context);

    use owl::ToByteVec;
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
    let mut vertex_array_object = owl::VertexArray::new();
    vertex_array_object.add_input(
        owl::Attribute { name: "pos".to_owned(), glsl_type: owl::AttributeType::Vec3 },
        owl::AttributePointer { buffer: &vertex_buffer, stride: 3*std::mem::size_of::<f32>(), offset: 0,
            format: owl::VertexFormat::Size3 { normalised: false, data_type: owl::DataTypeSize3::Float }}
    )?;
    vertex_array_object.add_input_from_buffer(&vertex_buffer, owl::Attribute::Vec3("pos".to_owned()), false, 3, 0)
        .expect("failed to add vertex attribute");
    vertex_array_object.add_element_buffer_data(owl::TypedData::U32(indices), owl::BufferUsage::StaticDraw)
        .expect("failed to create / add element buffer");
    let shader_program = owl::ShaderPipeline::new(430).expect("failed to create pipeline")
        .with_inputs_from_vertex_array(&vertex_array_object)
        .with_vertex_body(r#"
        void main() {
            gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
        }
        "#).expect("failed to parse vertex body")
        .with_fragment_body(r#"
        void main() {
            colour = vec4(0.0, 0.5, 0.5, 1.0);
        }
        "#, "colour").expect("failed to parse fragment body")
        .compile()
        .unwrap_or_else(|e| panic!("{e}"));
    let triangle = owl::Mesh { start: 0, count: 4 };

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
