extern crate bvhtracer;
extern crate bvhtracer_demos;
extern crate cglinalg;


const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 640;


use bvhtracer::*;
use bvhtracer_demos::*;
use cglinalg::{
    Matrix4x4,
    Vector3,
    Radians,
    Degrees,
    Scale3,
    Translation3,
    Rotation3,
};
use std::io;


struct AppStateTwoArmadillos {
    angle: f32,
    active_scene: Scene,
}

impl AppStateTwoArmadillos {
    fn new() -> Self {
        let projection_spec = SymmetricFovSpec::new(
            Degrees(90_f32),
            1_f32,
            2_f32,
            10000_f32,
        );
        let attitude_spec = CameraAttitudeSpec::new(
            Vector3::new(0_f32, 1_f32, -2.5_f32),
            Vector3::unit_z(),
            Vector3::unit_x(),
            Vector3::unit_y(),
            Vector3::unit_z()
        );
        let camera = Camera::new(&projection_spec, &attitude_spec);
        let mesh_file = include_bytes!("assets/armadillo.tri");
        let mesh_decoder = TriMeshDecoder::new(io::Cursor::new(mesh_file));
        let mesh = mesh_decoder.read_mesh().unwrap();
        let model_builder = ModelBuilder::new();
        let model = model_builder.with_mesh(mesh).build();
        let mut objects = vec![];
        /*
        let model1_transform = Matrix4x4::from_affine_translation(
            &Vector3::new(-1.3_f32, 0_f32, 0_f32)
        );
        let model2_transform = Matrix4x4::from_affine_translation(
            &Vector3::new(1.3_f32, 0_f32, 0_f32)
        );
        */
        let model1_transform = Transform3::from_translation(&Vector3::new(-1.3_f32, 0_f32, 0_f32));
        let model2_transform = Transform3::from_translation(&Vector3::new(1.3_f32, 0_f32, 0_f32));
        let model1 = model.clone();
        let model2 = model.clone();
        let mut physics = World::new();
        let rigid_body1 = RigidBody::default();
        let rigid_body1_instance = physics.register_body(rigid_body1);
        let rigid_body2 = RigidBody::default();
        let rigid_body2_instance = physics.register_body(rigid_body2);
        let object1 = SceneObjectBuilder::new(model1, rigid_body1_instance)
            .with_transform(&model1_transform)
            .build();
        let object2 = SceneObjectBuilder::new(model2, rigid_body2_instance)
            .with_transform(&model2_transform)
            .build();
        objects.push(object1);
        objects.push(object2);
        let active_scene = SceneBuilder::new(camera)
            .with_objects(objects)
            .build();
        let angle = 0_f32;

        Self { angle, active_scene, }
    }
}

impl AppState for AppStateTwoArmadillos {
    fn update(&mut self, _elapsed: f64) {
        self.angle += 0.01;
        if self.angle > std::f32::consts::FRAC_2_PI {
            self.angle -= std::f32::consts::FRAC_2_PI;
        }
        /*
        self.active_scene.get_mut_unchecked(0).set_transform(
            &Matrix4x4::from_affine_translation(&Vector3::new(-1.3_f32, 0_f32, 0_f32))
        );
        self.active_scene.get_mut_unchecked(1).set_transform(&(
            Matrix4x4::from_affine_translation(&Vector3::new(1.3_f32, 0_f32, 0_f32)) *
            Matrix4x4::from_affine_angle_y(Radians(self.angle))
        ));
        */
        self.active_scene.get_mut_unchecked(0).set_transform(
            &Transform3::from_translation(&Vector3::new(-1.3_f32, 0_f32, 0_f32))
        );
        self.active_scene.get_mut_unchecked(1).set_transform(
            &Transform3::new(
                &Vector3::zero(),
                &Vector3::new(1.3_f32, 0_f32, 0_f32),
                Rotation3::from_angle_y(Radians(self.angle))
            )
        );

        self.active_scene.rebuild();
    }

    fn active_scene(&self) -> &Scene {
        &self.active_scene
    }

    fn active_scene_mut(&mut self) -> &mut Scene {
        &mut self.active_scene
    }
}


pub fn main() -> io::Result<()> {
    use std::time::SystemTime;
    println!("Building scene.");
    let now = SystemTime::now();
    let state = Box::new(AppStateTwoArmadillos::new());
    let elapsed = now.elapsed().unwrap();
    println!("Scene building time = {:?}", elapsed);
    let accumulator = Box::new(DepthAccumulator::new());
    let pixel_shader = Box::new(DepthMappingShader::new(80_f32, 3_f32));
    let renderer = Renderer::new(Box::new(PathTracer::new()));
    let context = init_gl("OpenGL Window", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).unwrap();
    let mut app = App::new(context, pixel_shader, accumulator, state, renderer, SCREEN_WIDTH, SCREEN_HEIGHT);
    app.run();

    Ok(())
}

