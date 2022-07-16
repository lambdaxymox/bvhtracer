use crate::aabb::*;
use crate::triangle::*;
use crate::ray::*;
use cglinalg::{
    Vector3,
};

use std::fmt;
use std::ops;
use std::mem;


// NOTE: We use local implementations of min and max of vector components here because the
// compiler does not seem to want to inline it here.
#[inline] 
fn __min(this: &Vector3<f32>, that: &Vector3<f32>) -> Vector3<f32> { 
    // this.component_min(that)
    Vector3::new(
        f32::min(this.x, that.x),
        f32::min(this.y, that.y),
        f32::min(this.z, that.z),
    )
}

#[inline] 
fn __max(this: &Vector3<f32>, that: &Vector3<f32>) -> Vector3<f32> {
    // this.component_max(that)
    Vector3::new(
        f32::max(this.x, that.x),
        f32::max(this.y, that.y),
        f32::max(this.z, that.z),
    )
}


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
    primitives: &'a [Triangle<f32>],
    primitive_count: u32,
    base_primitive_index: u32,
    current_offset: u32,
}

impl<'a> PrimitiveIter<'a> {
    fn new(primitives: &'a [Triangle<f32>], primitive_count: u32, base_primitive_index: u32) -> Self {
        Self {
            primitives,
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
            let current_object = &self.primitives[current_primitive_index as usize];
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
struct BvhNode {
    aabb: Aabb<f32>,
    primitive_count: u32,
    _left_first: BranchOrLeafData,
}

impl BvhNode {
    #[inline]
    fn is_leaf(&self) -> bool {
        self.primitive_count > 0
    }

    #[inline]
    fn is_branch(&self) -> bool {
        self.primitive_count == 0
    }

    #[inline]
    const fn as_leaf(&self) -> &BvhLeafNode {
        unsafe { 
            mem::transmute::<&BvhNode, &BvhLeafNode>(self)
        }
    }

    #[inline]
    const fn as_branch(&self) -> &BvhBranchNode {
        unsafe { 
            mem::transmute::<&BvhNode, &BvhBranchNode>(self)
        }
    }

    #[inline]
    fn as_leaf_mut(&mut self) -> &mut BvhLeafNode {
        unsafe { 
            mem::transmute::<&mut BvhNode, &mut BvhLeafNode>(self)
        }
    }

    #[inline]
    fn as_branch_mut(&mut self) -> &mut BvhBranchNode {
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

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct Bvh {
    nodes: BvhNodeArray,
    node_indices: Vec<u32>,
    root_node_index: u32,
    nodes_used: u32,
}

impl Bvh {
    fn primitive_iter<'a>(&self, mesh: &'a [Triangle<f32>], node: &BvhNode) -> PrimitiveIter<'a> {
        let base_primitive_index = self.node_indices[node.as_leaf().first_primitive_index as usize];
        
        PrimitiveIter::new(mesh, node.primitive_count, base_primitive_index)
    }

    fn intersect_subtree(&self, mesh: &[Triangle<f32>], ray: &Ray<f32>, node_index: u32) -> Option<f32> {    
        let mut current_node = &self.nodes[node_index];
        let mut stack = vec![];
        let mut closest_ray = *ray;
        loop {
            if current_node.is_leaf() {
                for primitive in self.primitive_iter(mesh, current_node) {
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


    pub fn intersect(&self, mesh: &[Triangle<f32>], ray: &Ray<f32>) -> Option<f32> {
        self.intersect_subtree(mesh, ray, self.root_node_index)
    }

    /// Returns the number of nodes in the boundary volume hierarchy.
    #[inline]
    pub const fn nodes_used(&self) -> usize {
        self.nodes_used as usize
    }

    fn update_node_bounds(&mut self, mesh: &[Triangle<f32>], node_index: u32) {
        let mut new_aabb = Aabb::new(Vector3::from_fill(f32::MAX), Vector3::from_fill(-f32::MAX));
        for primitive in self.primitive_iter(mesh, &self.nodes[node_index]) {
            new_aabb.bounds_min = __min(&new_aabb.bounds_min, &primitive.vertex0);
            new_aabb.bounds_min = __min(&new_aabb.bounds_min, &primitive.vertex1);
            new_aabb.bounds_min = __min(&new_aabb.bounds_min, &primitive.vertex2);
            new_aabb.bounds_max = __max(&new_aabb.bounds_max, &primitive.vertex0);
            new_aabb.bounds_max = __max(&new_aabb.bounds_max, &primitive.vertex1);
            new_aabb.bounds_max = __max(&new_aabb.bounds_max, &primitive.vertex2);
        }

        let node = &mut self.nodes[node_index];
        node.aabb = new_aabb;
    }

    // TODO: Optimize by finding the longest axis first?
    fn find_best_split_plane(&self, mesh: &[Triangle<f32>], node: &BvhNode) -> (isize, f32, f32) {
        const BIN_COUNT: usize = 8;
        let mut best_axis = -1;
        let mut best_position = 0_f32;
        let mut best_cost = f32::MAX;
        for axis in 0..3 {
            let mut bounds_min = 1e30;
            let mut bounds_max = 1e-30;
            for primitive in self.primitive_iter(mesh, node) {
                bounds_min = f32::min(bounds_min, primitive.centroid[axis]);
                bounds_max = f32::max(bounds_max, primitive.centroid[axis]);
            }
            if bounds_min == bounds_max {
                continue;
            }

            let mut bins = [Bin::default(); BIN_COUNT];
            let bin_scale = (BIN_COUNT as f32) / (bounds_max - bounds_min);
            for primitive in self.primitive_iter(mesh, node) {
                let possible_bin_index = ((primitive.centroid[axis] - bounds_min) * bin_scale) as usize;
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

    fn subdivide(&mut self, mesh: &mut [Triangle<f32>], node_index: u32) {
        // Terminate recursion.
        let (best_axis, best_position, best_cost) = {
            let node = &self.nodes[node_index];
            self.find_best_split_plane(mesh, node)
        };
        let (left_count, i) = {
            let node = &self.nodes[node_index];
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
                if mesh[i as usize].centroid[axis] < split_position {
                    i += 1;
                } else {
                    mesh.swap(i as usize, j as usize);
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
            let nodes_used = self.nodes_used;
            self.nodes_used += 1;
            nodes_used
        };
        let right_child_index = {
            let nodes_used = self.nodes_used;
            self.nodes_used += 1;
            nodes_used
        };
        {
            self.nodes[left_child_index].as_leaf_mut().first_primitive_index = self.nodes[node_index].as_leaf().first_primitive_index;
            self.nodes[left_child_index].primitive_count = left_count;
            self.nodes[right_child_index].as_leaf_mut().first_primitive_index = i;
            self.nodes[right_child_index].primitive_count = self.nodes[node_index].primitive_count - left_count;
        }
        {
            let node = &mut self.nodes[node_index];
            node.as_branch_mut().left_node = left_child_index;
            node.primitive_count = 0;
        }

        self.update_node_bounds(mesh, left_child_index);
        self.update_node_bounds(mesh, right_child_index);
        // Recurse
        self.subdivide(mesh, left_child_index);
        self.subdivide(mesh, right_child_index);
    }

    pub fn refit(&mut self, mesh: &[Triangle<f32>]) {
        for node_index in (0..self.nodes_used).rev() {
            if node_index != 1 {
                {
                    let node = &self.nodes[node_index];
                    if node.is_leaf() {
                        // Leaf node: adjust bounds to contained primitives.
                        self.update_node_bounds(mesh, node_index as u32);
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

    /// Returns the bounding box for the boundary volume hierarchy. 
    /// 
    /// This bounding box should enclose the entire model that the BVH is 
    /// associated with.
    pub fn bounds(&self) -> Aabb<f32> {
        self.nodes[0].aabb
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
        // We set the nodes_used count to 2 to skip node index 1. Every branch node
        // has two child nodes, and we want them to align nicely in the cache. In order
        // to do that, we insert a dummy node at index 1, using index 0 for the root.
        let nodes_used = 2;

        let partial_bvh = Bvh { nodes, node_indices, root_node_index, nodes_used, };

        Self { partial_bvh, }
    }

    pub fn build_for(mut self, mesh: &mut [Triangle<f32>]) -> Bvh {
        // Populate the triangle index array.
        for i in 0..mesh.len() {
            self.partial_bvh.node_indices.push(i as u32);
        }

        self.partial_bvh.nodes = BvhNodeArray(vec![BvhNode::default(); 2 * mesh.len()]);
        
        let root_node_index = self.partial_bvh.root_node_index;
        let mut root_node: &mut BvhNode = &mut self.partial_bvh.nodes[root_node_index];
        root_node.as_branch_mut().left_node = 0;
        root_node.primitive_count = mesh.len() as u32;

        self.partial_bvh.update_node_bounds(mesh, self.partial_bvh.root_node_index);
        self.partial_bvh.subdivide(mesh, self.partial_bvh.root_node_index);

        self.partial_bvh
    }
}


#[cfg(test)]
mod bvh_tests {
    use super::*;
    use cglinalg::{
        Vector3,
    };
    

    fn bvh() -> (Bvh, Vec<Triangle<f32>>) {
        let displacement_x = Vector3::new(5_f32, 0_f32, 0_f32);
        let displacement_y = Vector3::new(0_f32, 5_f32, 0_f32);
        let triangle0 = Triangle::new(
            Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32),
            Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
            Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
        );
        let mut mesh = (-100..100).zip(-100..100).map(|(i, j)| {
            Triangle::new(
                triangle0.vertex0 + (i as f32) * displacement_x + (j as f32) * displacement_y,
                triangle0.vertex1 + (i as f32) * displacement_x + (j as f32) * displacement_y,
                triangle0.vertex1 + (i as f32) * displacement_x + (j as f32) * displacement_y,
            )
        })
        .collect::<Vec<Triangle<_>>>();
        
        let builder = BvhBuilder::new();
        let bvh = builder.build_for(&mut mesh);

        (bvh, mesh)
    }

    #[test]
    fn test_bvh_nodes_used_smaller_than_node_array_length() {
        let bvh = bvh().0;
        
        assert!(bvh.nodes.len() > bvh.nodes_used());
    }

    #[test]
    fn test_bvh_nodes_used() {
        let bvh = bvh().0;
        let default_node = BvhNode::default();
        for i in (0..bvh.nodes_used()).filter(|i| *i != 1) {
            assert_ne!(bvh.nodes[i as u32], default_node);
        }
    }

    #[test]
    fn test_bvh_nodes_unused() {
        let bvh = bvh().0;
        let default_node = BvhNode::default();
        for i in bvh.nodes_used()..bvh.nodes.len() {
            assert_eq!(bvh.nodes[i as u32], default_node);
        }
    }

    #[test]
    fn test_bvh_branch_node_children_always_have_larger_indices_than_parents() {
        let bvh = bvh().0;
        for node_index in (0..bvh.nodes_used).filter(|i| *i != 1) {
            let node = &bvh.nodes[node_index];
            if node.is_branch() {
                assert!(node.as_branch().left_node() > node_index);
                assert!(node.as_branch().right_node() > node_index);
                assert!(node.as_branch().left_node() < node.as_branch().right_node());
            }
        }
    }

    #[test]
    fn test_branch_nodes_have_no_primitives() {
        let bvh = bvh().0;
        for i in 0..bvh.nodes_used {
            if bvh.nodes[i].is_branch() {
                assert_eq!(bvh.nodes[i].primitive_count, 0);
            }
        }
    }

    /// The second entry in the BVH hierarchy's table exists to make the left and right nodes
    /// align in the cache, and it never used during the lifetime of the BVH.
    #[test]
    fn test_second_bvh_entry_should_be_default_node() {
        let bvh = bvh().0;
        let expected = BvhNode::default();
        let result = bvh.nodes[1];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bvh_refit_should_not_affect_node_position_1() {
        let (mut bvh, mesh) = bvh();
        let expected = bvh.nodes[1];
        bvh.refit(&mesh);
        let result = bvh.nodes[1];

        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod bvh_node_tests {
    use super::*;
    use cglinalg::{
        Vector3,
    };


    fn leaf_node1() -> BvhNode {
        let aabb = Aabb::new(
            Vector3::new(-1_f32, -1_f32, -1_f32),
            Vector3::new(1_f32, 1_f32, 1_f32),
        );
        let node = BvhNode {
            aabb,
            primitive_count: 1200,
            _left_first: BranchOrLeafData {
                first_primitive_index: 0,
            },
        };

        node
    }

    fn leaf_node2() -> BvhNode {
        let aabb = Aabb::new(
            Vector3::new(1_f32, -1_f32, -1_f32),
            Vector3::new(2_f32, 2_f32, 1_f32),
        );
        let node = BvhNode {
            aabb,
            primitive_count: 200,
            _left_first: BranchOrLeafData {
                first_primitive_index: 0,
            },
        };

        node
    }

    fn branch_node1() -> BvhNode {
        let aabb = Aabb::new(
            Vector3::new(-2_f32, -3_f32, -5_f32),
            Vector3::new(2_f32, 3_f32, 5_f32),
        );
        let node = BvhNode {
            aabb,
            primitive_count: 0,
            _left_first: BranchOrLeafData {
                left_node: 2,
            }
        };

        node
    }

    fn branch_node2() -> BvhNode {
        let aabb = Aabb::new(
            Vector3::new(-2_f32, -3_f32, -5_f32),
            Vector3::new(2_f32, 3_f32, 5_f32),
        );
        let node = BvhNode {
            aabb,
            primitive_count: 0,
            _left_first: BranchOrLeafData {
                left_node: 3,
            }
        };

        node
    }

    /// For maximum cache coherence, a BVH node should fill one or two cache lines on a 
    /// modern microprocessor.
    #[test]
    fn test_bvh_node_fits_inside_a_cache_line() {
        assert_eq!(std::mem::size_of::<super::BvhNode>(), 32);
    } 

    #[test]
    fn test_leaf_node() {
        let node = leaf_node1();

        assert!(node.is_leaf());
    }

    #[test]
    fn test_branch_node() {
        let node = branch_node1();

        assert!(node.is_branch());
    }

    #[test]
    fn test_branch_node_compare_leaf_branch() {
        let leaf_node = leaf_node1();
        let branch_node = branch_node1();

        assert_ne!(leaf_node, branch_node);
    }
    
    #[test]
    fn test_compare_leaf_node_to_self() {
        let leaf_node = leaf_node1();

        assert_eq!(leaf_node, leaf_node);
    }
    
    #[test]
    fn test_compare_branch_node_to_self() {
        let branch_node = branch_node1();

        assert_eq!(branch_node, branch_node);
    }

    #[test]
    fn test_compare_branch_node_branch_node() {
        let branch_node1 = branch_node1();
        let branch_node2 = branch_node2();

        assert_ne!(branch_node1, branch_node2);
    }

    #[test]
    fn test_compare_leaf_node_leaf_node() {
        let leaf_node1 = leaf_node1();
        let leaf_node2 = leaf_node2();

        assert_ne!(leaf_node1, leaf_node2);
    }
}

#[cfg(test)]
mod bvh_tree_one_triangle_tests {
    use super::*;
    use cglinalg::{
        Vector3,
    };

    fn triangle() -> Triangle<f32> {
        Triangle::new(
            Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32),
            Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
            Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
        )
    }

    fn result_bvh() -> Bvh {
        let triangle = triangle();
        let mut mesh = vec![triangle];
        let builder = BvhBuilder::new();
        
        builder.build_for(&mut mesh)
    }

    fn expected_bvh() -> Bvh {
        let triangle = triangle();
        let mut aabb = Aabb::new_empty();
        aabb.grow(&triangle.vertex0);
        aabb.grow(&triangle.vertex1);
        aabb.grow(&triangle.vertex2);

        let root_node = BvhNode {
            aabb: aabb,
            primitive_count: 1,
            _left_first: BranchOrLeafData {
                first_primitive_index: 0,
            },
        };
        let dummy_node = BvhNode::default();
        let nodes = BvhNodeArray(vec![root_node, dummy_node]);
        let node_indices = vec![0];
        let root_node_index = 0;
        let nodes_used = 2;

        Bvh { nodes, node_indices, root_node_index, nodes_used, }
    }


    #[test]
    fn test_bvh_one_triangle() {
        let result = result_bvh();
        let expected = expected_bvh();

        assert_eq!(result, expected);
    }
}

