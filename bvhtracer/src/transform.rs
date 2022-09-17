use cglinalg::{
    Rotation3,
    SimdScalarFloat,
    Vector3,
    Matrix4x4,
    Unit,
    Radians, 
    Quaternion,
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
        matrix[0][1] *= scale[0];
        matrix[0][2] *= scale[0];
        
        matrix[1][0] *= scale[1];
        matrix[1][1] *= scale[1];
        matrix[1][2] *= scale[1];
        
        matrix[2][0] *= scale[2];
        matrix[2][1] *= scale[2];
        matrix[2][2] *= scale[2];
        
        Self { matrix, }
    }

    #[inline]
    pub fn identity() -> Self {
        Self {
            matrix: Matrix4x4::identity(),
        }
    }

    #[inline]
    pub fn from_scale(scale: S) -> Self {
        let matrix = Matrix4x4::from_affine_scale(scale);

        Self { matrix, }
    }

    #[inline]
    pub fn from_nonuniform_scale(scale: &Vector3<S>) -> Self {
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
        
        matrix[3][0] = translation[0];
        matrix[3][1] = translation[1];
        matrix[3][2] = translation[2];

        matrix[0][0] = scale[0];
        matrix[1][1] = scale[1];
        matrix[2][2] = scale[2];

        Self { matrix, }
    }

    #[inline]
    pub fn from_scale_axis_angle<A: Into<Radians<S>>>(scale: &Vector3<S>, axis: &Unit<Vector3<S>>, angle: A) -> Self 
    where
        A: Into<Radians<S>>,
    {
        let mut matrix = Matrix4x4::from_affine_axis_angle(axis, angle);
        
        matrix[0][0] *= scale[0]; 
        matrix[0][1] *= scale[0];
        matrix[0][2] *= scale[0];
        
        matrix[1][0] *= scale[1];
        matrix[1][1] *= scale[1];
        matrix[1][2] *= scale[1];
        
        matrix[2][0] *= scale[2];
        matrix[2][1] *= scale[2];
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

        matrix[3][0] = translation[0];
        matrix[3][1] = translation[1];
        matrix[3][2] = translation[2];

        matrix[0][0] *= scale[0]; 
        matrix[0][1] *= scale[0];
        matrix[0][2] *= scale[0];
        
        matrix[1][0] *= scale[1];
        matrix[1][1] *= scale[1];
        matrix[1][2] *= scale[1];
        
        matrix[2][0] *= scale[2];
        matrix[2][1] *= scale[2];
        matrix[2][2] *= scale[2];

        Self { matrix, }
    }

    #[inline]
    pub fn from_angle_x<A>(angle: A) -> Self 
    where
        A: Into<Radians<S>>
    {
        let matrix = Matrix4x4::from_affine_angle_x(angle);

        Self { matrix, }
    }

    #[inline]
    pub fn from_angle_y<A>(angle: A) -> Self 
    where
        A: Into<Radians<S>>
    {
        let matrix = Matrix4x4::from_affine_angle_y(angle);

        Self { matrix, }
    }

    #[inline]
    pub fn from_angle_z<A>(angle: A) -> Self 
    where
        A: Into<Radians<S>>
    {
        let matrix = Matrix4x4::from_affine_angle_z(angle);

        Self { matrix, }
    }

    // Implementation of Euler-Rodruiguez Formula.
    pub fn from_translation_rotation(translation: &Vector3<S>, rotation: &Quaternion<S>) -> Self {
        let mut matrix = Matrix4x4::zero();
        let zero = S::zero();
        let one = S::one();
        let two = one + one;
    
        matrix[0][0] = one - two * rotation.v.y * rotation.v.y - two * rotation.v.z * rotation.v.z;
        matrix[0][1] = two * rotation.v.x * rotation.v.y + two * rotation.s * rotation.v.z;
        matrix[0][2] = two * rotation.v.x * rotation.v.z - two * rotation.s * rotation.v.y;
        matrix[0][3] = zero;
        
        matrix[1][0] = two * rotation.v.x * rotation.v.y - two * rotation.s * rotation.v.z;
        matrix[1][1] = one - two * rotation.v.x * rotation.v.x - two * rotation.v.z * rotation.v.z;
        matrix[1][2] = two * rotation.v.y * rotation.v.z + two * rotation.s * rotation.v.x;
        matrix[1][3] = zero;
    
        matrix[2][0] = two * rotation.v.x * rotation.v.z + two * rotation.s * rotation.v.y;
        matrix[2][1] = two * rotation.v.y * rotation.v.z - two * rotation.s * rotation.v.x;
        matrix[2][2] = one - two * rotation.v.x * rotation.v.x - two * rotation.v.y * rotation.v.y;
        matrix[2][3] = zero;
    
        matrix[3][0] = translation.x;
        matrix[3][1] = translation.y;
        matrix[3][2] = translation.z;
        matrix[3][3] = one;

        Self { matrix, }
    }

    pub fn transform_point(&self, point: &Vector3<S>) -> Vector3<S> {
        let _point = point.extend(S::one());
        let result = self.matrix * _point;
        result.contract()
    }

    pub fn inverse_transform_point(&self, point: &Vector3<S>) -> Vector3<S> {
        let transform_inverse = self.inverse().unwrap();
        transform_inverse.transform_point(point)
    }

    pub fn transform_vector(&self, vector: &Vector3<S>) -> Vector3<S> {
        let _vector = vector.extend(S::zero());
        let result = self.matrix * _vector;
        result.contract()
    }

    pub fn inverse_transform_vector(&self, vector: &Vector3<S>) -> Vector3<S> {
        let transform_inverse = self.inverse().unwrap();
        transform_inverse.transform_vector(vector)
    }

    pub fn compute_matrix_mut(&self, out: &mut Matrix4x4<S>) {
        *out = self.matrix;
    }

    pub fn compute_matrix(&self) -> Matrix4x4<S> {
        let mut out = Matrix4x4::zero();
        self.compute_matrix_mut(&mut out);

        out
    }

    pub fn inverse(&self) -> Option<Self> {
        self.matrix.inverse().map(|matrix| Self { matrix, })
    }

    pub fn get_translation(&self) -> Vector3<S> {
        Vector3::new(self.matrix[3][0], self.matrix[3][1], self.matrix[3][2])
    }
}

impl<S> Default for Transform3<S> 
where
    S: SimdScalarFloat,
{
    fn default() -> Self {
        Self::identity()
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

