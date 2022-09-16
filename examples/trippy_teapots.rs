extern crate bvhtracer;
extern crate bvhtracer_demos;
extern crate cglinalg;


const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 640;


use bvhtracer::*;
use bvhtracer_demos::*;
use cglinalg::{
    Vector3,
    Radians,
    Degrees,
    Rotation3,
};
use std::io;


struct Physics {
    angle: f32,
    position_init: Vector3<f32>,
    height: f32,
    speed: f32,
    angular_velocity: f32,
    acceleration: f32,
    direction: f32,
}

impl Physics {
    fn new(angle: f32, position_init: Vector3<f32>, height: f32, speed: f32, angular_velocity: f32, acceleration: f32) -> Self {
        Self { 
            angle, 
            height, 
            position_init, 
            speed, 
            angular_velocity, 
            acceleration, 
            direction: -1_f32,
        }
    }
}


struct AppStateTrippyTeapots {
    active_scene: Scene,
    physics: Vec<Physics>,
}

impl AppStateTrippyTeapots {
    fn new() -> Self {
        let projection_spec = SymmetricFovSpec::new(
            Degrees(90_f32),
            1_f32,
            2_f32,
            10000_f32,
        );
        let attitude_spec = CameraAttitudeSpec::new(
            Vector3::new(0_f32, 1.5_f32, -5.5_f32),
            Vector3::unit_z(),
            -Vector3::unit_x(),
            Vector3::unit_y(),
            Vector3::unit_z()
        );
        let camera = Camera::new(&projection_spec, &attitude_spec);
        let mesh_file = include_bytes!("assets/teapot.obj");
        let mesh_reader = io::Cursor::new(mesh_file);
        let material_file = include_bytes!("assets/bricks_rgb.png");
        let material_reader = io::Cursor::new(material_file);
        let model = SimpleModelDecoder::new(mesh_reader, material_reader)
            .read_model()
            .unwrap();
        let mut world = World::new();
        let mut objects = vec![]; 
        let mut physics = vec![];
        let height_init = vec![
            5_f32, 4_f32, 3_f32, 2_f32, 1_f32, 5_f32, 4_f32, 3_f32, 
            5_f32, 4_f32, 3_f32, 2_f32, 1_f32, 5_f32, 4_f32, 3_f32,
        ];
        let mut i = 0;
        for x in 0..4 { for y in 0..4 {
            let rigid_body = {
                let mut _rigid_body = RigidBody::default();
                _rigid_body.set_can_sleep(false);
                _rigid_body.set_awake(true);
                _rigid_body
            };
            let rigid_body_instance = world.register_body(rigid_body);
            let angular_velocity = if ((x + y) & 1) == 0 {
                (((i * 13) & 7 + 2) as f32) * 0.10
            } else {
                0_f32
            };
            let horizontal = Vector3::new((x as f32 - 1.5) * 2.5, 0_f32, (y as f32 - 1.5) * 2.5);
            let vertical = if ((x + y) & 1) == 0 {
                Vector3::zero()
            } else {
                Vector3::new(0_f32, height_init[i / 2], 0_f32)
            };
            let scale = Vector3::from_fill(0.75);
            let translation = horizontal + vertical;
            let rotation = Rotation3::from_angle_x(Radians(0_f32)) * 
                Rotation3::from_angle_z(Radians(0_f32));
            let transform = Transform3::new(&scale, &translation, rotation);
            let angle = 0_f32;
            let acceleration =  if ((x + y) & 1) == 0 { 0_f32 } else { 9.8 };
            let speed = 0_f32;
            let object_physics = Physics::new(
                angle, 
                translation, 
                vertical.y,
                speed, 
                angular_velocity, 
                acceleration
            );
            let object = SceneObjectBuilder::new(model.clone(), rigid_body_instance)
                .with_transform(&transform)
                .build();
            objects.push(object);
            physics.push(object_physics);

            i += 1;
        }}
        let active_scene = SceneBuilder::new(camera)
            .with_objects(objects)
            .build();

        Self { active_scene, physics, }
    }
}

impl AppState for AppStateTrippyTeapots {
    fn update(&mut self, elapsed: f64) {
        for i in 0..16 {
            let new_transform = {
                let scale = Vector3::from_fill(0.75_f32);
                let height = self.physics[i].height;
                let translation = self.physics[i].position_init + Vector3::new(0_f32, height, 0_f32);
                let angle = Radians(self.physics[i].angle);
                let rotation = Rotation3::from_angle_x(angle) * Rotation3::from_angle_z(angle);
                Transform3::new(&scale, &translation, rotation)
            };
            self.active_scene.get_mut_unchecked(i).set_transform(&new_transform);
        }

        for i in 0..16 {
            let new_angle = self.physics[i].angle + self.physics[i].angular_velocity * (elapsed as f32);
            self.physics[i].angle = new_angle;
            self.physics[i].speed += self.physics[i].acceleration * (elapsed as f32);
            self.physics[i].height += self.physics[i].direction * self.physics[i].speed * (elapsed as f32);
            if self.physics[i].height < -3_f32 {
                self.physics[i].height = (-3_f32) + 0.01;
                self.physics[i].direction = -self.physics[i].direction;
                self.physics[i].speed = 0.2;
            } else if self.physics[i].height > self.physics[i].position_init.y { 
                self.physics[i].height = self.physics[i].position_init.y - 0.01;
                self.physics[i].direction = -self.physics[i].direction;
            } else {

            }
        }

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
    let state = Box::new(AppStateTrippyTeapots::new());
    let elapsed = now.elapsed().unwrap();
    println!("Scene building time = {:?}", elapsed);
    let accumulator = Box::new(NormalMappingAccumulator::new());
    let pixel_shader = Box::new(RadianceToRgbShader::new());
    let renderer = Renderer::new(Box::new(PathTracer::new()));
    let context = init_gl("OpenGL Window", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).unwrap();
    let mut app = App::new(context, pixel_shader, accumulator, state, renderer, SCREEN_WIDTH, SCREEN_HEIGHT);
    app.run();

    Ok(())
}

