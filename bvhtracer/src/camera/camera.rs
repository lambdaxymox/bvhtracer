use crate::query::{
    Ray,
};
use cglinalg::{
    Degrees,
    Radians,
    Vector2,
    Vector3,
    Vector4,
    Magnitude,
    Matrix4x4, 
    Quaternion,
    SimdScalarFloat,
    Unit,
    Angle,
};
use std::fmt;


/// A type with this trait can be used as a camera model. A camera model
/// is a process of mapping incoming light rays from the camera's view space into
/// the camera model's canonical view volume.
pub trait CameraProjection {
    /// The scalar number type for the data model.
    type Scalar: SimdScalarFloat;
    /// The type representing the underlying projection from view space into 
    /// normalized device coordinates.
    type Projection;

    /// Exposed the underlying transformation that maps vector in the camera's
    /// view space into the canonical view volume of the camera.
    fn projection(&self) -> &Self::Projection;

    /// Get the location in eye space of the top left corner of the viewport.
    fn top_left_eye(&self) -> Vector3<Self::Scalar>;

    /// Get the location in eye space of the top right corner of the viewport.
    fn top_right_eye(&self) -> Vector3<Self::Scalar>;

    /// Get the location in eye space of the bottom left corner of the viewport.
    fn bottom_left_eye(&self) -> Vector3<Self::Scalar>;

    /// Get the location in eye space of the bottom right corner of the viewport.
    fn bottom_right_eye(&self) -> Vector3<Self::Scalar>;
}


/// A specification object describing a projection based on the `near` plane, 
/// the `far` plane and the vertical field of view angle `fovy` and the 
/// horizontal/vertical aspect ratio `aspect`.
///
/// We assume the following constraints to make a useful transformation.
/// ```text
/// 0 radians < fovy < pi radians
/// aspect > 0
/// near < far (along the negative z-axis)
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SymmetricFovSpec<S> {
    /// The vertical field of view angle of the viewport.
    fovy: Degrees<S>,
    /// The ratio of the horizontal width to the vertical height.
    aspect: S,
    /// The position of the near plane along the **negative z-axis**.
    near: S,
    /// The position of the far plane along the **negative z-axis**.
    far: S,
}

impl<S> SymmetricFovSpec<S> {
    #[inline]
    pub const fn new(fovy: Degrees<S>, aspect: S, near: S, far: S) -> Self {
        Self {
            fovy: fovy,
            aspect: aspect,
            near: near,
            far: far,
        }
    }
}

/// A projection based on arbitrary `left`, `right`, `bottom`, `top`, `near`, and 
/// `far` planes.
///
/// We assume the following constraints to construct a useful projection
/// ```text
/// left   < right
/// bottom < top
/// near   < far   (along the negative z-axis)
/// ```
/// Each parameter in the specification is a description of the position along
/// an axis of a plane that the axis is perpendicular to.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BoxSpec<S> {
    /// The horizontal position of the left-hand plane in camera space.
    /// The left-hand plane is a plane parallel to the **yz-plane** at
    /// the origin.
    left: S,
    /// The horizontal position of the right-hand plane in camera space.
    /// The right-hand plane is a plane parallel to the **yz-plane** at
    /// the origin.
    right: S,
    /// The vertical position of the bottom plane in camera space.
    /// The bottom plane is a plane parallel to the **xz-plane** at the origin.
    bottom: S,
    /// The vertical position of the top plane in camera space.
    /// the top plane is a plane parallel to the **xz-plane** at the origin.
    top: S,
    /// The distance along the **negative z-axis** of the near plane from the eye.
    /// The near plane is a plane parallel to the **xy-plane** at the origin.
    near: S,
    /// the distance along the **negative z-axis** of the far plane from the eye.
    /// The far plane is a plane parallel to the **xy-plane** at the origin.
    far: S,
}

