extern crate bvhtracer;
extern crate bvhtracer_demos;
extern crate cglinalg;


const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 640;


use bvhtracer::*;
use bvhtracer_demos::*;
use cglinalg::{
    Matrix4x4,
    Vector2,
    Vector3,
};
use std::io;


struct AppStateQuad {
    active_scene: Scene,
}

impl AppStateQuad {
    fn new() -> Self {
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


pub fn run() -> io::Result<()> {
    use std::time::SystemTime;
    println!("Building scene.");
    let now = SystemTime::now();
    let state = Box::new(AppStateQuad::new());
    let elapsed = now.elapsed().unwrap();
    println!("Scene building time = {:?}", elapsed);    
    let accumulator = Box::new(TextureMaterialAccumulator::new());
    let pixel_shader = Box::new(RadianceToRgbShader::new());
    let renderer = Renderer::new(Box::new(PathTracer::new()));

    let context = init_gl("OpenGL Window", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).unwrap();
    let mut app = App::new(context, pixel_shader, accumulator, state, renderer, SCREEN_WIDTH, SCREEN_HEIGHT);
    app.run();

    Ok(())
}

