use crate::aabb::*;
use crate::triangle::*;
use crate::ray::*;
use cglinalg::{
    Vector3,
};


#[derive(Copy, Clone, Debug, PartialEq)]
struct Bin {
    bounding_box: Aabb<f32>,
    primitive_count: usize,
}

impl Bin {
    fn new(bounding_box: Aabb<f32>, primitive_count: usize) -> Self {
        Self { bounding_box, primitive_count, }
    }
}

impl Default for Bin {
    fn default() -> Self {
        Self::new(Aabb::default(), 0)
    }
}

#[derive(Clone, Debug)]
struct PrimitiveIter<'a> {
    objects: &'a [Triangle<f32>],
    primitive_count: usize,
    base_primitive_index: usize,
    current_offset: usize,
}

impl<'a> PrimitiveIter<'a> {
    fn new(objects: &'a [Triangle<f32>], primitive_count: usize, base_primitive_index: usize) -> Self {
        Self {
            objects,
            primitive_count,
            base_primitive_index,
            current_offset: 0,
        }
    }
}

impl<'a> Iterator for PrimitiveIter<'a> {
    type Item = &'a Triangle<f32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_offset < self.primitive_count {
            let current_primitive_index = self.base_primitive_index + self.current_offset;
            let current_object = &self.objects[current_primitive_index];
            self.current_offset += 1;
            
            return Some(current_object);
        }

        None
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct BvhNode {
    aabb: Aabb<f32>,
    left_node: usize,
    first_primitive_idx: usize,
    primitive_count: usize,
}

impl BvhNode {
    #[inline]
    pub fn is_leaf(&self) -> bool {
        self.primitive_count > 0
    }

    #[inline]
    pub fn is_branch(&self) -> bool {
        self.primitive_count == 0
    }

    #[inline]
    pub fn left_node(&self) -> usize {
        self.left_node
    }

    #[inline]
    pub fn right_node(&self) -> usize {
        self.left_node + 1
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
    fn primitive_iter<'a>(&self, objects: &'a [Triangle<f32>], node: &BvhNode) -> PrimitiveIter<'a> {
        let base_primitive_index = self.node_indices[node.first_primitive_idx];
        
        PrimitiveIter::new(objects, node.primitive_count, base_primitive_index)
    }

    fn intersect_subtree(&self, objects: &[Triangle<f32>], ray: &Ray<f32>, node_idx: usize) -> Option<f32> {    
        let mut current_node = &self.nodes[node_idx];
        let mut stack = vec![];
        let mut closest_ray = *ray;
        loop {
            if current_node.is_leaf() {
                for primitive in self.primitive_iter(objects, current_node) {
                    if let Some(t_intersect) = primitive.intersect(&closest_ray) {
                        closest_ray.t = t_intersect;
                    }
                }

                if !stack.is_empty() {
                    current_node = stack.pop().unwrap();
                } else {
                    break;
                }
            } else {
                let (closest_child, closest_dist, farthest_child, farthest_dist) = {
                    let left_child = &self.nodes[current_node.left_node()];
                    let right_child = &self.nodes[current_node.right_node()];
                    let left_child_dist = left_child.aabb.intersect(&closest_ray);
                    let right_child_dist = right_child.aabb.intersect(&closest_ray);
                    if left_child_dist.unwrap_or(f32::MAX) < right_child_dist.unwrap_or(f32::MAX) {
                        (left_child, left_child_dist, right_child, right_child_dist)
                    } else {
                        (right_child, right_child_dist, left_child, left_child_dist)
                    }
                };

                if closest_dist.is_some() {
                    current_node = closest_child;
                    if farthest_dist.is_some() {
                        stack.push(farthest_child);
                    }

                    continue;
                }
                    
                if !stack.is_empty() {
                    current_node = stack.pop().unwrap();
                } else {
                    break;
                }
            }
        }

        if closest_ray.t < f32::MAX { Some(closest_ray.t) } else { None }
    }


    pub fn intersect(&self, objects: &[Triangle<f32>], ray: &Ray<f32>) -> Option<f32> {
        self.intersect_subtree(objects, ray, self.root_node_idx)
    }

    /// Returns the number of nodes in the boundary volume hierarchy.
    #[inline]
    pub const fn nodes_used(&self) -> usize {
        self.nodes_used
    }
}