impl<S> BoxSpec<S> {
    #[inline]
    pub const fn new(left: S, right: S, bottom: S, top: S, near: S, far: S) -> Self {
        Self {
            left: left,
            right: right,
            bottom: bottom,
            top: top,
            near: near,
            far: far,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AsymmetricFovSpec<S> {
    fovy_top: Degrees<S>,
    fovy_bottom: Degrees<S>,
    fovx_left: Degrees<S>,
    fovx_right: Degrees<S>,
    near: S,
    far: S,
}

impl<S> AsymmetricFovSpec<S> {
    #[inline]
    pub const fn new(
        fovy_top: Degrees<S>, 
        fovy_bottom: Degrees<S>, 
        fovx_left: Degrees<S>, 
        fovx_right: Degrees<S>, 
        near: S, 
        far: S
    ) -> Self 
    {
        Self { fovy_top, fovy_bottom, fovx_left, fovx_right, near, far, }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Frustum<S> {
    top_left: Vector2<S>,
    extent: Vector2<S>, 
    fovy_top: Degrees<S>,
    fovy_bottom: Degrees<S>,
    fovx_left: Degrees<S>,
    fovx_right: Degrees<S>,
    aspect: S,
    near: S,
    far: S,
}

impl<S> Frustum<S>
where
    S: SimdScalarFloat,
{
    #[inline]
    pub const fn near(&self) -> S {
        self.near
    }

    #[inline]
    pub const fn far(&self) -> S {
        self.far
    }

    #[inline]
    pub const fn aspect(&self) -> S {
        self.aspect
    }

    #[inline]
    pub fn fovy(&self) -> Degrees<S> {
        self.fovy_top + self.fovy_bottom
    }

    #[inline]
    pub fn fovx(&self) -> Degrees<S> {
        self.fovx_left + self.fovx_right
    }

    #[inline]
    pub fn top_left_eye(&self) -> Vector3<S> {
        Vector3::new(self.top_left.x, self.top_left.y, -self.near)
    }

    #[inline]
    pub fn top_right_eye(&self) -> Vector3<S> {
        Vector3::new(self.top_left.x + self.extent.x, self.top_left.y, -self.near)
    }

    #[inline]
    pub fn bottom_left_eye(&self) -> Vector3<S> {
        Vector3::new(self.top_left.x, self.top_left.y - self.extent.y, -self.near)
    }

    #[inline]
    pub fn bottom_right_eye(&self) -> Vector3<S> {
        Vector3::new(self.top_left.x + self.extent.x, self.top_left.y - self.extent.y, -self.near)
    }
}

impl<S> From<SymmetricFovSpec<S>> for Frustum<S>
where
    S: SimdScalarFloat,
{
    fn from(spec: SymmetricFovSpec<S>) -> Frustum<S> {
        let two = S::one() + S::one();
        let fovy_over_two = spec.fovy / two;
        let tan_fovy_over_two = Degrees::tan(fovy_over_two);
        let top = spec.near * tan_fovy_over_two;
        let bottom = -top;
        let left = -spec.aspect * top;
        let right = spec.aspect * top;
        let top_left = Vector2::new(left, top);
        let extent = Vector2::new(right - left, top - bottom);
        let fovy_top = fovy_over_two;
        let fovy_bottom = fovy_over_two;
        let tan_fovx_over_two = right / spec.near;
        let fovx_over_two = S::atan(tan_fovx_over_two);
        let fovx_left = Degrees::from(Radians(fovx_over_two));
        let fovx_right = Degrees::from(Radians(fovx_over_two));

        Frustum {
            top_left: top_left,
            extent: extent,
            fovy_top: fovy_top,
            fovy_bottom: fovy_bottom,
            fovx_left: fovx_left,
            fovx_right: fovx_right,
            aspect: spec.aspect,
            near: spec.near,
            far: spec.far,
        }
    }
}

impl<S> From<&SymmetricFovSpec<S>> for Frustum<S>
where
    S: SimdScalarFloat,
{
    fn from(spec: &SymmetricFovSpec<S>) -> Frustum<S> {
        let two = S::one() + S::one();
        let fovy_over_two = spec.fovy / two;
        let tan_fovy_over_two = Degrees::tan(fovy_over_two);
        let top = spec.near * tan_fovy_over_two;
        let bottom = -top;
        let left = -spec.aspect * top;
        let right = spec.aspect * top;
        let top_left = Vector2::new(left, top);
        let extent = Vector2::new(right - left, top - bottom);
        let fovy_top = fovy_over_two;
        let fovy_bottom = fovy_over_two;
        let tan_fovx_over_two = right / spec.near;
        let fovx_over_two = S::atan(tan_fovx_over_two);
        let fovx_left = Degrees::from(Radians(fovx_over_two));
        let fovx_right = Degrees::from(Radians(fovx_over_two));

        Frustum {
            top_left: top_left,
            extent: extent,
            fovy_top: fovy_top,
            fovy_bottom: fovy_bottom,
            fovx_left: fovx_left,
            fovx_right: fovx_right,
            aspect: spec.aspect,
            near: spec.near,
            far: spec.far,
        }
    }
}

impl<S> From<AsymmetricFovSpec<S>> for Frustum<S>
where
    S: SimdScalarFloat,
{
    fn from(spec: AsymmetricFovSpec<S>) -> Frustum<S> {
        let left = spec.near * Degrees::tan(spec.fovx_left);
        let right = spec.near * Degrees::tan(spec.fovx_right);
        let top = spec.near * Degrees::tan(spec.fovy_top);
        let bottom = spec.near * Degrees::tan(spec.fovy_bottom);
        let top_left = Vector2::new(left, top);
        let extent = Vector2::new(right - left, top - bottom);
        let aspect = (right - left) / (top - bottom);

        Frustum {
            top_left: top_left,
            extent: extent,
            fovy_top: spec.fovy_top,
            fovy_bottom: spec.fovy_bottom,
            fovx_left: spec.fovx_left,
            fovx_right: spec.fovx_right,
            aspect: aspect,
            near: spec.near,
            far: spec.far,
        }
    }
}

impl<S> From<&AsymmetricFovSpec<S>> for Frustum<S>
where
    S: SimdScalarFloat,
{
    fn from(spec: &AsymmetricFovSpec<S>) -> Frustum<S> {
        let left = spec.near * Degrees::tan(spec.fovx_left);
        let right = spec.near * Degrees::tan(spec.fovx_right);
        let top = spec.near * Degrees::tan(spec.fovy_top);
        let bottom = spec.near * Degrees::tan(spec.fovy_bottom);
        let top_left = Vector2::new(left, top);
        let extent = Vector2::new(right - left, top - bottom);
        let aspect = (right - left) / (top - bottom);

        Frustum {
            top_left: top_left,
            extent: extent,
            fovy_top: spec.fovy_top,
            fovy_bottom: spec.fovy_bottom,
            fovx_left: spec.fovx_left,
            fovx_right: spec.fovx_right,
            aspect: aspect,
            near: spec.near,
            far: spec.far,
        }
    }
}

impl<S> From<BoxSpec<S>> for Frustum<S>
where
    S: SimdScalarFloat,
{
    fn from(spec: BoxSpec<S>) -> Frustum<S> {
        let top_left = Vector2::new(spec.left, spec.top);
        let extent = Vector2::new(spec.right - spec.left, spec.top - spec.bottom);
        let aspect = extent.x / extent.y;
        let fovy_top = Degrees::atan(spec.top / spec.near);
        let fovy_bottom = Degrees::atan(-spec.bottom / spec.near);
        let fovx_left = Degrees::atan(-spec.left / spec.near);
        let fovx_right = Degrees::atan(spec.right / spec.near);

        Frustum {
            top_left: top_left,
            extent: extent,
            fovy_top: fovy_top,
            fovy_bottom: fovy_bottom,
            fovx_left: fovx_left,
            fovx_right: fovx_right,
            aspect: aspect,
            near: spec.near,
            far: spec.far,
        }
    }
}

impl<S> From<&BoxSpec<S>> for Frustum<S>
where
    S: SimdScalarFloat,
{
    fn from(spec: &BoxSpec<S>) -> Frustum<S> {
        let top_left = Vector2::new(spec.left, spec.top);
        let extent = Vector2::new(spec.right - spec.left, spec.top - spec.bottom);
        let aspect = extent.x / extent.y;
        let fovy_top = Degrees::atan(spec.top / spec.near);
        let fovy_bottom = Degrees::atan(-spec.bottom / spec.near);
        let fovx_left = Degrees::atan(-spec.left / spec.near);
        let fovx_right = Degrees::atan(spec.right / spec.near);

        Frustum {
            top_left: top_left,
            extent: extent,
            fovy_top: fovy_top,
            fovy_bottom: fovy_bottom,
            fovx_left: fovx_left,
            fovx_right: fovx_right,
            aspect: aspect,
            near: spec.near,
            far: spec.far,
        }
    }
}

/// A perspective projection transformation for converting from camera space to
/// normalized device coordinates.
///
/// Orthographic projections differ from perspective projections because
/// orthographic projections keeps parallel lines parallel, whereas perspective 
/// projections preserve the perception of distance. Perspective 
/// projections preserve the spatial ordering of points in the distance they 
/// are located from the viewing plane. This property of perspective projection 
/// transformations is important for operations such as z-buffering and 
/// occlusion detection.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PerspectiveProjection<S> {
    /// The underlying view volume geometry for the camera projection.
    frustum: Frustum<S>,
    /// The underlying perspective projection matrix.
    matrix: Matrix4x4<S>,
}

impl<S> PerspectiveProjection<S> {
    /// Returns a reference to the underlying perspective projection matrix.
    #[inline]
    pub fn to_matrix(&self) -> &Matrix4x4<S> {
        &self.matrix
    }
}

impl<S> fmt::Display for PerspectiveProjection<S> 
where 
    S: fmt::Display
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "PerspectiveProjection [{}]",
            self.matrix
        )
    }
}

impl<S> From<BoxSpec<S>> for PerspectiveProjection<S>
where
    S: SimdScalarFloat
{
    #[inline]
    fn from(spec: BoxSpec<S>) -> Self {
        let frustum = Frustum::from(spec);
        let matrix = Matrix4x4::from_perspective(
            spec.left, 
            spec.right, 
            spec.bottom, 
            spec.top,
            spec.near,
            spec.far
        );

        Self {
            frustum: frustum,
            matrix: matrix,
        }
    }
}

impl<S> From<&BoxSpec<S>> for PerspectiveProjection<S>
where
    S: SimdScalarFloat
{
    #[inline]
    fn from(spec: &BoxSpec<S>) -> Self {
        let frustum = Frustum::from(spec);
        let matrix = Matrix4x4::from_perspective(
            spec.left, 
            spec.right, 
            spec.bottom, 
            spec.top,
            spec.near,
            spec.far
        );

        Self {
            frustum: frustum,
            matrix: matrix,
        }
    }
}

impl<S> From<SymmetricFovSpec<S>> for PerspectiveProjection<S>
where
    S: SimdScalarFloat
{
    #[inline]
    fn from(spec: SymmetricFovSpec<S>) -> Self {
        let frustum = Frustum::from(spec);
        let matrix = Matrix4x4::from_perspective_fov(
            spec.fovy, 
            spec.aspect, 
            spec.near, 
            spec.far
        );

        Self {
            frustum: frustum,
            matrix: matrix,
        }
    }
}

impl<S> From<&SymmetricFovSpec<S>> for PerspectiveProjection<S>
where
    S: SimdScalarFloat
{
    #[inline]
    fn from(spec: &SymmetricFovSpec<S>) -> Self {
        let frustum = Frustum::from(spec);
        let matrix = Matrix4x4::from_perspective_fov(
            spec.fovy, 
            spec.aspect, 
            spec.near, 
            spec.far
        );

        Self {
            frustum: frustum,
            matrix: matrix,
        }
    }
}

impl<S> From<AsymmetricFovSpec<S>> for PerspectiveProjection<S>
where
    S: SimdScalarFloat
{
    #[inline]
    fn from(spec: AsymmetricFovSpec<S>) -> Self {
        let frustum = Frustum::from(spec);
        let top_left = frustum.top_left_eye();
        let bottom_right = frustum.bottom_right_eye();
        let near = frustum.near();
        let far = frustum.far();
        let matrix = Matrix4x4::from_perspective(
            top_left.x, 
            bottom_right.x, 
            bottom_right.y, 
            top_left.y,
            near,
            far
        );

        Self {
            frustum: frustum,
            matrix: matrix,
        }
    }
}

impl<S> From<&AsymmetricFovSpec<S>> for PerspectiveProjection<S>
where
    S: SimdScalarFloat
{
    #[inline]
    fn from(spec: &AsymmetricFovSpec<S>) -> Self {
        let frustum = Frustum::from(spec);
        let top_left = frustum.top_left_eye();
        let bottom_right = frustum.bottom_right_eye();
        let near = frustum.near();
        let far = frustum.far();
        let matrix = Matrix4x4::from_perspective(
            top_left.x, 
            bottom_right.x, 
            bottom_right.y, 
            top_left.y,
            near,
            far
        );

        Self {
            frustum: frustum,
            matrix: matrix,
        }
    }
}

impl<S> CameraProjection for PerspectiveProjection<S>
where 
    S: SimdScalarFloat
{
    type Scalar = S;
    type Projection = Matrix4x4<S>;

    #[inline]
    fn projection(&self) -> &Self::Projection {
        &self.matrix
    }

    fn top_left_eye(&self) -> Vector3<Self::Scalar> {
        self.frustum.top_left_eye()
    }

    fn top_right_eye(&self) -> Vector3<Self::Scalar> {
        self.frustum.top_right_eye()
    }

    fn bottom_left_eye(&self) -> Vector3<Self::Scalar> {
        self.frustum.bottom_left_eye()
    }

    fn bottom_right_eye(&self) -> Vector3<Self::Scalar> {
        self.frustum.bottom_right_eye()
    }
}


/// An orthographic projection transformation for converting from camera space to
/// normalized device coordinates. 
///
/// Orthographic projections differ from perspective projections in that 
/// orthographic projections keeps parallel lines parallel, whereas perspective 
/// projections preserve the perception of distance. Perspective 
/// projections preserve the spatial ordering in the distance that points are 
/// located from the viewing plane.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct OrthographicProjection<S> {
    /// The underlying view volume geometry for the camera projection.
    frustum: Frustum<S>,
    /// The underlying matrix that implements the orthographic projection.
    matrix: Matrix4x4<S>,
}

