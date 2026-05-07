use glium::{Surface, winit::{application::ApplicationHandler, event::WindowEvent}};
use rasterized_renderer::vertex::Vertex;

struct SampleApp;

impl ApplicationHandler for SampleApp {
    fn resumed(&mut self, event_loop: &glium::winit::event_loop::ActiveEventLoop) {
        // todo!()
    }

    fn window_event(
        &mut self,
        event_loop: &glium::winit::event_loop::ActiveEventLoop,
        window_id: glium::winit::window::WindowId,
        event: glium::winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => { event_loop.exit(); }
            _ => {}
        }
    }
}


fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build().expect("Could not build event loop");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    
    let shape = [
        Vertex { position: [-0.5, -0.5 ] },
        Vertex { position: [ 0.5, -0.25] },
        Vertex { position: [ 0.0, 0.5 ] },
            Vertex { position: [ 0.0, 0.5 ] },
            Vertex { position: [ 0.5, -0.25] },
        Vertex { position: [0.75, 0.75] },
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = glium::Program::from_source(
        &display,
        include_str!("default.vert"),
        include_str!("default.frag"),
        None,
    ).unwrap();
        
    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 1.0, 1.0);
    frame.draw(
        &vertex_buffer, 
        &indices, 
        &program, 
        &glium::uniforms::EmptyUniforms, 
        &glium::DrawParameters {
            backface_culling: glium::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        }
    ).unwrap();
    frame.finish().unwrap();
    

    let mut app = SampleApp;

    event_loop.run_app(&mut app).unwrap();
}
