use crate::aabb::*;
use crate::triangle::*;
use crate::ray::*;
use cglinalg::{
    Vector3,
};

use std::fmt;
use std::ops;


#[derive(Copy, Clone, Debug, PartialEq)]
struct Bin {
    bounding_box: Aabb<f32>,
    primitive_count: u32,
}

impl Bin {
    fn new(bounding_box: Aabb<f32>, primitive_count: u32) -> Self {
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
    primitive_count: u32,
    base_primitive_index: u32,
    current_offset: u32,
}

impl<'a> PrimitiveIter<'a> {
    fn new(objects: &'a [Triangle<f32>], primitive_count: u32, base_primitive_index: u32) -> Self {
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
            let current_object = &self.objects[current_primitive_index as usize];
            self.current_offset += 1;
            
            return Some(current_object);
        }

        None
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
struct BvhLeafNode {
    aabb: Aabb<f32>,
    primitive_count: u32,
    first_primitive_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
struct BvhBranchNode {
    aabb: Aabb<f32>,
    primitive_count: u32,
    left_node: u32,
}

impl BvhBranchNode {
    #[inline]
    pub fn left_node(&self) -> u32 {
        self.left_node
    }

    #[inline]
    pub fn right_node(&self) -> u32 {
        self.left_node + 1
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
union BranchOrLeafData {
    left_node: u32,
    first_primitive_index: u32,
}

impl Default for BranchOrLeafData {
    fn default() -> Self {
        BranchOrLeafData { first_primitive_index: 0 }
    }
}

#[derive(Copy, Clone, Default)]
pub struct BvhNode {
    aabb: Aabb<f32>,
    primitive_count: u32,
    _left_first: BranchOrLeafData,
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
    const fn as_leaf(&self) -> &BvhLeafNode {
        use std::mem;
        unsafe { 
            mem::transmute::<&BvhNode, &BvhLeafNode>(self)
        }
    }

    #[inline]
    const fn as_branch(&self) -> &BvhBranchNode {
        use std::mem;
        unsafe { 
            mem::transmute::<&BvhNode, &BvhBranchNode>(self)
        }
    }

    #[inline]
    fn as_mut_leaf(&mut self) -> &mut BvhLeafNode {
        use std::mem;
        unsafe { 
            mem::transmute::<&mut BvhNode, &mut BvhLeafNode>(self)
        }
    }

    #[inline]
    fn as_mut_branch(&mut self) -> &mut BvhBranchNode {
        use std::mem;
        unsafe { 
            mem::transmute::<&mut BvhNode, &mut BvhBranchNode>(self)
        }
    }
}

impl fmt::Debug for BvhNode {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if self.is_leaf() {
            <BvhLeafNode as fmt::Debug>::fmt(self.as_leaf(), formatter)
        } else {
            <BvhBranchNode as fmt::Debug>::fmt(self.as_branch(), formatter)
        }
    }
}

impl PartialEq for BvhNode {
    fn eq(&self, other: &Self) -> bool {
        if self.is_leaf() && other.is_leaf() {
            return self.as_leaf() == other.as_leaf();
        }

        if self.is_branch() && other.is_branch() {
            return self.as_branch() == other.as_branch();
        }

        false
    }
}

#[derive(Clone, Debug)]
struct BvhNodeArray(Vec<BvhNode>);

impl BvhNodeArray {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl ops::Index<u32> for BvhNodeArray {
    type Output = BvhNode;

    #[inline]
    fn index(&self, _index: u32) -> &Self::Output {
        self.0.index(_index as usize)
    }
    
}

impl ops::IndexMut<u32> for BvhNodeArray {
    #[inline]
    fn index_mut(&mut self, _index: u32) -> &mut Self::Output {
        self.0.index_mut(_index as usize)
    }
}

#[derive(Clone, Debug)]
pub struct Bvh {
    nodes: BvhNodeArray,
    node_indices: Vec<u32>,
    root_node_index: u32,
    nodes_used: u32,
}

impl Bvh {
    fn primitive_iter<'a>(&self, objects: &'a [Triangle<f32>], node: &BvhNode) -> PrimitiveIter<'a> {
        let base_primitive_index = self.node_indices[node.as_leaf().first_primitive_index as usize];
        
        PrimitiveIter::new(objects, node.primitive_count, base_primitive_index)
    }