impl<S> OrthographicProjection<S>
where 
    S: SimdScalarFloat
{
    /// Get the underlying matrix implementing the orthographic transformation.
    #[inline]
    pub fn to_matrix(&self) -> &Matrix4x4<S> {
        &self.matrix
    }
}

impl<S> fmt::Display for OrthographicProjection<S> 
where 
    S: fmt::Display
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "OrthographicProjection [{}]",
            self.matrix
        )
    }
}

impl<S> From<BoxSpec<S>> for OrthographicProjection<S>
where
    S: SimdScalarFloat
{
    #[inline]
    fn from(spec: BoxSpec<S>) -> Self {
        let frustum = Frustum::from(spec);
        let matrix = Matrix4x4::from_orthographic(
            spec.left, 
            spec.right, 
            spec.bottom, 
            spec.top,
            spec.near,
            spec.far
        );

        Self {
            frustum: frustum, 
            matrix: matrix,
        }
    }
}

impl<S> From<&BoxSpec<S>> for OrthographicProjection<S>
where
    S: SimdScalarFloat
{
    #[inline]
    fn from(spec: &BoxSpec<S>) -> Self {
        let frustum = Frustum::from(spec);
        let matrix = Matrix4x4::from_orthographic(
            spec.left, 
            spec.right, 
            spec.bottom, 
            spec.top,
            spec.near,
            spec.far
        );

        Self {
            frustum: frustum, 
            matrix: matrix,
        }
    }
}

