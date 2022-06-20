use cglinalg::{
    Vector3,
};
use crate::bvhtracer::*;


fn intersect_aabb(ray: &Ray, aabb_min: &Vector3<f32>, aabb_max: &Vector3<f32>) -> bool {
    let t_x1 = (aabb_min.x - ray.origin.x) / ray.direction.x;
    let t_x2 = (aabb_max.x - ray.origin.x) / ray.direction.x;
    let t_min = f32::min(t_x1, t_x2);
    let t_max = f32::max(t_x1, t_x2);
    let t_y1 = (aabb_min.y - ray.origin.y) / ray.direction.y; 
    let t_y2 = (aabb_max.y - ray.origin.y) / ray.direction.y;
    let t_min = f32::max(t_min, f32::min(t_y1, t_y2)); 
    let t_max = f32::min(t_max, f32::max(t_y1, t_y2));
    let t_z1 = (aabb_min.z - ray.origin.z) / ray.direction.z;
    let t_z2 = (aabb_max.z - ray.origin.z) / ray.direction.z;
    let t_min = f32::max(t_min, f32::min(t_z1, t_z2)); 
    let t_max = f32::min(t_max, f32::max(t_z1, t_z2));
    
    (t_max >= t_min) && (t_min < ray.t) && (t_max > 0_f32)
}


#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct BvhNode {
    aabb_min: Vector3<f32>,
    aabb_max: Vector3<f32>,
    left_node: usize,
    first_primitive_idx: usize,
    primitive_count: usize,
}

impl BvhNode {
    #[inline]
    pub fn is_leaf(&self) -> bool {
        self.primitive_count > 0
    }
}

pub struct Bvh {
    nodes: Vec<BvhNode>,
    node_indices: Vec<usize>,
    root_node_idx: usize,
    nodes_used: usize,
}

impl Bvh {
    pub fn intersect(&self, objects: &[Triangle], ray: &Ray, node_idx: usize) -> Option<Ray> {
        let node = &self.nodes[node_idx];
        if !intersect_aabb(ray, &node.aabb_min, &node.aabb_max) {
            return None;
        }
        if node.is_leaf() {
            for i in 0..node.primitive_count {
                let primitive_idx = self.node_indices[node.first_primitive_idx + i];
                if let Some(new_ray) = objects[primitive_idx].intersect(ray) {
                    return Some(new_ray);
                }
            }

            None
        } else {
            if let Some(new_ray) = self.intersect(objects, ray, node.left_node) {
                Some(new_ray) 
            } else if let Some(new_ray) = self.intersect(objects, ray, node.left_node + 1) {
                Some(new_ray)
            } else {
                return None;
            }
        }
    }
}

pub struct BvhBuilder {
    partial_bvh: Bvh,
}

impl BvhBuilder {
    pub fn new() -> Self {
        let nodes = vec![];
        let node_indices = vec![];
        let root_node_idx = 0;
        let nodes_used = 1;

        let partial_bvh = Bvh { nodes, node_indices, root_node_idx, nodes_used, };

        Self { partial_bvh, }
    }

    fn update_node_bounds(&mut self, objects: &mut [Triangle], node_idx: usize) {
        #[inline]
        fn min(vector1: &Vector3<f32>, vector2: &Vector3<f32>) -> Vector3<f32> {
            Vector3::new(
                f32::min(vector1.x, vector2.x),
                f32::min(vector1.y, vector2.y),
                f32::min(vector1.z, vector2.z),
            )
        }

        #[inline]
        fn max(vector1: &Vector3<f32>, vector2: &Vector3<f32>) -> Vector3<f32> {
            Vector3::new(
                f32::max(vector1.x, vector2.x),
                f32::max(vector1.y, vector2.y),
                f32::max(vector1.z, vector2.z),
            )
        }

        let first = {
            let mut node = &mut self.partial_bvh.nodes[node_idx];
            node.aabb_min = Vector3::from_fill(f32::MAX); 
            node.aabb_max = Vector3::from_fill(-f32::MAX);
            let first = node.first_primitive_idx;

            first
        };

        for i in 0..self.partial_bvh.nodes[node_idx].primitive_count {
            let leaf_triangle_idx = self.partial_bvh.node_indices[first + i];
            let leaf_triangle = &objects[leaf_triangle_idx];
            let node = &mut self.partial_bvh.nodes[node_idx];
            node.aabb_min = min(&node.aabb_min, &leaf_triangle.vertex0);
            node.aabb_min = min(&node.aabb_min, &leaf_triangle.vertex1);
            node.aabb_min = min(&node.aabb_min, &leaf_triangle.vertex2);
            node.aabb_max = max(&node.aabb_max, &leaf_triangle.vertex0);
            node.aabb_max = max(&node.aabb_max, &leaf_triangle.vertex1);
            node.aabb_max = max(&node.aabb_max, &leaf_triangle.vertex2);
        }
    }

