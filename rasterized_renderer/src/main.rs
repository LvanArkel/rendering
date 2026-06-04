use std::time::Instant;

use cgmath::{Deg, Quaternion, Rad, Rotation3, Vector3, point3, vec3};
use glium::{Surface, glutin::surface::WindowSurface, winit::{application::ApplicationHandler, event::WindowEvent}};
use rasterized_renderer::{camera::Camera, mesh::{Mesh, MeshRenderer, Model}, transform::Transform, vertex::Vertex};

fn cube() -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let positions = [
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
        [-0.5, -0.5, 0.5],
        [0.5, -0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5],
    ];
    let index_pattern: [u16; _] = [0, 1, 2, 2, 1, 3];
    let face_vertices = [
        [0, 4, 2, 6], // -X
        [5, 1, 7, 3], // +X
        [0, 1, 4, 5], // -Y
        [6, 7, 2, 3], // +Y
        [1, 0, 3, 2], // -Z
        [4, 5, 6, 7], // +Z
    ];

    for face in 0..6 {
        let pattern = &face_vertices[face];
        for vertex_i in pattern {
            vertices.push(
                Vertex {
                    position: positions[*vertex_i]
                }
            );
        }
        for index in index_pattern{
            indices.push(4 * (face as u16) + index);
        }
    }
    

    (vertices, indices)
}

struct SampleApp<'a> {
    window: glium::winit::window::Window,
    display: glium::Display<WindowSurface>,

    camera: Camera,
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
        let t = self.t0.elapsed().as_secs_f32();

        let model = &mut self.models[0];
        model.transform.rotation = Quaternion::from_angle_y(Rad(t));
        model.transform.position = vec3((t*2.0).sin(), 0.0, 0.0);
        model.transform.scale = vec3(t.cos()*0.5+0.5, 1.0, 1.0);

        let mut frame = self.display.draw();
        frame.clear_color(0.0, 0.0, 1.0, 1.0);
        self.mesh_renderer.render(&mut frame, &self.camera, &self.models);
        frame.finish().unwrap();
    }
}


fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build().expect("Could not build event loop");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let cube = cube();
    let (vertices, indices) = cube;

    let mesh = Mesh::new(
        &display, 
        &vertices, 
        &indices
    ).unwrap();

    let model = Model {
        mesh: &mesh,
        transform: Transform { 
            position: Vector3::new(0.5, 0.0, 0.0),
            rotation: Quaternion::from_angle_x(Deg(45.0)),
            scale: vec3(1.0, 1.0, 1.0),
        },
    };

    
    let camera_position = point3(2.0, 2.0, 2.0);
    let camera_target = point3(0.5, 0.5, 0.5);
    let fov_y = Deg(90.0);
    let window_size = window.inner_size();
    let aspect_ratio = window_size.width as f32 / window_size.height as f32;
    let near_z= 0.1;
    let far_z = 1000.0;

    let camera = Camera::new(camera_position, camera_target, fov_y, aspect_ratio, near_z, far_z);

    let mesh_renderer = MeshRenderer::new(&display).unwrap();

    let mut app = SampleApp {
        window,
        display,
        camera,
        mesh_renderer,
        models: vec![model],
        t0: Instant::now(),
    };

    event_loop.run_app(&mut app).unwrap();
}
