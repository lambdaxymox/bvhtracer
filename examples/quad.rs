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
    Unit,
    Radians,
    Degrees,
    Rotation3,
};
use std::io;


struct AppStateQuad {
    active_scene: Scene,
}

impl AppStateQuad {
    fn new() -> Self {
        let projection_spec = SymmetricFovSpec::new(
            Degrees(90_f32),
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
        let camera = Camera::new(&projection_spec, &attitude_spec);
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
            let material_file = include_bytes!("assets/bricks_rgb.png");
            let material_reader = io::Cursor::new(material_file);
            let decoder: PngTextureBufferDecoder<Rgb<u8>, _> = PngTextureBufferDecoder::new(material_reader);
            let texture = decoder.read_texture().unwrap();
            TextureMaterial::new(texture)
        };
        let model = ModelBuilder::new()
            .with_mesh(mesh)
            .with_texture(material)
            .build();
        let mut physics = World::new();
        let rigid_body = {
            let mut _rigid_body = RigidBody::default();
            let angular_frequency = 2_f32;
            let position_world = Vector3::new(0_f32, 0_f32, 0_f32);
            _rigid_body.set_rotation(&Vector3::new(0_f32, 0_f32, angular_frequency));
            _rigid_body.set_awake(true);
            _rigid_body.set_angular_drag(1_f32);
            _rigid_body.set_position(&position_world);
            _rigid_body
        };
        let rigid_body_instance = physics.register_body(rigid_body);
        let scene_object = SceneObjectBuilder::new(model, rigid_body_instance)
            .with_transform(&Transform3::identity())
            .build();
        let active_scene = SceneBuilder::new(camera)
            .with_physics(physics)
            .with_object(scene_object)
            .build();

        Self { active_scene, }
    }
}

impl AppState for AppStateQuad {
    fn update(&mut self, elapsed: f64) {
        self.active_scene.run(elapsed);
    }

    fn active_scene(&self) -> &Scene {
        &self.active_scene
    }

    fn active_scene_mut(&mut self) -> &mut Scene {
        &mut self.active_scene
    }
}


fn main() -> io::Result<()> {
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

