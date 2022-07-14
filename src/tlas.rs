use crate::aabb::*;
use crate::ray::*;
use crate::bvh::*;
use cglinalg::{
    Vector3,
};
use std::rc::{
    Rc,
};


#[derive(Copy, Clone)]
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

pub struct Tlas {
    blas: Vec<Rc<Bvh>>,
    nodes: Vec<TlasNode>,
    nodes_used: u32,
}

impl Tlas {
    pub fn intersect(&self, ray: &Ray<f32>) -> Option<f32> {
        unimplemented!()
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
            nodes: vec![TlasNode::default(); nodes_used as usize],
            nodes_used: nodes_used,
        };
        
        Self { partial_tlas, }
    }

    pub fn with_bvh_list(mut self, bvh_list: Vec<Rc<Bvh>>) -> Self {
        let len_nodes = 2 * bvh_list.len();
        self.partial_tlas.blas = bvh_list;
        self.partial_tlas.nodes = vec![TlasNode::default(); len_nodes];

        self
    }

    pub fn with_bvh(mut self, bvh: Rc<Bvh>) -> Self {
        self.partial_tlas.blas.push(bvh);
        self.partial_tlas.nodes.push(TlasNode::default());
        self.partial_tlas.nodes.push(TlasNode::default());

        self
    }

    // TODO: Make this more general.
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

        self.partial_tlas
    }
}