impl<S> From<SymmetricFovSpec<S>> for OrthographicProjection<S>
where
    S: SimdScalarFloat
{
    #[inline]
    fn from(spec: SymmetricFovSpec<S>) -> Self {
        let frustum = Frustum::from(spec);
        let matrix = Matrix4x4::from_orthographic_fov(
            spec.fovy, 
            spec.aspect, 
            spec.near,
            spec.far
        );

        Self {
            frustum: frustum,
            matrix: matrix,
        }
    }
}

impl<S> From<&SymmetricFovSpec<S>> for OrthographicProjection<S>
where
    S: SimdScalarFloat
{
    #[inline]
    fn from(spec: &SymmetricFovSpec<S>) -> Self {
        let frustum = Frustum::from(spec);
        let matrix = Matrix4x4::from_orthographic_fov(
            spec.fovy, 
            spec.aspect, 
            spec.near,
            spec.far
        );

        Self {
            frustum: frustum,
            matrix: matrix,
        }
    }
}

impl<S> From<AsymmetricFovSpec<S>> for OrthographicProjection<S>
where
    S: SimdScalarFloat
{
    #[inline]
    fn from(spec: AsymmetricFovSpec<S>) -> Self {
        let frustum = Frustum::from(spec);
        let top_left = frustum.top_left_eye();
        let bottom_right = frustum.bottom_right_eye();
        let near = frustum.near();
        let far = frustum.far();
        let matrix = Matrix4x4::from_orthographic(
            top_left.x, 
            bottom_right.x, 
            bottom_right.y, 
            top_left.y,
            near,
            far
        );

        Self {
            frustum: frustum,
            matrix: matrix,
        }
    }
}

