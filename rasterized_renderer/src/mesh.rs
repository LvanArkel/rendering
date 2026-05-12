use glium::{Surface, backend::Facade, uniform};

use crate::{camera::Camera, transform::Transform, vertex::Vertex};

pub struct Mesh {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
}

impl Mesh {
    pub fn new(
        facade: &dyn glium::backend::Facade,
        vertices: &[Vertex],
        indices: &[u16],
    ) -> anyhow::Result<Mesh> {
        let vertex_buffer = glium::VertexBuffer::new(facade, vertices)?;
        let index_buffer = glium::IndexBuffer::new(
            facade, 
            glium::index::PrimitiveType::TrianglesList, 
            indices)?;
        Ok(Self { vertex_buffer, index_buffer })
    }
}

pub struct Model<'a> {
    pub mesh: &'a Mesh,
    pub transform: Transform,
}

pub struct MeshRenderer<'a> {
    program: glium::Program,
    draw_parameters: glium::DrawParameters<'a>,
}

impl<'a> MeshRenderer<'a> {
    pub fn new(facade: &dyn Facade) -> anyhow::Result<Self> {
        let program = glium::Program::from_source(
            facade, 
            include_str!("default.vert"),
            include_str!("default.frag"),
            None
        )?;
        let draw_parameters = glium::DrawParameters {
            backface_culling: glium::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };
        
        Ok(Self { program, draw_parameters })
    }

    pub fn render(
        &self, 
        frame: &mut glium::Frame, 
        camera: &Camera,
        models: &[Model],
    ) -> anyhow::Result<()> {
        let view_projection = camera.view_projection_matrix();

        for model in models {
            let model_matrix = model.transform.to_matrix();

            let uniforms = uniform! {
                model_view_projection: Into::<[[f32; 4]; 4]>::into(view_projection * model_matrix),
            };

            frame.draw(
                &model.mesh.vertex_buffer, 
                &model.mesh.index_buffer,
                &self.program, 
                &uniforms,
                &self.draw_parameters
            )?;
        }
        Ok(())
    }
}