    fn intersect_subtree(&self, objects: &[Triangle<f32>], ray: &Ray<f32>, node_index: u32) -> Option<f32> {    
        let mut current_node = &self.nodes[node_index];
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
                    let left_child = &self.nodes[current_node.as_branch().left_node()];
                    let right_child = &self.nodes[current_node.as_branch().right_node()];
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
        self.intersect_subtree(objects, ray, self.root_node_index)
    }

    /// Returns the number of nodes in the boundary volume hierarchy.
    #[inline]
    pub const fn nodes_used(&self) -> usize {
        self.nodes_used as usize
    }

    fn update_node_bounds(&mut self, objects: &[Triangle<f32>], node_index: u32) {
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

        let it = self.primitive_iter(objects, &self.nodes[node_index]);
        let node = &mut self.nodes[node_index];
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

    pub fn refit(&mut self, objects: &[Triangle<f32>]) {
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

        for i in 0..self.nodes_used {
            let node_index = (self.nodes_used - 1) - i;
            {
                let node = &self.nodes[node_index];
                if node.is_leaf() {
                    // Leaf node: adjust bounds to contained triangles.
                    self.update_node_bounds(objects, node_index as u32);
                    continue;
                }
            }

            // Interior node: adjust bounds to child node bounds.
            let left_child_aabb = { 
                let node = &self.nodes[node_index];
                self.nodes[node.as_branch().left_node()].aabb
            };
            let right_child_aabb = {
                let node = &self.nodes[node_index];
                self.nodes[node.as_branch().right_node()].aabb
            };
            let mut node = &mut self.nodes[node_index];
            node.aabb.bounds_min = __min(&left_child_aabb.bounds_min, &right_child_aabb.bounds_min);
            node.aabb.bounds_max = __max(&left_child_aabb.bounds_max, &right_child_aabb.bounds_max);
        }
    }
}

#[derive(Clone, Debug)]
pub struct BvhBuilder {
    partial_bvh: Bvh,
}

impl BvhBuilder {
    pub fn new() -> Self {
        let nodes = BvhNodeArray(vec![]);
        let node_indices = vec![];
        let root_node_index = 0;
        let nodes_used = 1;

        let partial_bvh = Bvh { nodes, node_indices, root_node_index, nodes_used, };

        Self { partial_bvh, }
    }

    fn primitive_iter<'a>(&self, objects: &'a [Triangle<f32>], node: &BvhNode) -> PrimitiveIter<'a> {
        let base_primitive_index = self.partial_bvh.node_indices[node.as_leaf().first_primitive_index as usize];
        
        PrimitiveIter::new(objects, node.primitive_count, base_primitive_index as u32)
    }