#[derive(Clone, Debug)]
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

    fn primitive_iter<'a>(&self, objects: &'a [Triangle<f32>], node: &BvhNode) -> PrimitiveIter<'a> {
        let base_primitive_index = self.partial_bvh.node_indices[node.first_primitive_idx];
        
        PrimitiveIter::new(objects, node.primitive_count, base_primitive_index)
    }

    fn update_node_bounds(&mut self, objects: &mut [Triangle<f32>], node_idx: usize) {
        // NOTE: We use local implementations of min and max of vector components here because the
        // compiler does not seem to want to inline it here.
        #[inline] fn __min(this: &Vector3<f32>, that: &Vector3<f32>) -> Vector3<f32> { 
            // this.component_min(that)
            Vector3::new(
                f32::min(this.x, that.x),
                f32::min(this.y, that.y),
                f32::min(this.z, that.z),
            )
        }

        #[inline] fn __max(this: &Vector3<f32>, that: &Vector3<f32>) -> Vector3<f32> {
            // this.component_max(that)
            Vector3::new(
                f32::max(this.x, that.x),
                f32::max(this.y, that.y),
                f32::max(this.z, that.z),
            )
        }

        let it = self.primitive_iter(objects, &self.partial_bvh.nodes[node_idx]);
        let node = &mut self.partial_bvh.nodes[node_idx];
        node.aabb = Aabb::new(Vector3::from_fill(f32::MAX), Vector3::from_fill(-f32::MAX));
        for primitive in it {
            node.aabb.bounds_min = __min(&node.aabb.bounds_min, &primitive.vertex0);
            node.aabb.bounds_min = __min(&node.aabb.bounds_min, &primitive.vertex1);
            node.aabb.bounds_min = __min(&node.aabb.bounds_min, &primitive.vertex2);
            node.aabb.bounds_max = __max(&node.aabb.bounds_max, &primitive.vertex0);
            node.aabb.bounds_max = __max(&node.aabb.bounds_max, &primitive.vertex1);
            node.aabb.bounds_max = __max(&node.aabb.bounds_max, &primitive.vertex2);
        }
    }
    
    fn find_best_split_plane(&self, objects: &[Triangle<f32>], node: &BvhNode) -> (isize, f32, f32) {
        const BIN_COUNT: usize = 8;
        let mut best_axis = -1;
        let mut best_position = 0_f32;
        let mut best_cost = f32::MAX;
        for axis in 0..3 {
            let mut bounds_min = 1e30;
            let mut bounds_max = 1e-30;
            for primitive in self.primitive_iter(objects, node) {
                bounds_min = f32::min(bounds_min, primitive.centroid[axis]);
                bounds_max = f32::max(bounds_max, primitive.centroid[axis]);
            }
            if bounds_min == bounds_max {
                continue;
            }

            let mut bins = [Bin::default(); BIN_COUNT];
            let scale = (BIN_COUNT as f32) / (bounds_max - bounds_min);
            for primitive in self.primitive_iter(objects, node) {
                let possible_bin_index = ((primitive.centroid[axis] - bounds_min) * scale) as usize;
                let bin_index = usize::min(BIN_COUNT - 1, possible_bin_index);
                bins[bin_index].primitive_count += 1;
                bins[bin_index].bounding_box.grow(&primitive.vertex0);
                bins[bin_index].bounding_box.grow(&primitive.vertex1);
                bins[bin_index].bounding_box.grow(&primitive.vertex2);
            }

            // Assemble the data for calculating the `BIN_COUNT - 1` planes between the `BIN_COUNT` bins.
            let mut left_area = [0_f32; BIN_COUNT - 1];
            let mut right_area = [0_f32; BIN_COUNT - 1];
            let mut left_count = [0; BIN_COUNT - 1];
            let mut right_count = [0; BIN_COUNT - 1];
            let mut left_box = Aabb::default();
            let mut right_box = Aabb::default();
            let mut left_sum = 0;
            let mut right_sum = 0;
            for i in 0..(BIN_COUNT - 1) {
                left_sum += bins[i].primitive_count;
                left_count[i] = left_sum;
                left_box.grow_aabb(&bins[i].bounding_box);
                left_area[i] = left_box.area();
    
                right_sum += bins[BIN_COUNT - 1 - i].primitive_count;
                right_count[BIN_COUNT - 2 - i] = right_sum;
                right_box.grow_aabb(&bins[BIN_COUNT - 1 - i].bounding_box);
                right_area[BIN_COUNT - 2 - i] = right_box.area();
            }

            // Calculate the SAH cost for the seven planes.
            let scale = (bounds_max - bounds_min) / (BIN_COUNT as f32);
            for i in 0..(BIN_COUNT - 1) {
                let plane_cost = (left_count[i] as f32) * left_area[i] + (right_count[i] as f32) * right_area[i];
                if plane_cost < best_cost {
                    best_axis = axis as isize;
                    best_position = bounds_min + scale * ((i + 1) as f32);
                    best_cost = plane_cost;
                }
            }
        }


        (best_axis, best_position, best_cost)
    }

    #[inline]
    fn calculate_node_cost(&self, node: &BvhNode) -> f32 {
        let extent = node.aabb.extent();
        let parent_area = extent.x * extent.y + extent.y * extent.z + extent.z * extent.x;
        let primitive_count = node.primitive_count as f32;

        primitive_count * parent_area
    }

    fn subdivide(&mut self, objects: &mut [Triangle<f32>], node_idx: usize) {
        // Terminate recursion.
        let (best_axis, best_position, best_cost) = {
            let node = &self.partial_bvh.nodes[node_idx];
            self.find_best_split_plane(objects, node)
        };
        let (left_count, i) = {
            let node = &self.partial_bvh.nodes[node_idx];
            let axis = best_axis as usize;
            let split_position = best_position;
            let no_split_cost = self.calculate_node_cost(node);
            if best_cost >= no_split_cost {
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

    pub fn build_for(mut self, objects: &mut [Triangle<f32>]) -> Bvh {
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
    objects: Vec<Triangle<f32>>,
    bvh_builder: BvhBuilder
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bvh_builder: BvhBuilder::new(),
        }
    }

    pub fn with_objects(mut self, objects: Vec<Triangle<f32>>) -> Self {
        self.objects = objects;

        self
    }

    pub fn build(mut self) -> Scene {
        let bvh = self.bvh_builder.build_for(&mut self.objects);

        Scene::new(self.objects, bvh)
    }
}


pub struct Scene {
    pub objects: Vec<Triangle<f32>>,
    pub bvh: Bvh,
}

impl Scene {
    pub fn new(objects: Vec<Triangle<f32>>, bvh: Bvh) -> Self {
        Self { objects, bvh, }
    }

    pub fn intersect(&self, ray: &Ray<f32>) -> Option<f32> {
        self.bvh.intersect(&self.objects, ray)
    }
}

