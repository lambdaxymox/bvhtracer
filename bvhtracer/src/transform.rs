use cglinalg::{
    Translation3,
    Rotation3,
    Scale3,
    SimdScalarFloat,
    Vector3,
    Point3,
    Matrix4x4,
};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform3<S> {
    pub scale: Scale3<S>,
    pub translation: Translation3<S>,
    pub rotation: Rotation3<S>,
}

impl<S> Transform3<S>
where
    S: SimdScalarFloat,
{
    #[inline]
    pub const fn new(scale: Scale3<S>, translation: Translation3<S>, rotation: Rotation3<S>) -> Self {
        Self { scale, translation, rotation, }
    }

    pub fn identity() -> Self {
        Self {
            scale: Scale3::identity(),
            translation: Translation3::identity(),
            rotation: Rotation3::identity(),
        }
    }

    pub fn transform_point(&self, point: &Vector3<S>) -> Vector3<S> {
        let _point = Point3::new(point.x, point.y, point.z);
        let scaled = self.scale.scale_point(&_point);
        let rotated = self.rotation.rotate_point(&scaled);
        let translated = self.translation.translate_point(&rotated);
        translated.to_vector()
    }

    pub fn transform_vector(&self, vector: &Vector3<S>) -> Vector3<S> {
        let scaled = self.scale.scale_vector(vector);
        let rotated = self.rotation.rotate_vector(&scaled);
        self.translation.translate_vector(&rotated)
    }

    pub fn to_matrix4x4_mut(&self, out: &mut Matrix4x4<S>) {
        let mut new_matrix = self.rotation.to_affine_matrix();
        let translation = self.translation.to_vector();
        let scale = self.scale.to_vector();
        new_matrix[3][0] = translation[0];
        new_matrix[3][1] = translation[1];
        new_matrix[3][2] = translation[2];
        new_matrix[0][0] *= scale[0];
        new_matrix[1][1] *= scale[1];
        new_matrix[2][2] *= scale[2];
        *out = new_matrix;
    }

    pub fn to_matrix4x4(&self) -> Matrix4x4<S> {
        let mut out = Matrix4x4::zero();
        self.to_matrix4x4_mut(&mut out);

        out
    }

    pub fn inverse(&self) -> Option<Self> {
        let transform_inv = Self::new(
            self.scale.inverse(),
            self.translation.inverse(),
            self.rotation.inverse(),
        );
        Some(transform_inv)
    }
}