impl<S> From<&AsymmetricFovSpec<S>> for OrthographicProjection<S>
where
    S: SimdScalarFloat
{
    #[inline]
    fn from(spec: &AsymmetricFovSpec<S>) -> Self {
        let frustum = Frustum::from(spec);
        let top_left = frustum.top_left_eye();
        let bottom_right = frustum.bottom_right_eye();
        let near = frustum.near();
        let far = frustum.far();
        let matrix = Matrix4x4::from_orthographic(
            top_left.x, 
            bottom_right.x, 
            bottom_right.y, 
            top_left.y,
            near,
            far
        );

        Self {
            frustum: frustum,
            matrix: matrix,
        }
    }
}

impl<S> CameraProjection for OrthographicProjection<S> 
where 
    S: SimdScalarFloat
{
    type Scalar = S;
    type Projection = Matrix4x4<S>;

    #[inline]
    fn projection(&self) -> &Self::Projection {
        &self.matrix
    }

    fn top_left_eye(&self) -> Vector3<Self::Scalar> {
        self.frustum.top_left_eye()
    }

    fn top_right_eye(&self) -> Vector3<Self::Scalar> {
        self.frustum.top_right_eye()
    }

    fn bottom_left_eye(&self) -> Vector3<Self::Scalar> {
        self.frustum.bottom_left_eye()
    }

    fn bottom_right_eye(&self) -> Vector3<Self::Scalar> {
        self.frustum.bottom_right_eye()
    }
}

/// A specification describing a rigid body transformation for the attitude 
/// (position and orientation) of a camera. The spec describes the location, 
/// local coordinate system, and rotation axis for the camera in world space.
/// The coordinate transformation is right-handed orthonormal transformation.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CameraAttitudeSpec<S> {
    /// The location of the camera eye position in world space.
    position: Vector3<S>,
    /// The direction of the **negative z-axis** (forward axis) of the camera.
    forward: Vector3<S>,
    /// The direction of the **positive x-axis** (right axis) of the camera.
    right: Vector3<S>,
    /// The direction of the **positive y-axis** (up axis) of the camera.
    up: Vector3<S>,
    /// The **axis of rotation** of the camera. It is not necessary that 
    /// the axis of rotation of the camera be the same as one of the coordinate
    /// axes.
    axis: Vector3<S>,
}

