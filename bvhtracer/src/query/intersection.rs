use super::ray::*;
use cglinalg::{
    SimdScalarFloat,
    SimdScalar,
};


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct SurfaceInteraction<S> 
where
    S: SimdScalar
{
    pub t: S,
    pub u: S,
    pub v: S,
}

impl<S> SurfaceInteraction<S>
where
    S: SimdScalar
{
    pub fn new(t: S, u: S, v: S) -> Self {
        Self { t, u, v, }
    }
}

/// An instance primitive index that marks out which instance of a mesh in a 
/// scene we are intersection querying, as well as which primitive in that mesh 
/// we are making a query about.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InstancePrimitiveIndex {
    /// The underlying storage for the instance and primitive indices. Instance and 
    /// primitive indices are packed into a single `u32`. This enables a `f32`
    /// intersection result to fit into one 16 byte cache line. This is optimizing 
    /// memory footprint for the GPU as well as the CPU.
    data: u32,
}

impl InstancePrimitiveIndex {
    /// Construct a new instance primitive index.
    pub const fn new(instance_index: u32, primitive_index: u32) -> Self {        
        Self { 
            data: ((instance_index & 0x00000FFF) << 20) | (primitive_index & 0x000FFFFF),
        }
    }

    /// Get the index of the model instance that we are querying in a scene 
    /// for an intersection test.
    #[inline]
    pub const fn instance_index(self) -> u32 {
        (self.data & 0xFFF00000) >> 20
    }

    /// Get the indes of the primitive from the mesh that we are querying in 
    /// a scene for an intersection test.
    #[inline]
    pub const fn primitive_index(self) -> u32 {
        self.data & 0x000FFFFF
    }

    #[inline]
    pub const fn from_primitive(primitive_index: u32) -> Self {
        Self::new(0, primitive_index)
    }
}

impl Default for InstancePrimitiveIndex {
    fn default() -> Self {
        Self { 
            data: 0, 
        }
    }
}

/// An intersection record that is tuned to be exactly sixteen bytes in size when 
/// each field in the intersection result is a 32-bit floating point number.
/// 
/// In particular, given three 32 bit floating point numbers and one 32 bit 
/// instance primitive index, the intersection record has a size of 128 bits, 
/// or 16 bytes (3 * 4 bytes + 4 bytes == 16 bytes).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<S> 
where
    S: SimdScalar
{
    pub ray: Ray<S>,
    pub interaction: SurfaceInteraction<S>,
    pub instance_primitive: InstancePrimitiveIndex,
}

impl<S> Intersection<S>
where
    S: SimdScalarFloat
{
    pub fn new(ray: Ray<S>, interaction: SurfaceInteraction<S>, instance_primitive: InstancePrimitiveIndex) -> Self {
        Self { ray, interaction, instance_primitive, }
    }

    pub fn from_ray_interaction(ray: Ray<S>, interaction: SurfaceInteraction<S>) -> Self {
        Self {
            ray: ray,
            interaction: interaction,
            instance_primitive: InstancePrimitiveIndex::default(),
        }
    }
}

