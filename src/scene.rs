use cglinalg::{
    Vector3,
    SimdScalarFloat,
};
use crate::triangle::*;


#[derive(Copy, Clone, Debug, PartialEq, Default)]
struct Aabb {
    b_min: Vector3<f32>,
    b_max: Vector3<f32>,
}

impl Aabb {
    fn new(b_min: Vector3<f32>, b_max: Vector3<f32>) -> Self {
        Self { b_min, b_max, }
    }

    fn grow(&mut self, position: &Vector3<f32>) {
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

        self.b_min = min(&self.b_min, position);
        self.b_max = max(&self.b_max, position);
    }

    fn area(&self) -> f32 {
        let extent = self.extent();

        extent.x * extent.y + extent.y * extent.z + extent.z * extent.x
    }

    #[inline]
    fn extent(&self) -> Vector3<f32> {
        self.b_max - self.b_min
    }

    fn intersect(&self, ray: &Ray) -> f32 {
        let t_x1 = (self.b_min.x - ray.origin.x) * ray.recip_direction.x;
        let t_x2 = (self.b_max.x - ray.origin.x) * ray.recip_direction.x;
        let t_min = f32::min(t_x1, t_x2);
        let t_max = f32::max(t_x1, t_x2);
        let t_y1 = (self.b_min.y - ray.origin.y) * ray.recip_direction.y; 
        let t_y2 = (self.b_max.y - ray.origin.y) * ray.recip_direction.y;
        let t_min = f32::max(t_min, f32::min(t_y1, t_y2)); 
        let t_max = f32::min(t_max, f32::max(t_y1, t_y2));
        let t_z1 = (self.b_min.z - ray.origin.z) * ray.recip_direction.z;
        let t_z2 = (self.b_max.z - ray.origin.z) * ray.recip_direction.z;
        let t_min = f32::max(t_min, f32::min(t_z1, t_z2)); 
        let t_max = f32::min(t_max, f32::max(t_z1, t_z2));
        
        if (t_max >= t_min) && (t_min < ray.t) && (t_max > 0_f32) {
            t_min
        } else {
            f32::MAX
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct BvhNode {
    aabb: Aabb,
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

#[derive(Clone, Debug)]
pub struct Bvh {
    nodes: Vec<BvhNode>,
    node_indices: Vec<usize>,
    root_node_idx: usize,
    nodes_used: usize,
}

impl Bvh {
    fn intersect_subtree_recursive(&self, objects: &[Triangle], ray: &Ray, node_idx: usize) -> Option<Ray> {
        let node = &self.nodes[node_idx];
        if node.aabb.intersect(ray) == f32::MAX {
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
            if let Some(new_ray) = self.intersect_subtree_recursive(objects, ray, node.left_node) {
                Some(new_ray) 
            } else if let Some(new_ray) = self.intersect_subtree_recursive(objects, ray, node.left_node + 1) {
                Some(new_ray)
            } else {
                return None;
            }
        }
    }

    pub fn intersect_recursive(&self, objects: &[Triangle], ray: &Ray) -> Option<Ray> {
        self.intersect_subtree_recursive(objects, ray, self.root_node_idx)
    }

    fn intersect_subtree(&self, objects: &[Triangle], ray: &Ray, node_idx: usize) -> Option<Ray> {
        let mut node = &self.nodes[node_idx];
        let mut stack = vec![];
        let mut best_ray = *ray;
        loop {
            if node.is_leaf() {
                for i in 0..node.primitive_count {
                    let primitive_idx = self.node_indices[node.first_primitive_idx];
                    let primitive = objects[primitive_idx + i];
                    if let Some(intersected_ray) = primitive.intersect(&best_ray) {
                        best_ray = intersected_ray;
                    }
                }
                if stack.is_empty() {
                    break;
                } else {
                    node = stack.pop().unwrap();
                }

                continue;
            }
            let mut child1 = &self.nodes[node.left_node];
            let mut child2 = &self.nodes[node.left_node + 1];
            let mut dist1 = child1.aabb.intersect(&best_ray);
            let mut dist2 = child2.aabb.intersect(&best_ray);
            if dist1 > dist2 { 
                let dist_temp = dist1;
                let child_temp = child1;
                dist1 = dist2;
                dist2 = dist_temp;
                child1 = child2;
                child2 = child_temp;

            }
            if dist1 == f32::MAX {
                if stack.is_empty() {
                    break;
                } else {
                    node = stack.pop().unwrap();
                }
            } else {
                node = child1;
                if dist2 != f32::MAX {
                    stack.push(child2);
                }
            }
        }

        Some(best_ray)
    }


    pub fn intersect(&self, objects: &[Triangle], ray: &Ray) -> Option<Ray> {
        self.intersect_subtree(objects, ray, self.root_node_idx)
    }

    /// Returns the number of nodes in the boundary volume hierarchy.
    #[inline]
    pub const fn nodes_used(&self) -> usize {
        self.nodes_used
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
            node.aabb = Aabb::new(Vector3::from_fill(f32::MAX), Vector3::from_fill(-f32::MAX));
            let first = node.first_primitive_idx;

            first
        };

        for i in 0..self.partial_bvh.nodes[node_idx].primitive_count {
            let leaf_triangle_idx = self.partial_bvh.node_indices[first + i];
            let leaf_triangle = &objects[leaf_triangle_idx];
            let node = &mut self.partial_bvh.nodes[node_idx];
            node.aabb.b_min = min(&node.aabb.b_min, &leaf_triangle.vertex0);
            node.aabb.b_min = min(&node.aabb.b_min, &leaf_triangle.vertex1);
            node.aabb.b_min = min(&node.aabb.b_min, &leaf_triangle.vertex2);
            node.aabb.b_max = max(&node.aabb.b_max, &leaf_triangle.vertex0);
            node.aabb.b_max = max(&node.aabb.b_max, &leaf_triangle.vertex1);
            node.aabb.b_max = max(&node.aabb.b_max, &leaf_triangle.vertex2);
        }
    }

    fn evaluate_sah(&self, objects: &[Triangle], node: &BvhNode, axis: usize, position: f32) -> f32 {
        let mut left_box = Aabb::default();
        let mut right_box = Aabb::default();
        let mut left_count = 0;
        let mut right_count = 0;
        for i in 0..node.primitive_count {
            let primitive_idx = self.partial_bvh.node_indices[node.first_primitive_idx + i];
            let primitive = &objects[primitive_idx];
            if primitive.centroid[axis] < position {
                left_count += 1;
                left_box.grow(&primitive.vertex0);
                left_box.grow(&primitive.vertex1);
                left_box.grow(&primitive.vertex2);
            } else {
                right_count += 1;
                right_box.grow(&primitive.vertex0);
                right_box.grow(&primitive.vertex1);
                right_box.grow(&primitive.vertex2);
            }
        }
        let cost = (left_count as f32) * left_box.area() + (right_count as f32) * right_box.area();
        
        if cost > 0_f32 { cost } else { f32::MAX }
    }

    fn subdivide_recursive(&mut self, objects: &mut [Triangle], node_idx: usize) {
        println!("Subdividing node_idx = {}", node_idx);
        // Terminate recursion.
        let (best_axis, best_position) = {
            let node = &mut self.partial_bvh.nodes[node_idx];
            // Determine the split axis using the midpoint of the boundary volume
            if node.primitive_count <= 2 {
                return;
            }
            // Determine the split axis and position using the bounding volume's midpoint.
            // let extent = node.aabb_max - node.aabb_min;
            let extent = node.aabb.extent();
            let mut best_axis = 0;
            if extent.y > extent.x {
                best_axis = 1;
            } 
            if extent.z > extent[best_axis] {
                best_axis = 2;
            }
            let best_position = node.aabb.b_min[best_axis] + extent[best_axis] * (1_f32 / 2_f32);

            (best_axis, best_position)
        };
        let (left_count, i) = {
            let node = &mut self.partial_bvh.nodes[node_idx];
            let axis = best_axis as usize;
            let split_position = best_position;
            // In-place partition.
            let mut i = node.first_primitive_idx;
            let mut j = i + node.primitive_count - 1;
            while i <= j {
                if objects[i].centroid[axis] < split_position {
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
            let nodes_used = self.partial_bvh.nodes_used;
            self.partial_bvh.nodes_used += 1;
            nodes_used
        };
        let right_child_idx = {
            let nodes_used = self.partial_bvh.nodes_used;
            self.partial_bvh.nodes_used += 1;
            nodes_used
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

    fn subdivide(&mut self, objects: &mut [Triangle], node_idx: usize) {
        println!("Subdividing node_idx = {}", node_idx);
        // Terminate recursion.
        let (best_axis, best_position, best_cost) = {
            // Determine the split axis using the surface area heuristic (SAH).
            let node = self.partial_bvh.nodes[node_idx];
            let mut best_axis = -1;
            let mut best_position = 0_f32;
            let mut best_cost = f32::MAX;
            for axis in 0..3 {
                for i in 0..node.primitive_count {
                    let primitive_idx = self.partial_bvh.node_indices[node.first_primitive_idx + i];
                    let primitive = &objects[primitive_idx];
                    let candidate_position = primitive.centroid[axis as usize];
                    let cost = self.evaluate_sah(objects, &node, axis, candidate_position);
                    if cost < best_cost {
                        best_position = candidate_position;
                        best_axis = axis as isize;
                        best_cost = cost;
                    }
                }
            }

            (best_axis, best_position, best_cost)
        };
        let (left_count, i) = {
            let node = &mut self.partial_bvh.nodes[node_idx];
            let axis = best_axis as usize;
            let split_position = best_position;
            // let extent = node.aabb_max - node.aabb_min;
            let extent = node.aabb.extent();
            let parent_area = extent.x * extent.y + extent.y * extent.z + extent.z * extent.x;
            let parent_cost = (node.primitive_count as f32) * parent_area;
            if best_cost >= parent_cost {
                return;
            }

            // In-place partition.
            let mut i = node.first_primitive_idx;
            let mut j = i + node.primitive_count - 1;
            while i <= j {
                if objects[i].centroid[axis] < split_position {
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
            let nodes_used = self.partial_bvh.nodes_used;
            self.partial_bvh.nodes_used += 1;
            nodes_used
        };
        let right_child_idx = {
            let nodes_used = self.partial_bvh.nodes_used;
            self.partial_bvh.nodes_used += 1;
            nodes_used
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
            self.partial_bvh.node_indices.push(i);
        }

        self.partial_bvh.nodes = vec![BvhNode::default(); 2 * objects.len()];
        
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
        self.bvh.intersect(&self.objects, ray)
    }

    pub fn intersect_recursive(&self, ray: &Ray) -> Option<Ray> {
        self.bvh.intersect_recursive(&self.objects, ray)
    }
}