impl<S> CameraAttitudeSpec<S> 
where 
    S: SimdScalarFloat
{
    /// Construct a new camera attitude specification.
    #[inline]
    pub const fn new(
        position: Vector3<S>,
        forward: Vector3<S>,
        right: Vector3<S>,
        up: Vector3<S>,
        axis: Vector3<S>) -> Self {

        Self {
            position: position,
            forward: forward,
            right: right,
            up: up,
            axis: axis,
        }
    }
}

/// This type contains all the data for tracking the position and orientation
/// of a camera in world space as well as for transforming vectors from world 
/// space to the camera's view space. The camera attitude here uses a 
/// right-handed coordinate system facing along the camera's **negative z-axis**.
/// The coordinate system is a right-handed coordinate system with orthonormal
/// basis vectors.
#[repr(C)]
#[derive(Clone, Debug)]
struct CameraAttitude<S> {
    /// The world space position of the camera.
    position: Vector3<S>,
    /// The distance from the camera eye perpendicular
    forward: Vector4<S>,
    /// The horizontal axis of the camera's viewing plane.
    right: Vector4<S>,
    /// The vertical axis of the camera's viewing plane.
    up: Vector4<S>,
    /// The **axis of rotation** of the camera. It is not necessary that 
    /// the axis of rotation of the camera be the same as one of the coordinate
    /// axes.
    axis: Quaternion<S>,
    /// The translation matrix mapping objects from the world space coordinate
    /// frame to the coordinate frame centered at the eye position of the camera.
    translation_matrix: Matrix4x4<S>,
    /// The rotation matrix rotating the a vector in world space to the coordinate
    /// system of the camera's view space.
    rotation_matrix: Matrix4x4<S>,
    /// The viewing matrix of the camera mapping the complete translation + rotation
    /// of the camera. The transformation direction is from world space to eye space.
    view_matrix: Matrix4x4<S>,
    /// The inverse of the viewing matrix of the camera mapping the the complete 
    /// translation + rotation of the camera. The transformation direction is 
    /// from eye space to world space.
    view_matrix_inv: Matrix4x4<S>,
}

impl<S> CameraAttitude<S> 
where 
    S: SimdScalarFloat 
{
    /// Construct the camera's viewing transformation from its specification. 
    fn from_spec(spec: &CameraAttitudeSpec<S>) -> Self {
        let axis = Quaternion::from_parts(S::zero(), spec.axis);
        let translation_matrix = Matrix4x4::from_affine_translation(
            &(-spec.position)
        );
        let rotation_matrix = Matrix4x4::new(
            spec.right.x, spec.up.x, -spec.forward.x, S::zero(),
            spec.right.y, spec.up.y, -spec.forward.y, S::zero(),
            spec.right.z, spec.up.z, -spec.forward.z, S::zero(),
            S::zero(),    S::zero(), S::zero(),       S::one()
        );
        let view_matrix = rotation_matrix * translation_matrix;
        let view_matrix_inv = view_matrix.inverse().unwrap();

        Self {
            position: spec.position,
            forward: spec.forward.extend(S::zero()),
            right: spec.right.extend(S::zero()),
            up: spec.up.extend(S::zero()),
            axis: axis,
            translation_matrix: translation_matrix,
            rotation_matrix: rotation_matrix,
            view_matrix: view_matrix,
            view_matrix_inv: view_matrix_inv,
        }
    }

    /// Get the camera's up direction in camera space.
    #[inline]
    fn up_axis_eye(&self) -> Vector3<S> {
        let zero = S::zero();
        let one = S::one();

        Vector3::new(zero, one, zero)
    }
        
    /// Get the camera's right axis in camera space.
    #[inline]
    fn right_axis_eye(&self) -> Vector3<S> {
        let zero = S::zero();
        let one = S::one();
        
        Vector3::new(one, zero,zero)
    }
        
    /// Get the camera's forward axis in camera space.
    #[inline]
    fn forward_axis_eye(&self) -> Vector3<S> {
        let zero = S::zero();
        let one = S::one();
        
        Vector3::new(zero, zero, -one)
    }

    /// Get the underlying viewing matrix of the camera.
    #[inline]
    fn view_matrix(&self) -> &Matrix4x4<S> {
        &self.view_matrix
    }

    /// Get the underlying inverse viewing matrix for the camera.
    #[inline]
    fn view_matrix_inv(&self) -> &Matrix4x4<S> {
        &self.view_matrix_inv
    }
}


/// A camera that maps light rays from a scene to pixels in a viewport. 
///
/// This camera model has two components:
///
/// * A camera model for mapping light rays to images. This can model many kinds
///   of ranging from the usual orthographic and perspective pinhole cameras, to
///   more sophisticated camera models including effects like depth of field, etc.
/// * The camera's attitude: the attitude is the camera's orientation and position in
///   world space, modeled as a rotation and a translation.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Camera<S, P> {
    /// The camera's model for mapping light rays to normalized device
    /// coordinates.
    projection: P,
    /// The position and orientation of the camera in world space.
    attitude: CameraAttitude<S>,
}