    fn subdivide(&mut self, objects: &mut [Triangle], node_idx: usize) {
        // Terminate recursion.
        let (left_count, i) = {
            let node = &mut self.partial_bvh.nodes[node_idx];
            if node.primitive_count <= 2 {
                return;
            }
            // Determine the split axis and position.
            let extent = node.aabb_max - node.aabb_min;
            let mut axis = 0;
            if extent.y > extent.x {
                axis = 1;
            } 
            if extent.z > extent[axis] {
                axis = 2;
            }
            let split_pos = node.aabb_min[axis] + extent[axis] * (1_f32 / 2_f32);
            // In-place partition.
            let mut i = node.first_primitive_idx;
            let mut j = i + node.primitive_count - 1;
            while i <= j {
                if objects[i].centroid[axis] < split_pos {
                    i += 1;
                } else {
                    objects.swap(i, j);
                    j -= 1;
                }
            }

            // Abort split if one of the sides is empty.
            let left_count = i - node.first_primitive_idx;
            if left_count == 0 || left_count == node.primitive_count {
                return;
            }

            (left_count, i)
        };
        // Create child nodes.
        let left_child_idx = {
            self.partial_bvh.nodes_used += 1;
            self.partial_bvh.nodes_used
        };
        let right_child_idx = {
            self.partial_bvh.nodes_used += 1;
            self.partial_bvh.nodes_used
        };
        {
            self.partial_bvh.nodes[left_child_idx].first_primitive_idx = self.partial_bvh.nodes[node_idx].first_primitive_idx;
            self.partial_bvh.nodes[left_child_idx].primitive_count = left_count;
            self.partial_bvh.nodes[right_child_idx].first_primitive_idx = i;
            self.partial_bvh.nodes[right_child_idx].primitive_count = self.partial_bvh.nodes[node_idx].primitive_count - left_count;
        }
        {
            let node = &mut self.partial_bvh.nodes[node_idx];
            node.left_node = left_child_idx;
            node.primitive_count = 0;
        }

        self.update_node_bounds(objects, left_child_idx);
        self.update_node_bounds(objects, right_child_idx);
        // Recurse
        self.subdivide(objects, left_child_idx);
        self.subdivide(objects, right_child_idx);
    }

    pub fn build_for(mut self, objects: &mut [Triangle]) -> Bvh {
        // Populate the triangle index array.
        for i in 0..objects.len() {
            // self.partial_bvh.node_indices[i] = i;
            self.partial_bvh.node_indices.push(i);
        }

        self.partial_bvh.nodes = vec![BvhNode::default(); 2 * objects.len()];
        
        for object in objects.iter_mut() {
            // TODO: construct centroid when constructing triangle object instead of doing it 
            // directly in the builder.
            let one_third = 1_f32 / 3_f32;
            object.centroid = (object.vertex0 + object.vertex1 + object.vertex2) * one_third;
        }
        // let len_nodes = self.partial_bvh.nodes.len();
        let root_node_idx = self.partial_bvh.root_node_idx;
        let mut root_node: &mut BvhNode = &mut self.partial_bvh.nodes[root_node_idx];
        root_node.left_node = 0;
        root_node.primitive_count = objects.len();

        self.update_node_bounds(objects, self.partial_bvh.root_node_idx);
        self.subdivide(objects, self.partial_bvh.root_node_idx);

        self.partial_bvh
    }
}


pub struct SceneBuilder {
    objects: Vec<Triangle>,
    bvh_builder: BvhBuilder
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bvh_builder: BvhBuilder::new(),
        }
    }

    pub fn with_objects(mut self, objects: Vec<Triangle>) -> Self {
        self.objects = objects;

        self
    }

    pub fn build(mut self) -> Scene {
        let bvh = self.bvh_builder.build_for(&mut self.objects);

        Scene::new(self.objects, bvh)
    }
}


pub struct Scene {
    pub objects: Vec<Triangle>,
    pub bvh: Bvh,
}

impl Scene {
    pub fn new(objects: Vec<Triangle>, bvh: Bvh) -> Self {
        Self { objects, bvh, }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Ray> {
        self.bvh.intersect(&self.objects, ray, self.bvh.root_node_idx)
    }
}
