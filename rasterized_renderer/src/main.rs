use std::time::Instant;

use cgmath::{Vector3, vec3};
use glium::{Surface, glutin::surface::WindowSurface, winit::{application::ApplicationHandler, event::WindowEvent}};
use rasterized_renderer::{mesh::{Mesh, MeshRenderer, Model}, transform::Transform, vertex::Vertex};

struct SampleApp<'a> {
    window: glium::winit::window::Window,
    display: glium::Display<WindowSurface>,
    mesh_renderer: MeshRenderer<'a>,
    
    models: Vec<Model<'a>>,

    t0: Instant,
}

impl<'a> ApplicationHandler for SampleApp<'a> {
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
            WindowEvent::RedrawRequested => { self.render(); }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &glium::winit::event_loop::ActiveEventLoop) {
        self.window.request_redraw();
    }
}

impl<'a> SampleApp<'a> {
    fn render(&mut self) {
        let t = self.t0.elapsed().as_secs_f32().sin();

        let model = &mut self.models[0];
        model.transform.position = vec3(t, 0.0, 0.0);

        let mut frame = self.display.draw();
        frame.clear_color(0.0, 0.0, 1.0, 1.0);
        self.mesh_renderer.render(&mut frame, &self.models);
        frame.finish().unwrap();
    }
}


fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build().expect("Could not build event loop");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    
    let vertices = [
        Vertex { position: [-0.5, -0.5 ] },
        Vertex { position: [ 0.5, -0.25] },
        Vertex { position: [ 0.0, 0.5 ] },
        Vertex { position: [0.75, 0.75] },
    ];
    let indices: [u16; _] = [
        0, 1, 2, 2, 1, 3
    ];

    let mesh = Mesh::new(
        &display, 
        &vertices, 
        &indices
    ).unwrap();

    let model = Model {
        mesh: &mesh,
        transform: Transform { position: Vector3::new(-0.5, 0.0, 0.0) },
    };

    let mesh_renderer = MeshRenderer::new(&display).unwrap();

    let mut app = SampleApp {
        window,
        display,
        mesh_renderer,
        models: vec![model],
        t0: Instant::now(),
    };

    event_loop.run_app(&mut app).unwrap();
}