impl<S, P> Camera<S, P> 
where 
    S: SimdScalarFloat,
    P: CameraProjection<Scalar = S>,
{
    /// Construct a new camera.
    pub fn new<PSpec>(projection_spec: PSpec, attitude_spec: &CameraAttitudeSpec<S>) -> Self 
    where
        PSpec: Into<P>,
    {
        Self {
            projection: projection_spec.into(),
            attitude: CameraAttitude::from_spec(attitude_spec),
        }
    }

    /// Get the camera's position in world space.
    #[inline]
    pub fn position(&self) -> Vector3<S> { 
        self.attitude.position
    }
    
    /// Get the camera's up direction in world space.
    #[inline]
    pub fn up_axis_world(&self) -> Vector3<S> {
        self.attitude.up.contract()
    }
    
    /// Get the camera's right axis in world space.
    #[inline]
    pub fn right_axis_world(&self) -> Vector3<S> {
        self.attitude.right.contract()
    }
    
    /// Get the camera's forward axis in world space.
    #[inline]
    pub fn forward_axis_world(&self) -> Vector3<S> {
        self.attitude.forward.contract()
    }
    
    /// Get the camera's **vertical y-axis** in camera view space.
    #[inline]
    pub fn up_axis_eye(&self) -> Vector3<S> {
        self.attitude.up_axis_eye()
    }
        
    /// Get the camera's **horizontal x-axis** in camera view space.
    #[inline]
    pub fn right_axis_eye(&self) -> Vector3<S> {
        self.attitude.right_axis_eye()
    }
        
    /// Get the camera's **forward z-axis** in camera view space.
    #[inline]
    pub fn forward_axis_eye(&self) -> Vector3<S> {
        self.attitude.forward_axis_eye()
    }
    
    /// Get the camera's axis of rotation.
    #[inline]
    pub fn rotation_axis(&self) -> Vector3<S> {
        self.attitude.axis.v
    }

    /// Get the camera's viewing matrix.
    #[inline]
    pub fn view_matrix(&self) -> &Matrix4x4<S> {
        self.attitude.view_matrix()
    }

    #[inline]
    pub fn view_matrix_inv(&self) -> &Matrix4x4<S> {
        self.attitude.view_matrix_inv()
    }

    /// Return the underlying projection the camera uses to transform from
    /// view space to the camera's canonical view volume.
    #[inline]
    pub fn projection(&self) -> &P::Projection {
        self.projection.projection()
    }

    pub fn top_left_eye(&self) -> Vector3<S> {
        self.projection.top_left_eye()
    }

    pub fn top_right_eye(&self) -> Vector3<S> {
        self.projection.top_right_eye()
    }

    pub fn bottom_left_eye(&self) -> Vector3<S> {
        self.projection.bottom_left_eye()
    }

    pub fn bottom_right_eye(&self) -> Vector3<S> {
        self.projection.bottom_right_eye()
    }

    pub fn get_ray_eye(&self, u: S, v: S) -> Ray<S> {
        let ray_origin = Vector3::zero();
        let pixel_position = ray_origin + self.top_left_eye() + 
            (self.top_right_eye() - self.top_left_eye()) * u + 
            (self.bottom_left_eye() - self.top_left_eye()) * v;
        let ray_direction = (pixel_position - ray_origin).normalize();

        Ray::from_origin_dir(ray_origin, ray_direction)
    }

    pub fn get_ray_world(&self, u: S, v: S) -> Ray<S> {
       let ray_eye = self.get_ray_eye(u, v);
       let ray_origin_world = (self.attitude.view_matrix_inv * ray_eye.origin.extend(S::one())).contract();
       let ray_direction_world = (self.attitude.view_matrix_inv * ray_eye.direction.extend(S::zero())).contract();

       Ray::from_origin_dir(ray_origin_world, ray_direction_world)
    }
}


#[cfg(test)]
mod attitude_tests1 {
    use super::*;

    fn attitude() -> CameraAttitude<f64> {
        let attitude_spec = CameraAttitudeSpec::new(
            Vector3::new(0_f64, 0_f64, -5_f64),
            Vector3::unit_z(),
            Vector3::unit_x(),
            -Vector3::unit_y(),
            Vector3::unit_z()
        );

        CameraAttitude::from_spec(&attitude_spec)
    }

