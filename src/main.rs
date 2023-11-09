use winit;
use raw_gl_context;
use gl;

fn main() {
    use winit::{
        dpi::LogicalSize,
        event::{Event,WindowEvent},
        event_loop::EventLoop,
        window,
    };
    use raw_gl_context::{GlConfig,GlContext};
    let event_loop = EventLoop::new().expect("Should have created an event loop");
    let window = window::WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800,600))
        .with_title("opengl-winit")
        .with_enabled_buttons(winit::window::WindowButtons::CLOSE)
        .build(&event_loop).expect("Failed to create a window");
    let context = unsafe { GlContext::create(&window,
        GlConfig { version: (3,3), ..Default::default() })
        .expect("Failed to create an OpenGL context") };
    unsafe {
        context.make_current();
    }
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    event_loop.run(|event, elwt| {
        match event {
            Event::WindowEvent {event, ..} => {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::RedrawRequested => {
                        unsafe {
                            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
                            gl::Clear(gl::COLOR_BUFFER_BIT);
                        }
                        context.swap_buffers();
                    },
                    _ => ()
                }
            }
            _ => ()
        }
    }).expect("Failed to run an event loop");
}
