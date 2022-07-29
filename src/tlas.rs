use crate::intersection::*;
use crate::geometry::*;
use crate::ray::*;
use crate::scene_object::*;
use cglinalg::{
    Vector3,
};
use std::ops;


// TODO: Rethink some naming and understandability.
#[derive(Copy, Clone, Debug)]
struct TlasNode {
    aabb: Aabb<f32>,
    // Upper u16 is the left index, lower u16 is the right index.
    left_right: u32,
    blas: u32,
}

impl Default for TlasNode {
    fn default() -> Self {
        Self {
            aabb: Aabb::new_empty(),
            left_right: 0,
            blas: 0,
        }
    }
}

impl TlasNode {
    #[inline]
    fn is_leaf(&self) -> bool {
        self.left_right == 0
    }

    #[inline]
    fn is_branch(&self) -> bool {
        self.left_right != 0
    }

    #[inline]
    fn left_blas(&self) -> u32 {
        (self.left_right & 0xFFFF0000) >> 16
    }

    #[inline]
    fn right_blas(&self) -> u32 {
        self.left_right & 0x0000FFFF
    }

    #[inline]
    fn blas(&self) -> u32 {
        self.blas
    }
}

#[derive(Clone, Debug)]
struct TlasNodeArray(Vec<TlasNode>);

impl TlasNodeArray {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl ops::Index<u32> for TlasNodeArray {
    type Output = TlasNode;

    #[inline]
    fn index(&self, _index: u32) -> &Self::Output {
        self.0.index(_index as usize)
    }
    
}

impl ops::IndexMut<u32> for TlasNodeArray {
    #[inline]
    fn index_mut(&mut self, _index: u32) -> &mut Self::Output {
        self.0.index_mut(_index as usize)
    }
}

pub struct Tlas {
    blas: Vec<SceneObject>,
    nodes: TlasNodeArray,
    nodes_used: u32,
}

impl Tlas {
    #[inline]
    pub fn get_unchecked(&self, index: usize) -> &SceneObject {
        &self.blas[index]
    }

    #[inline]
    pub fn get_mut_unchecked(&mut self, index: usize) -> &mut SceneObject {
        &mut self.blas[index]
    }

    #[inline]
    pub fn nodes_used(&self) -> usize {
        self.nodes_used as usize
    }

    pub fn intersect(&self, ray: &Ray<f32>) -> Option<Intersection<f32>> {
        let mut current_node = &self.nodes[0];
        let mut stack = vec![];
        let mut closest_ray = *ray;
        let mut closest_intersection = None;
        loop {
            if current_node.is_leaf() {
                if let Some(intersection) = self.blas[current_node.blas() as usize].intersect(ray) {
                    closest_ray.t = intersection.ray.t;
                    closest_intersection = Some(intersection);
                }

                if !stack.is_empty() {
                    current_node = stack.pop().unwrap();
                } else {
                    break;
                }
            } else {
                let (closest_child, closest_dist, farthest_child, farthest_dist) = {
                    let left_child = &self.nodes[current_node.left_blas()];
                    let right_child = &self.nodes[current_node.right_blas()];
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

        if (closest_ray.t < f32::MAX) && closest_intersection.is_some() { 
            closest_intersection
        } else { 
            None 
        }
    }

    fn find_best_match(&self, list: &[i32], n: i32, a: i32) -> i32 {
        let mut smallest = f32::MAX;
        let mut best_b: i32 = -1;
        for b in 0..n { 
            if b != a {
                let bounds_max = Vector3::component_max(
                    &self.nodes[list[a as usize] as u32].aabb.bounds_max,
                    &self.nodes[list[b as usize] as u32].aabb.bounds_max,
                );
                let bounds_min = Vector3::component_min(
                    &self.nodes[list[a as usize] as u32].aabb.bounds_min,
                    &self.nodes[list[b as usize] as u32].aabb.bounds_min,
                );
                let extent = bounds_max - bounds_min;
                let surface_area = extent.x * extent.y + extent.y * extent.z + extent.z * extent.x;
                if surface_area < smallest {
                    smallest = surface_area;
                    best_b = b;
                }
            }
        }

        best_b
    }

    // TODO: Make this more general. We are currently building TLASes for at most 256 objects.
    pub fn rebuild(&mut self) {
        // Assign a Tlasleaf node to each BLAS.
        let blas_count = self.blas.len();
        let mut node_index_count = blas_count;
        let mut node_indices = vec![0_i32; 256];
        let mut nodes_used = 1;
        for i in 0..blas_count {
            node_indices[i] = nodes_used;
            let bounds_i = self.blas[i].bounds();
            self.nodes[nodes_used as u32].aabb = bounds_i;
            self.nodes[nodes_used as u32].blas = i as u32;
            // Make it a leaf.
            self.nodes[nodes_used as u32].left_right = 0;
            nodes_used += 1;
        }

        // Use agglomerative clustering to build the TLAS.
        let mut a = 0;
        let mut b = self.find_best_match(node_indices.as_slice(), node_index_count as i32, a);
        while node_index_count > 1 {
            let c = self.find_best_match(node_indices.as_slice(), node_index_count as i32, b);
            if a == c {
                let node_index_a = node_indices[a as usize];
                let node_index_b = node_indices[b as usize];
                let node_a = self.nodes[node_index_a as u32];
                let node_b = self.nodes[node_index_b as u32];
                let new_node = &mut self.nodes[nodes_used as u32];
                new_node.left_right = (node_index_a + (node_index_b << 16)) as u32;
                new_node.aabb = Aabb::new(
                    Vector3::component_min(&node_a.aabb.bounds_min, &node_b.aabb.bounds_min),
                    Vector3::component_max(&node_a.aabb.bounds_max, &node_b.aabb.bounds_max),
                );

                node_indices[a as usize] = nodes_used;
                nodes_used += 1;
                node_indices[b as usize] = node_indices[node_index_count - 1];
                
                node_index_count -= 1;
                b = self.find_best_match(&node_indices, node_index_count as i32, a);
            } else {
                a = b;
                b = c;
            }
        }
        self.nodes[0] = self.nodes[node_indices[a as usize] as u32];
        self.nodes_used = nodes_used as u32;
    }
}

pub struct TlasBuilder {
    partial_tlas: Tlas,
}

impl TlasBuilder {
    pub fn new() -> Self {
        // For an initial top level acceleration structure that contains no bottom level
        // acceleration structures (i.e. BVHs), the initial number of nodes used is two, 
        // one for the root node, and one for an unused filler node in the array that aligns
        // the buffer in memory nicely (every node's left and right children sit next to each 
        // other in memory).
        let nodes_used = 2;
        let partial_tlas = Tlas {
            blas: vec![],
            nodes: TlasNodeArray(vec![TlasNode::default(); nodes_used as usize]),
            nodes_used: nodes_used,
        };
        
        Self { partial_tlas, }
    }

    pub fn with_objects(mut self, objects: Vec<SceneObject>) -> Self {
        let len_nodes = 2 * objects.len();
        self.partial_tlas.blas = objects;
        self.partial_tlas.nodes = TlasNodeArray(vec![TlasNode::default(); len_nodes]);

        self
    }

    pub fn with_object(mut self, object: SceneObject) -> Self {
        self.partial_tlas.blas.push(object);
        self.partial_tlas.nodes.0.push(TlasNode::default());
        self.partial_tlas.nodes.0.push(TlasNode::default());

        self
    }

    // TODO: Make this more general. We are currently building TLASes for at most 256 objects.
    pub fn build(mut self) -> Tlas { 
        self.partial_tlas.rebuild();

        self.partial_tlas
    }
}

