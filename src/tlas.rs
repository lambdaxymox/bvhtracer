use crate::aabb::*;
use crate::ray::*;
use crate::scene::{
    SceneObject,
};
use cglinalg::{
    Vector3,
};
use std::ops;
use std::rc::{
    Rc,
};


#[derive(Copy, Clone, Debug)]
struct TlasNode {
    aabb: Aabb<f32>,
    left_blas: u32,
    is_leaf: u32,
}

impl Default for TlasNode {
    fn default() -> Self {
        Self {
            aabb: Aabb::new_empty(),
            left_blas: 0,
            is_leaf: true as u32,
        }
    }
}

impl TlasNode {
    #[inline]
    fn is_leaf(&self) -> bool {
        self.is_leaf == true as u32
    }

    #[inline]
    fn left_blas(&self) -> u32 {
        self.left_blas
    }

    #[inline]
    fn right_blas(&self) -> u32 {
        self.left_blas + 1
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

    pub fn intersect(&self, ray: &Ray<f32>) -> Option<f32> {
        let mut current_node = &self.nodes[0];
        let mut stack = vec![];
        let mut closest_ray = *ray;
        loop {
            if current_node.is_leaf() {
                if let Some(t_intersect) = self.blas[current_node.left_blas() as usize].intersect(ray) {
                    closest_ray.t = t_intersect;
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

        if closest_ray.t < f32::MAX { Some(closest_ray.t) } else { None }
    }

    #[inline]
    pub fn nodes_used(&self) -> usize {
        self.nodes_used as usize
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

    // TODO: Make this more general. We are currently only constructing it for a scene with
    // two armadillos.
    pub fn build(mut self) -> Tlas {
        // Assign a TLASLeaf node to each BLAS.
        self.partial_tlas.nodes[2].left_blas = 0;
        self.partial_tlas.nodes[2].aabb = Aabb::new(
            Vector3::from_fill(-100_f32), 
            Vector3::from_fill(100_f32),
        );
        self.partial_tlas.nodes[2].is_leaf = true as u32;
        self.partial_tlas.nodes[3].left_blas = 1;
        self.partial_tlas.nodes[3].aabb = Aabb::new(
            Vector3::from_fill(-100_f32), 
            Vector3::from_fill(100_f32),
        );
        self.partial_tlas.nodes[3].is_leaf = true as u32;
        // Create a root node over the two leaf nodes.
        // NOTE: Node 1 is a filler to get two tlas child nodes to fit into the 
        // same cache line.
        self.partial_tlas.nodes[0].left_blas = 2;
        self.partial_tlas.nodes[0].aabb = Aabb::new(
            Vector3::from_fill(-100_f32), 
            Vector3::from_fill(100_f32)
        );
        self.partial_tlas.nodes[0].is_leaf = false as u32;

        self.partial_tlas.nodes_used = 4;

        self.partial_tlas
    }
}