    #[test]
    fn test_forward_axis_eye_to_world() {
        let attitude = attitude();
        let expected = Vector3::new(0_f64, 0_f64, 1_f64);
        let result = {
            let forward_eye = attitude.forward_axis_eye();
            let forward_world = attitude.view_matrix_inv() * forward_eye.extend(0_f64);
            forward_world.contract()
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_axis_eye_to_world() {
        let attitude = attitude();
        let expected = Vector3::new(1_f64, 0_f64, 0_f64);
        let result = {
            let right_eye = attitude.right_axis_eye();
            let right_world = attitude.view_matrix_inv() * right_eye.extend(0_f64);
            right_world.contract()
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_up_axis_eye_to_world() {
        let attitude = attitude();
        let expected = Vector3::new(0_f64, -1_f64, 0_f64);
        let result = {
            let up_eye = attitude.up_axis_eye();
            let up_world = attitude.view_matrix_inv() * up_eye.extend(0_f64);
            up_world.contract()
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_z_axis_eye_to_world() {
        let attitude = attitude();
        let expected = Vector3::new(0_f64, 0_f64, -1_f64);
        let result = {
            let z_axis_eye = Vector3::new(0_f64, 0_f64, 1_f64);
            let z_axis_world = attitude.view_matrix_inv() * z_axis_eye.extend(0_f64);
            z_axis_world.contract()
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_view_matrix() {
        let attitude = attitude();
        let expected = Matrix4x4::new(
            1_f64,  0_f64,  0_f64, 0_f64,
            0_f64, -1_f64,  0_f64, 0_f64,
            0_f64,  0_f64, -1_f64, 0_f64,
            0_f64,  0_f64, -5_f64, 1_f64
        );
        let result = attitude.view_matrix();

        assert_eq!(result, &expected);
    }

    #[test]
    fn test_view_matrix_inv() {
        let attitude = attitude();
        let expected = Matrix4x4::new(
            1_f64,  0_f64,  0_f64, 0_f64,
            0_f64, -1_f64,  0_f64, 0_f64,
            0_f64,  0_f64, -1_f64, 0_f64,
            0_f64,  0_f64, -5_f64, 1_f64
        ).inverse().unwrap();
        let result = attitude.view_matrix_inv();

        assert_eq!(result, &expected);
    }
}

#[cfg(test)]
mod attitude_tests2 {
    use super::*;

    fn attitude() -> CameraAttitude<f64> {
        let attitude_spec = CameraAttitudeSpec::new(
            Vector3::new(0_f64, 0_f64, 5_f64),
            -Vector3::unit_z(),
            Vector3::unit_x(),
            Vector3::unit_y(),
            Vector3::unit_z()
        );

        CameraAttitude::from_spec(&attitude_spec)
    }

    #[test]
    fn test_forward_axis_eye_to_world() {
        let attitude = attitude();
        let expected = Vector3::new(0_f64, 0_f64, -1_f64);
        let result = {
            let forward_eye = attitude.forward_axis_eye();
            let forward_world = attitude.view_matrix_inv() * forward_eye.extend(0_f64);
            forward_world.contract()
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_axis_eye_to_world() {
        let attitude = attitude();
        let expected = Vector3::new(1_f64, 0_f64, 0_f64);
        let result = {
            let right_eye = attitude.right_axis_eye();
            let right_world = attitude.view_matrix_inv() * right_eye.extend(0_f64);
            right_world.contract()
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_up_axis_eye_to_world() {
        let attitude = attitude();
        let expected = Vector3::new(0_f64, 1_f64, 0_f64);
        let result = {
            let up_eye = attitude.up_axis_eye();
            let up_world = attitude.view_matrix_inv() * up_eye.extend(0_f64);
            up_world.contract()
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_z_axis_eye_to_world() {
        let attitude = attitude();
        let expected = Vector3::new(0_f64, 0_f64, 1_f64);
        let result = {
            let z_axis_eye = Vector3::new(0_f64, 0_f64, 1_f64);
            let z_axis_world = attitude.view_matrix_inv() * z_axis_eye.extend(0_f64);
            z_axis_world.contract()
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_view_matrix() {
        let attitude = attitude();
        let expected = Matrix4x4::new(
            1_f64,  0_f64,  0_f64, 0_f64,
            0_f64,  1_f64,  0_f64, 0_f64,
            0_f64,  0_f64,  1_f64, 0_f64,
            0_f64,  0_f64, -5_f64, 1_f64
        );
        let result = attitude.view_matrix();

        assert_eq!(result, &expected);
    }

    #[test]
    fn test_view_matrix_inv() {
        let attitude = attitude();
        let expected = Matrix4x4::new(
            1_f64,  0_f64,  0_f64, 0_f64,
            0_f64,  1_f64,  0_f64, 0_f64,
            0_f64,  0_f64,  1_f64, 0_f64,
            0_f64,  0_f64, -5_f64, 1_f64
        ).inverse().unwrap();
        let result = attitude.view_matrix_inv();

        assert_eq!(result, &expected);
    }
}

