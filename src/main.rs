use winit;
use raw_gl_context;
use gl;

fn main() {
    let event_loop = match winit::event_loop::EventLoop::new() {
        Ok(lp) => lp,
        Err(_) => std::process::exit(0)
    };
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize::new(800,600))
        .with_title("opengl-winit")
        .with_enabled_buttons(winit::window::WindowButtons::CLOSE)
        .build(&event_loop).unwrap();
    let context = match raw_gl_context::GlContext::create(&window, raw_gl_context::GlConfig::default()) {
        Ok(context) => context,
        Err(_) => std::process::exit(0)
    };
    context.make_current();
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    event_loop.run(|event, elwt| {
        match event {
            winit::event::Event::WindowEvent {event, ..} => {
                match event {
                    winit::event::WindowEvent::CloseRequested => elwt.exit(),
                    winit::event::WindowEvent::RedrawRequested => {
                        context.make_current();
                        unsafe {
                            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
                            gl::Clear(gl::COLOR_BUFFER_BIT);
                        }
                        context.swap_buffers();
                        context.make_not_current();
                    },
                    _ => ()
                }
            }
            _ => ()
        }
    }).unwrap();
}
