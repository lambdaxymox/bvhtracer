extern crate bvhtracer;
extern crate bvhtracer_demos;
extern crate cglinalg;


const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 640;


use bvhtracer::*;
use bvhtracer_demos::*;
use cglinalg::{
    Vector3,
};
use std::io;


struct AppStateBigBenClock {
    active_scene: Scene,
    r: f32,
    originals: Vec<Triangle<f32>>,
}

impl AppStateBigBenClock {
    fn new() -> Self {
        let focal_offset = 1.5;
        let model_spec = NdcBoxSpec::new(
            -1_f32, 
            1_f32, 
            -1_f32 + focal_offset, 
            1_f32 + focal_offset, 
            2_f32, 
            10000_f32, 
        );
        let attitude_spec = CameraAttitudeSpec::new(
            Vector3::new(0_f32, 0_f32, -4.5_f32),
            Vector3::unit_z(),
            Vector3::unit_x(),
            Vector3::unit_y(),
            Vector3::unit_z()
        );
        let camera = Camera::new(&model_spec, &attitude_spec);
        let mesh_file = include_bytes!("assets/bigben.tri");
        let mesh_decoder = TriMeshDecoder::new(io::Cursor::new(mesh_file));
        let mesh = mesh_decoder.read_mesh().unwrap();
        let model_builder = ModelBuilder::new();
        let model = model_builder.with_mesh(mesh).build();
        let object = SceneObjectBuilder::new(model)
            .build();
        let active_scene = SceneBuilder::new(camera)
            .with_object(object)
            .build();
        let r = 0_f32;
        let originals = active_scene.get_unchecked(0)
            .model()
            .model()
            .borrow()
            .primitives()
            .iter()
            .map(|p| *p)
            .collect::<Vec<_>>();

        Self { active_scene, r, originals, }
    }

    fn animate(&mut self) {
        self.r += 0.05;
        if self.r > std::f32::consts::FRAC_2_PI {
            self.r -= std::f32::consts::FRAC_2_PI;
        }
        let a = f32::sin(self.r) * 0.5;
        let mesh = self.active_scene().get_unchecked(0).model().model();
        for i in 0..self.originals.len() {
            let o_0 = self.originals[i].vertices[0];
            let s_0 = a * (o_0.y - 0.2) * 0.2;
            let x_0 = o_0.x * f32::cos(s_0) - o_0.y * f32::sin(s_0);
            let y_0 = o_0.x * f32::sin(s_0) + o_0.y * f32::cos(s_0);

            let o_1 = self.originals[i].vertices[1];
            let s_1 = a * (o_1.y - 0.2) * 0.2;
            let x_1 = o_1.x * f32::cos(s_1) - o_1.y * f32::sin(s_1);
            let y_1 = o_1.x * f32::sin(s_1) + o_1.y * f32::cos(s_1);

            let o_2 = self.originals[i].vertices[2];
            let s_2 = a * (o_2.y - 0.2) * 0.2;
            let x_2 = o_2.x * f32::cos(s_2) - o_2.y * f32::sin(s_2);
            let y_2 = o_2.x * f32::sin(s_2) + o_2.y * f32::cos(s_2);

            mesh.borrow_mut().primitives_mut()[i] = Triangle::new(
                Vector3::new(x_0, y_0, o_0.z),
                Vector3::new(x_1, y_1, o_1.z),
                Vector3::new(x_2, y_2, o_2.z),
            );
        }
    }
}

impl AppState for AppStateBigBenClock {
    fn update(&mut self, _elapsed: f64) {
        self.animate();
        self.active_scene.get_mut_unchecked(0).model().refit();
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
    let state = Box::new(AppStateBigBenClock::new());
    let elapsed = now.elapsed().unwrap();
    println!("Scene building time = {:?}", elapsed);

    let accumulator = Box::new(IntersectionAccumulator::new(
        Vector3::from_fill(1_f32), 
        Vector3::zero()
    ));
    let pixel_shader = Box::new(IntersectionShader::new(
        Rgba::new(255, 255, 255, 255), 
        Rgba::new(0, 0, 0, 255),
    ));
    let renderer = Renderer::new(Box::new(PathTracer::new()));
    let context = init_gl("OpenGL Window", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).unwrap();
    let mut app = App::new(context, pixel_shader, accumulator, state, renderer, SCREEN_WIDTH, SCREEN_HEIGHT);
    app.run();

    Ok(())
}

