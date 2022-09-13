use cglinalg::{
    Rotation3,
    SimdScalarFloat,
    Vector3,
    Matrix4x4,
    Unit,
    Radians,
};
use std::ops;


#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform3<S> {
    matrix: Matrix4x4<S>,
}

impl<S> Transform3<S>
where
    S: SimdScalarFloat,
{
    #[inline]
    pub fn new(scale: &Vector3<S>, translation: &Vector3<S>, rotation: Rotation3<S>) -> Self {
        let mut matrix = rotation.to_affine_matrix();
        matrix[3][0] = translation[0];
        matrix[3][1] = translation[1];
        matrix[3][2] = translation[2];
        matrix[0][0] *= scale[0];
        matrix[1][1] *= scale[1];
        matrix[2][2] *= scale[2];
        
        Self { matrix, }
    }

    #[inline]
    pub fn from_scale(scale: &Vector3<S>) -> Self {
        let matrix = Matrix4x4::from_affine_nonuniform_scale(scale.x, scale.y, scale.z);

        Self { matrix, }
    }

    #[inline]
    pub fn from_translation(translation: &Vector3<S>) -> Self {
        let matrix = Matrix4x4::from_affine_translation(translation);

        Self { matrix, }
    }

    #[inline]
    pub fn from_axis_angle<A>(axis: &Unit<Vector3<S>>, angle: A) -> Self 
    where
        A: Into<Radians<S>>,
    {
        let matrix = Matrix4x4::from_affine_axis_angle(axis, angle);

        Self { matrix, }
    }

    #[inline]
    pub fn from_scale_translation(scale: &Vector3<S>, translation: &Vector3<S>) -> Self {
        let mut matrix = Matrix4x4::identity();
        matrix[0][0] = scale[0];
        matrix[1][1] = scale[1];
        matrix[2][2] = scale[2];
        matrix[3][0] = translation[0];
        matrix[3][1] = translation[1];
        matrix[3][2] = translation[2];

        Self { matrix, }
    }

    #[inline]
    pub fn from_scale_axis_angle<A: Into<Radians<S>>>(scale: &Vector3<S>, axis: &Unit<Vector3<S>>, angle: A) -> Self 
    where
        A: Into<Radians<S>>,
    {
        let mut matrix = Matrix4x4::from_affine_axis_angle(axis, angle);
        matrix[0][0] *= scale[0];
        matrix[1][1] *= scale[1];
        matrix[2][2] *= scale[2];

        Self { matrix, }
    }

    #[inline]
    pub fn from_translation_axis_angle<A>(translation: &Vector3<S>, axis: &Unit<Vector3<S>>, angle: A) -> Self 
    where
        A: Into<Radians<S>>,
    {
        let mut matrix = Matrix4x4::from_affine_axis_angle(axis, angle);
        matrix[3][0] = translation[0];
        matrix[3][1] = translation[1];
        matrix[3][2] = translation[2];

        Self { matrix, }
    }

    #[inline]
    pub fn from_scale_translation_axis_angle<A>(scale: &Vector3<S>, translation: &Vector3<S>, axis: &Unit<Vector3<S>>, angle: A) -> Self 
    where
        A: Into<Radians<S>>,
    {
        let mut matrix = Matrix4x4::from_affine_axis_angle(axis, angle);
        matrix[0][0] *= scale[0];
        matrix[1][1] *= scale[1];
        matrix[2][2] *= scale[2];
        matrix[3][0] = translation[0];
        matrix[3][1] = translation[1];
        matrix[3][2] = translation[2];

        Self { matrix, }
    }

    pub fn identity() -> Self {
        Self {
            matrix: Matrix4x4::identity(),
        }
    }

    pub fn transform_point(&self, point: &Vector3<S>) -> Vector3<S> {
        let _point = point.extend(S::one());
        let result = self.matrix * _point;
        result.contract()
    }

    pub fn transform_vector(&self, vector: &Vector3<S>) -> Vector3<S> {
        let _vector = vector.extend(S::zero());
        let result = self.matrix * _vector;
        result.contract()
    }

    pub fn to_matrix4x4_mut(&self, out: &mut Matrix4x4<S>) {
        *out = self.matrix;
    }

    pub fn to_matrix4x4(&self) -> Matrix4x4<S> {
        let mut out = Matrix4x4::zero();
        self.to_matrix4x4_mut(&mut out);

        out
    }

    pub fn inverse(&self) -> Option<Self> {
        self.matrix.inverse().map(|matrix| Self { matrix, })
    }
}

impl<S> ops::Mul<Transform3<S>> for Transform3<S> 
where
    S: SimdScalarFloat,
{
    type Output = Transform3<S>;

    fn mul(self, other: Transform3<S>) -> Self::Output {
        let matrix = self.matrix * other.matrix;

        Self::Output { matrix, }
    }
}

impl<S> ops::Mul<&Transform3<S>> for Transform3<S> 
where
    S: SimdScalarFloat,
{
    type Output = Transform3<S>;

    fn mul(self, other: &Transform3<S>) -> Self::Output {
        let matrix = self.matrix * other.matrix;

        Self::Output { matrix, }
    }
}

impl<S> ops::Mul<Transform3<S>> for &Transform3<S> 
where
    S: SimdScalarFloat,
{
    type Output = Transform3<S>;

    fn mul(self, other: Transform3<S>) -> Self::Output {
        let matrix = self.matrix * other.matrix;

        Self::Output { matrix, }
    }
}

impl<'a, 'b, S> ops::Mul<&'a Transform3<S>> for &'b Transform3<S> 
where
    S: SimdScalarFloat,
{
    type Output = Transform3<S>;

    fn mul(self, other: &'a Transform3<S>) -> Self::Output {
        let matrix = self.matrix * other.matrix;

        Self::Output { matrix, }
    }
}