    fn update_node_bounds(&mut self, objects: &mut [Triangle<f32>], node_index: u32) {
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

        let it = self.primitive_iter(objects, &self.partial_bvh.nodes[node_index]);
        let node = &mut self.partial_bvh.nodes[node_index];
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

    fn subdivide(&mut self, objects: &mut [Triangle<f32>], node_index: u32) {
        // Terminate recursion.
        let (best_axis, best_position, best_cost) = {
            let node = &self.partial_bvh.nodes[node_index];
            self.find_best_split_plane(objects, node)
        };
        let (left_count, i) = {
            let node = &self.partial_bvh.nodes[node_index];
            let axis = best_axis as usize;
            let split_position = best_position;
            let no_split_cost = self.calculate_node_cost(node);
            if best_cost >= no_split_cost {
                return;
            }

            // In-place partition.
            let mut i = node.as_leaf().first_primitive_index;
            let mut j = i + node.primitive_count - 1;
            while i <= j {
                if objects[i as usize].centroid[axis] < split_position {
                    i += 1;
                } else {
                    objects.swap(i as usize, j as usize);
                    j -= 1;
                }
            }

            // Abort split if one of the sides is empty.
            let left_count = i - node.as_leaf().first_primitive_index;
            if left_count == 0 || left_count == node.primitive_count {
                return;
            }

            (left_count, i)
        };
        // Create child nodes.
        let left_child_index = {
            let nodes_used = self.partial_bvh.nodes_used;
            self.partial_bvh.nodes_used += 1;
            nodes_used
        };
        let right_child_index = {
            let nodes_used = self.partial_bvh.nodes_used;
            self.partial_bvh.nodes_used += 1;
            nodes_used
        };
        {
            self.partial_bvh.nodes[left_child_index].as_mut_leaf().first_primitive_index = self.partial_bvh.nodes[node_index].as_leaf().first_primitive_index;
            self.partial_bvh.nodes[left_child_index].primitive_count = left_count;
            self.partial_bvh.nodes[right_child_index].as_mut_leaf().first_primitive_index = i;
            self.partial_bvh.nodes[right_child_index].primitive_count = self.partial_bvh.nodes[node_index].primitive_count - left_count;
        }
        {
            let node = &mut self.partial_bvh.nodes[node_index];
            node.as_mut_branch().left_node = left_child_index;
            node.primitive_count = 0;
        }

        self.update_node_bounds(objects, left_child_index);
        self.update_node_bounds(objects, right_child_index);
        // Recurse
        self.subdivide(objects, left_child_index);
        self.subdivide(objects, right_child_index);
    }

    pub fn build_for(mut self, objects: &mut [Triangle<f32>]) -> Bvh {
        // Populate the triangle index array.
        for i in 0..objects.len() {
            self.partial_bvh.node_indices.push(i as u32);
        }

        self.partial_bvh.nodes = BvhNodeArray(vec![BvhNode::default(); 2 * objects.len()]);
        
        let root_node_index = self.partial_bvh.root_node_index;
        let mut root_node: &mut BvhNode = &mut self.partial_bvh.nodes[root_node_index];
        root_node.as_mut_branch().left_node = 0;
        root_node.primitive_count = objects.len() as u32;

        self.update_node_bounds(objects, self.partial_bvh.root_node_index);
        self.subdivide(objects, self.partial_bvh.root_node_index);

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

    pub fn refit(&mut self) {
        self.bvh.refit(&self.objects)
    }
}


#[cfg(test)]
mod bvh_tests {
    use crate::triangle::*;
    use cglinalg::{
        Vector3,
    };
    
    fn scene() -> super::Scene {
        let displacement_x = Vector3::new(5_f32, 0_f32, 0_f32);
        let displacement_y = Vector3::new(0_f32, 5_f32, 0_f32);
        let triangle0 = Triangle::new(
            Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32),
            Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
            Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
        );
        let triangles = (-100..100).zip(-100..100).map(|(i, j)| {
            Triangle::new(
                triangle0.vertex0 + (i as f32) * displacement_x + (j as f32) * displacement_y,
                triangle0.vertex1 + (i as f32) * displacement_x + (j as f32) * displacement_y,
                triangle0.vertex1 + (i as f32) * displacement_x + (j as f32) * displacement_y,
            )
        })
        .collect::<Vec<Triangle<f32>>>();
        
        let builder = super::SceneBuilder::new();
        
        builder.with_objects(triangles).build()
    }

    #[test]
    fn test_bvh_nodes_used_smaller_than_node_array_length() {
        let scene = scene();
        
        assert!(scene.bvh.nodes.len() > scene.bvh.nodes_used());
    }

    #[test]
    fn test_bvh_nodes_used() {
        let scene = scene();
        let default_node = super::BvhNode::default();
        for i in 0..scene.bvh.nodes_used() {
            assert_ne!(scene.bvh.nodes[i as u32], default_node);
        }
    }

    #[test]
    fn test_bvh_nodes_unused() {
        let scene = scene();
        let default_node = super::BvhNode::default();
        for i in scene.bvh.nodes_used()..scene.bvh.nodes.len() {
            assert_eq!(scene.bvh.nodes[i as u32], default_node);
        }
    }

    /// For maximum cache coherence, a BVH node should fill one or two cache lines on a 
    /// modern microprocessor.
    #[test]
    fn test_bvh_node_fits_inside_a_cache_line() {
        assert_eq!(std::mem::size_of::<super::BvhNode>(), 32);
    }

    #[test]
    fn test_bvh_branch_node_children_always_have_larger_indices_than_parents() {
        let scene = scene();
        for node_index in 0..scene.bvh.nodes_used {
            let node = &scene.bvh.nodes[node_index];
            if node.is_branch() {
                assert!(node.as_branch().left_node() > node_index);
                assert!(node.as_branch().right_node() > node_index);
                assert!(node.as_branch().left_node() < node.as_branch().right_node());
            }
        }
    }

    #[test]
    fn test_branch_nodes_have_no_primitives() {
        let scene = scene();
        for i in 0..scene.bvh.nodes_used {
            if scene.bvh.nodes[i].is_branch() {
                assert_eq!(scene.bvh.nodes[i].primitive_count, 0);
            }
        }
    }
}

