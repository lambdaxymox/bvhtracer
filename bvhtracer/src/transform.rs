use cglinalg::{
    Rotation3,
    Scale3,
    SimdScalarFloat,
    Vector3,
    Point3,
    Matrix4x4,
    Unit,
    Radians,
};
use std::ops;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform3<S> {
    pub scale: Vector3<S>,
    pub translation: Vector3<S>,
    pub rotation: Rotation3<S>,
}

impl<S> Transform3<S>
where
    S: SimdScalarFloat,
{
    #[inline]
    pub const fn new(scale: &Vector3<S>, translation: &Vector3<S>, rotation: Rotation3<S>) -> Self {
        Self { 
            scale: *scale, 
            translation: *translation, 
            rotation, 
        }
    }

    #[inline]
    pub fn from_scale(scale: Vector3<S>) -> Self {
        Self {
            scale,
            translation: Vector3::zero(),
            rotation: Rotation3::identity(),
        }
    }

    #[inline]
    pub fn from_translation(translation: &Vector3<S>) -> Self {
        Self {
            scale: Vector3::zero(),
            translation: *translation,
            rotation: Rotation3::identity(),
        }
    }

    #[inline]
    pub fn from_axis_angle<A>(axis: &Unit<Vector3<S>>, angle: A) -> Self 
    where
        A: Into<Radians<S>>,
    {
        Self {
            scale: Vector3::zero(),
            translation: Vector3::zero(),
            rotation: Rotation3::from_axis_angle(axis, angle),
        }
    }

    #[inline]
    pub fn from_scale_translation(scale: &Vector3<S>, translation: &Vector3<S>) -> Self {
        Self {
            scale: *scale,
            translation: *translation,
            rotation: Rotation3::identity(),
        }
    }

    #[inline]
    pub fn from_scale_axis_angle<A: Into<Radians<S>>>(scale: &Vector3<S>, axis: &Unit<Vector3<S>>, angle: A) -> Self 
    where
        A: Into<Radians<S>>,
    {
        Self {
            scale: *scale,
            translation: Vector3::zero(),
            rotation: Rotation3::from_axis_angle(axis, angle),
        }
    }

    #[inline]
    pub fn from_translation_axis_angle<A>(translation: &Vector3<S>, axis: &Unit<Vector3<S>>, angle: A) -> Self 
    where
        A: Into<Radians<S>>,
    {
        Self {
            scale: Vector3::zero(),
            translation: *translation,
            rotation: Rotation3::from_axis_angle(axis, angle),
        }
    }

    #[inline]
    pub fn from_scale_translation_axis_angle<A>(scale: &Vector3<S>, translation: &Vector3<S>, axis: &Unit<Vector3<S>>, angle: A) -> Self 
    where
        A: Into<Radians<S>>,
    {
        Self {
            scale: *scale,
            translation: *translation,
            rotation: Rotation3::from_axis_angle(axis, angle),
        }
    }

    pub fn identity() -> Self {
        Self {
            scale: Vector3::from_fill(S::one()),
            translation: Vector3::zero(),
            rotation: Rotation3::identity(),
        }
    }

    pub fn transform_point(&self, point: &Vector3<S>) -> Vector3<S> {
        let scaled = self.scale.component_mul(point);
        let rotated = self.rotation.rotate_vector(&scaled);
        let translated = rotated + self.translation;
        translated
    }

    pub fn transform_vector(&self, vector: &Vector3<S>) -> Vector3<S> {
        let scaled = self.scale.component_mul(vector);
        let rotated = self.rotation.rotate_vector(&scaled);
        rotated
    }

    pub fn to_matrix4x4_mut(&self, out: &mut Matrix4x4<S>) {
        let mut new_matrix = self.rotation.to_affine_matrix();
        new_matrix[3][0] = self.translation[0];
        new_matrix[3][1] = self.translation[1];
        new_matrix[3][2] = self.translation[2];
        new_matrix[0][0] *= self.scale[0];
        new_matrix[1][1] *= self.scale[1];
        new_matrix[2][2] *= self.scale[2];
        *out = new_matrix;
    }

    pub fn to_matrix4x4(&self) -> Matrix4x4<S> {
        let mut out = Matrix4x4::zero();
        self.to_matrix4x4_mut(&mut out);

        out
    }

    pub fn inverse(&self) -> Option<Self> {
        let scale_inv = Vector3::new(
            S::one() / self.scale.x,
            S::one() / self.scale.y,
            S::one() / self.scale.z,
        );
        let transform_inv = Self::new(
            &scale_inv,
            &(-self.translation),
            self.rotation.inverse(),
        );
        Some(transform_inv)
    }
}

