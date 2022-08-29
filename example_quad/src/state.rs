extern crate bvhtracer;
extern crate cglinalg;


#[allow(clippy::all)]
mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}


use bvhtracer::*;
use crate::app::*;
use crate::context::*;
use cglinalg::{
    Matrix4x4,
    Vector2,
    Vector3,
};
use std::io;


impl GpuTextureBuffer2D for TextureBuffer2D<Rgba<u8>, Vec<u8>> {
    fn format(&self) -> GpuTextureFormat { 
        GpuTextureFormat::Rgba8
    }

    fn width(&self) -> usize {
        TextureBuffer2D::width(self)
    }

    fn height(&self) -> usize {
        TextureBuffer2D::height(self)
    }

    fn dimensions(&self) -> (usize, usize) {
        TextureBuffer2D::dimensions(self)
    }

    fn as_ptr(&self) -> *const u8 {
        TextureBuffer2D::as_ptr(self) as *const u8
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        TextureBuffer2D::as_mut_ptr(self) as *mut u8
    }
}


pub struct AppStateQuad {
    active_scene: Scene,
}

impl AppStateQuad {
    pub fn new() -> Self {
        /*
        let model_spec = PerspectiveFovSpec::new(
            Degrees(90_f32), 
            1_f32, 
            0.001_f32, 
            100_f32, 
        );
        */
        let model_spec = PerspectiveSpec::new(
            -1_f32, 
            1_f32, 
            -1_f32, 
            1_f32, 
            1_f32, 
            100_f32, 
        );
        let attitude_spec = CameraAttitudeSpec::new(
            Vector3::new(0_f32, 0_f32, 2_f32),
            -Vector3::unit_z(),
            Vector3::unit_x(),
            Vector3::unit_y(),
            -Vector3::unit_z()
        );
        let camera = Camera::new(&model_spec, &attitude_spec);
        let mesh = MeshBuilder::new()
            .with_primitive(
                Triangle::new(
                    Vector3::new(-1.0, -1.0, 0.0), 
                    Vector3::new( 1.0,  1.0, 0.0), 
                    Vector3::new(-1.0,  1.0, 0.0),
                ),
                TextureCoordinates::from([
                    Vector2::new(0.0, 0.0),
                    Vector2::new(1.0, 1.0),
                    Vector2::new(0.0, 1.0),
                ]),
                Normals::from([
                    Vector3::new(0.0, 0.0, 1.0),
                    Vector3::new(0.0, 0.0, 1.0),
                    Vector3::new(0.0, 0.0, 1.0),
                ])
            )
            .with_primitive(
                Triangle::new(
                    Vector3::new(-1.0, -1.0, 0.0),
                    Vector3::new( 1.0, -1.0, 0.0),
                    Vector3::new( 1.0,  1.0, 0.0),
                ),
                TextureCoordinates::from([
                    Vector2::new(0.0, 0.0),
                    Vector2::new(1.0, 0.0),
                    Vector2::new(1.0, 1.0),
                ]),
                Normals::from([
                    Vector3::new(0.0, 0.0, 1.0),
                    Vector3::new(0.0, 0.0, 1.0),
                    Vector3::new(0.0, 0.0, 1.0),
                ])
            )
            .build();
        let material = {
            let material_file = include_bytes!("../assets/bricks_rgb.png");
            let material_reader = io::Cursor::new(material_file);
            let decoder: PngTextureBufferDecoder<Rgb<u8>, _> = PngTextureBufferDecoder::new(material_reader);
            let texture = decoder.read_texture().unwrap();
            TextureMaterial::new(texture)
        };
        let model = ModelBuilder::new()
            .with_mesh(mesh)
            .with_texture(material)
            .build();
        let scene_object = SceneObjectBuilder::new(model)
            .with_transform(&Matrix4x4::identity())
            .build();
        let active_scene = SceneBuilder::new(camera)
            .with_object(scene_object)
            .build();
        
        Self { active_scene, }
    }
}

impl AppState for AppStateQuad {
    fn update(&mut self, _elapsed: f64) {
    }

    fn active_scene(&self) -> &Scene {
        &self.active_scene
    }

    fn active_scene_mut(&mut self) -> &mut Scene {
        &mut self.active_scene
    }
}

