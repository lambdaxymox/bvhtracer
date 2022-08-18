use crate::query::{
    Ray,
};

use cglinalg::{
    Degrees,
    Radians,
    Vector3,
    Vector4,
    Magnitude,
    Matrix4x4, 
    Quaternion,
    SimdScalarFloat,
    Unit,
};

use std::fmt;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ActiveProjection {
    Perspective,
    Orthographic,
}

impl ActiveProjection {
    pub fn is_perspective(self) -> bool {
        self == ActiveProjection::Perspective
    }

    pub fn is_orthographic(self) -> bool {
        self == ActiveProjection::Orthographic
    }
}

/// A type with this trait can be used as a camera model. A camera model
/// is a process of mapping incoming light rays from the camera's view space into
/// the camera model's canonical view volume.
pub trait CameraModel {
    /// The scalar number type for the data model.
    type Scalar: SimdScalarFloat;
    /// The type containing the parameters for constructing the camera model.
    type Spec;
    /// The type representing the underlying projection from view space into 
    /// normalized device coordinates.
    type Projection;

    /// Construct a camera model from a description of the 
    /// camera model's parameters.
    fn from_spec(spec: &Self::Spec) -> Self;

    /// Exposed the underlying transformation that maps vector in the camera's
    /// view space into the canonical view volume of the camera.
    fn projection(&self) -> &Self::Projection;

    /// Update the camera model based on changes in the viewport dimensions.
    fn update_viewport(&mut self, width: usize, height: usize);

    /// Get the location in eye space of the top left corner of the viewport.
    fn top_left_eye(&self) -> Vector3<Self::Scalar>;

    /// Get the location in eye space of the top right corner of the viewport.
    fn top_right_eye(&self) -> Vector3<Self::Scalar>;

    /// Get the location in eye space of the bottom left corner of the viewport.
    fn bottom_left_eye(&self) -> Vector3<Self::Scalar>;

    /// Get the location in eye space of the bottom right corner of the viewport.
    fn bottom_right_eye(&self) -> Vector3<Self::Scalar>;

    /// Switch to a different projection in the underlying camera model.
    fn set_active(&mut self, projection: ActiveProjection);
}


/// A perspective projection based on the `near` plane, the `far` plane and 
/// the vertical field of view angle `fovy` and the horizontal/vertical aspect 
/// ratio `aspect`.
///
/// We assume the following constraints to make a useful perspective projection 
/// transformation.
/// ```text
/// 0 radians < fovy < pi radians
/// aspect > 0
/// near < far (along the negative z-axis)
/// ```
/// This perspective projection model imposes some constraints on the more 
/// general perspective specification based on the arbitrary planes. The `fovy` 
/// parameter combined with the aspect ratio `aspect` ensures that the top and 
/// bottom planes are the same distance from the eye position along the vertical 
/// axis on opposite side. They ensure that the `left` and `right` planes are 
/// equidistant from the eye on opposite sides along the horizontal axis. 
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PerspectiveFovSpec<S> {
    /// The vertical field of view angle of the perspective transformation
    /// viewport.
    fovy: Degrees<S>,
    /// The ratio of the horizontal width to the vertical height.
    aspect: S,
    /// The position of the near plane along the **negative z-axis**.
    near: S,
    /// The position of the far plane along the **negative z-axis**.
    far: S,
}

impl<S> PerspectiveFovSpec<S> {
    /// Construct a new perspective projection operation specification
    /// based on the vertical field of view angle `fovy`, the `near` plane, the 
    /// `far` plane, and aspect ratio `aspect`.
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

impl<S> fmt::Display for PerspectiveFovSpec<S> 
where 
    S: fmt::Display 
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
       write!(
           formatter,
           "PerspectiveFovSpec [fovy={}, aspect={}, near={}, far={}]",
           self.fovy, self.aspect, self.near, self.far
       )
    }
}


/// A perspective projection transformation for converting from camera space to
/// normalized device coordinates based on the perspective field of view model.
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
pub struct PerspectiveFovProjection<S> {
    /// The vertical field of view angle of the perspective transformation
    /// viewport.
    fovy: Degrees<S>,
    /// The ratio of the horizontal width to the vertical height.
    aspect: S,
    /// The position of the near plane along the **negative z-axis**.
    near: S,
    /// The position of the far plane along the **negative z-axis**.
    far: S,
    /// The underlying perspective projection transformation.
    matrix: Matrix4x4<S>,
}

impl<S> PerspectiveFovProjection<S> {
    /// Returns a reference to the underlying perspective projection matrix.
    #[inline]
    pub fn to_matrix(&self) -> &Matrix4x4<S> {
        &self.matrix
    }
}

impl<S> fmt::Display for PerspectiveFovProjection<S> 
where 
    S: fmt::Display 
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "PerspectiveFovProjection [{}]",
            self.matrix
        )
    }
}

impl<S> CameraModel for PerspectiveFovProjection<S> 
where 
    S: SimdScalarFloat 
{
    type Scalar = S;
    type Spec = PerspectiveFovSpec<S>;
    type Projection = Matrix4x4<S>;

    #[inline]
    fn from_spec(spec: &Self::Spec) -> Self {
        let matrix = Matrix4x4::from_perspective_fov(
            spec.fovy, 
            spec.aspect, 
            spec.near, 
            spec.far
        );

        Self {
            fovy: spec.fovy,
            aspect: spec.aspect,
            near: spec.near,
            far: spec.far,
            matrix: matrix,
        }
    }

    #[inline]
    fn projection(&self) -> &Self::Projection {
        &self.matrix
    }

    fn update_viewport(&mut self, width: usize, height: usize) {
        let width_float = num_traits::cast::<usize, S>(width).unwrap();
        let height_float = num_traits::cast::<usize, S>(height).unwrap();
        self.aspect = width_float / height_float;
        self.matrix = Matrix4x4::from_perspective_fov(
            self.fovy, 
            self.aspect, 
            self.near, 
            self.far
        );
    }

    fn top_left_eye(&self) -> Vector3<Self::Scalar> {
        unimplemented!()
    }

    fn top_right_eye(&self) -> Vector3<Self::Scalar> {
        unimplemented!()
    }

    fn bottom_left_eye(&self) -> Vector3<Self::Scalar> {
        unimplemented!()
    }

    fn bottom_right_eye(&self) -> Vector3<Self::Scalar> {
        unimplemented!()
    }

    fn set_active(&mut self, projection: ActiveProjection) {

    }
}


/// A perspective projection based on arbitrary `left`, `right`, `bottom`,
/// `top`, `near`, and `far` planes.
///
/// We assume the following constraints to construct a useful perspective 
/// projection
/// ```text
/// left   < right
/// bottom < top
/// near   < far   (along the negative z-axis)
/// ```
/// Each parameter in the specification is a description of the position along
/// an axis of a plane that the axis is perpendicular to.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PerspectiveSpec<S> {
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

impl<S> PerspectiveSpec<S> {
    /// Construct a new perspective specification.
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

impl<S> fmt::Display for PerspectiveSpec<S> 
where 
    S: fmt::Display
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "PerspectiveSpec [left={}, right={}, bottom={}, top={}, near={}, far={}]",
            self.left, self.right, self.bottom, self.top, self.near, self.far
        )
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

impl<S> CameraModel for PerspectiveProjection<S>
where 
    S: SimdScalarFloat
{
    type Scalar = S;
    type Spec = PerspectiveSpec<S>;
    type Projection = Matrix4x4<S>;

    #[inline]
    fn from_spec(spec: &Self::Spec) -> Self {
        let matrix = Matrix4x4::from_perspective(
            spec.left, 
            spec.right, 
            spec.bottom, 
            spec.top,
            spec.near,
            spec.far
        );

        Self {
            left: spec.left,
            right: spec.right,
            bottom: spec.bottom,
            top: spec.top,
            near: spec.near,
            far: spec.far,
            matrix: matrix,
        }
    }

    #[inline]
    fn projection(&self) -> &Self::Projection {
        &self.matrix
    }

    fn update_viewport(&mut self, width: usize, height: usize) {
        /*
        let width_float = num_traits::cast::<usize, S>(width).unwrap();
        let height_float = num_traits::cast::<usize, S>(height).unwrap();
        self.matrix = Matrix4x4::from_perspective(
            self.left,
            self.right,
            self.bottom,
            self.top,
            self.near,
            self.far
        );
        */
        unimplemented!()
    }

    fn top_left_eye(&self) -> Vector3<Self::Scalar> {
        Vector3::new(self.left, self.top, -self.near)
    }

    fn top_right_eye(&self) -> Vector3<Self::Scalar> {
        Vector3::new(self.right, self.top, -self.near)
    }

    fn bottom_left_eye(&self) -> Vector3<Self::Scalar> {
        Vector3::new(self.left, self.bottom, -self.near)
    }

    fn bottom_right_eye(&self) -> Vector3<Self::Scalar> {
        Vector3::new(self.right, self.bottom, -self.near)
    }

    fn set_active(&mut self, projection: ActiveProjection) {

    }
}


/// A description of an orthographic projection with arbitrary `left`, `right`, 
/// `top`, `bottom`, `near`, and `far` planes.
///
/// We assume the following constraints to construct a useful orthographic 
/// projection
/// ```text
/// left   < right
/// bottom < top
/// near   < far   (along the negative z-axis).
/// ```
/// Each parameter in the specification is a description of the position along 
/// an axis of a plane that the axis is perpendicular to.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrthographicSpec<S> {
    /// The horizontal position of the left-hand plane in camera space.
    /// The left-hand plane is a plane parallel to the **yz-plane** at
    /// the origin.
    left: S,
    /// The horizontal position of the right-hand plane in camera space.
    /// The right-hand plane is a plane parallel to the **yz-plane** at
    /// the origin.
    right: S,
    /// The vertical position of the **bottom plane** in camera space.
    /// The bottom plane is a plane parallel to the **xz-plane** at the origin.
    bottom: S,
    /// The vertical position of the **top plane** in camera space.
    /// the top plane is a plane parallel to the **xz-plane** at the origin.
    top: S,
    /// The distance along the **negative z-axis** of the **near plane** from the eye.
    /// The near plane is a plane parallel to the **xy-plane** at the origin.
    near: S,
    /// the distance along the **negative z-axis** of the **far plane** from the eye.
    /// The far plane is a plane parallel to the **xy-plane** at the origin.
    far: S,
}

impl<S> OrthographicSpec<S> {
    /// Construct a new orthographic specification.
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

impl<S> fmt::Display for OrthographicSpec<S>
where 
    S: fmt::Display
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "OrthographicSpec [left={}, right={}, bottom={}, top={}, near={}, far={}]",
            self.left, self.right, self.bottom, self.top, self.near, self.far
        )
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
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OrthographicProjection<S> {
    /// The horizontal position of the left-hand plane in camera space.
    /// The left-hand plane is a plane parallel to the **yz-plane** at
    /// the origin.
    left: S,
    /// The horizontal position of the right-hand plane in camera space.
    /// The right-hand plane is a plane parallel to the **yz-plane** at
    /// the origin.
    right: S,
    /// The vertical position of the **bottom plane** in camera space.
    /// The bottom plane is a plane parallel to the **xz-plane** at the origin.
    bottom: S,
    /// The vertical position of the **top plane** in camera space.
    /// the top plane is a plane parallel to the **xz-plane** at the origin.
    top: S,
    /// The distance along the **negative z-axis** of the **near plane** from the eye.
    /// The near plane is a plane parallel to the **xy-plane** at the origin.
    near: S,
    /// the distance along the **negative z-axis** of the **far plane** from the eye.
    /// The far plane is a plane parallel to the **xy-plane** at the origin.
    far: S,
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
            "Orthographicrojection [{}]",
            self.matrix
        )
    }
}

impl<S> CameraModel for OrthographicProjection<S> 
where 
    S: SimdScalarFloat
{
    type Scalar = S;
    type Spec = OrthographicSpec<S>;
    type Projection = Matrix4x4<S>;

    #[inline]
    fn from_spec(spec: &Self::Spec) -> Self {
        let matrix = Matrix4x4::from_orthographic(
            spec.left, 
            spec.right, 
            spec.bottom, 
            spec.top,
            spec.near,
            spec.far
        );

        Self {
            left: spec.left,
            right: spec.right,
            bottom: spec.bottom,
            top: spec.top,
            near: spec.near,
            far: spec.far,
            matrix: matrix,
        }
    }

    #[inline]
    fn projection(&self) -> &Self::Projection {
        &self.matrix
    }

    fn update_viewport(&mut self, _width: usize, _height: usize) {
        /*
        let width_float = num_traits::cast::<usize, S>(width).unwrap();
        let height_float = num_traits::cast::<usize, S>(height).unwrap();
        self.matrix = Matrix4x4::from_orthographic(
            self.left,
            self.right,
            self.bottom,
            self.top,
            self.near,
            self.far
        );
        */
        unimplemented!()
    }

    fn top_left_eye(&self) -> Vector3<Self::Scalar> {
        Vector3::new(self.left, self.top, -self.near)
    }

    fn top_right_eye(&self) -> Vector3<Self::Scalar> {
        Vector3::new(self.right, self.top, -self.near)
    }

    fn bottom_left_eye(&self) -> Vector3<Self::Scalar> {
        Vector3::new(self.left, self.bottom, -self.near)
    }

    fn bottom_right_eye(&self) -> Vector3<Self::Scalar> {
        Vector3::new(self.right, self.bottom, -self.near)
    }

    fn set_active(&mut self, projection: ActiveProjection) {

    }
}


/// An orthographic projection based on the `near` plane, the `far` plane and 
/// the vertical field of view angle `fovy` and the horizontal/vertical aspect 
/// ratio `aspect`.
///
/// We assume the following constraints to make a useful orthographic projection 
/// camera model.
/// ```text
/// 0 radians < fovy < pi radians
/// aspect > 0
/// near < far (along the negative z-axis)
/// ```
/// This orthographic projection model imposes some constraints on the more 
/// general orthographic specification based on the arbitrary planes. The `fovy` 
/// parameter combined with the aspect ratio `aspect` ensures that the top and 
/// bottom planes are the same distance from the eye position along the vertical 
/// axis on opposite side. They ensure that the `left` and `right` planes are 
/// equidistant from the eye on opposite sides along the horizontal axis. 
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrthographicFovSpec<S> {
    /// The vertical field of view angle of the orthographic camera model 
    /// viewport.
    fovy: Degrees<S>,
    /// The ratio of the horizontal width to the vertical height.
    aspect: S,
    /// The position of the near plane along the **negative z-axis**.
    near: S,
    /// The position of the far plane along the **negative z-axis**.
    far: S,
}

impl<S> OrthographicFovSpec<S> {
    /// Construct a new orthographic projection operation specification
    /// based on the vertical field of view angle `fovy`, the `near` plane, the 
    /// `far` plane, and aspect ratio `aspect`.
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

impl<S> fmt::Display for OrthographicFovSpec<S> 
where 
    S: fmt::Display
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "OrthographicFovSpec [fovy={}, aspect={}, near={}, far={}]",
            self.fovy, self.aspect, self.near, self.far
        )
    }
}


/// An orthographic projection camera model for converting from camera space to
/// normalized device coordinates.
///
/// Orthographic projections differ from perspective projections in that 
/// orthographic projections keeps parallel lines parallel, whereas perspective 
/// projections preserve the perception of distance. Perspective 
/// projections preserve the spatial ordering in the distance that points are 
/// located from the viewing plane.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct OrthographicFovProjection<S> {
    /// The vertical field of view angle of the orthographic camera model 
    /// viewport.
    fovy: Degrees<S>,
    /// The ratio of the horizontal width to the vertical height.
    aspect: S,
    /// The position of the near plane along the **negative z-axis**.
    near: S,
    /// The position of the far plane along the **negative z-axis**.
    far: S,
    /// The underlying matrix that implements the orthographic projection.
    matrix: Matrix4x4<S>,
}

impl<S> OrthographicFovProjection<S> 
where 
    S: SimdScalarFloat 
{
    /// Get the underlying matrix implementing the orthographic camera model.
    #[inline]
    pub fn to_matrix(&self) -> &Matrix4x4<S> {
        &self.matrix
    }
}

impl<S> fmt::Display for OrthographicFovProjection<S> 
where 
    S: fmt::Display 
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "OrthographicFovProjection [{}]",
            self.matrix
        )
    }
}

impl<S> CameraModel for OrthographicFovProjection<S> 
where 
    S: SimdScalarFloat
{
    type Scalar = S;
    type Spec = OrthographicFovSpec<S>;
    type Projection = Matrix4x4<S>;

    #[inline]
    fn from_spec(spec: &Self::Spec) -> Self {
        let matrix = Matrix4x4::from_orthographic_fov(
            spec.fovy, 
            spec.aspect, 
            spec.near,
            spec.far
        );

        Self {
            fovy: spec.fovy,
            aspect: spec.aspect,
            near: spec.near,
            far: spec.far,
            matrix: matrix,
        }
    }

    #[inline]
    fn projection(&self) -> &Self::Projection {
        &self.matrix
    }

    fn update_viewport(&mut self, width: usize, height: usize) {
        let width_float = num_traits::cast::<usize, S>(width).unwrap();
        let height_float = num_traits::cast::<usize, S>(height).unwrap();
        self.aspect = width_float / height_float;
        self.matrix = Matrix4x4::from_orthographic_fov(
            self.fovy, 
            self.aspect, 
            self.near, 
            self.far
        );
    }

    fn top_left_eye(&self) -> Vector3<Self::Scalar> {
        unimplemented!()
    }

    fn top_right_eye(&self) -> Vector3<Self::Scalar> {
        unimplemented!()
    }

    fn bottom_left_eye(&self) -> Vector3<Self::Scalar> {
        unimplemented!()
    }

    fn bottom_right_eye(&self) -> Vector3<Self::Scalar> {
        unimplemented!()
    }

    fn set_active(&mut self, projection: ActiveProjection) {

    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SwitchableFovSpec<S> {
    /// The vertical field of view angle of the orthographic camera model 
    /// viewport.
    fovy: Degrees<S>,
    /// The ratio of the horizontal width to the vertical height.
    aspect: S,
    /// The position of the near plane along the **negative z-axis**.
    near: S,
    /// The position of the far plane along the **negative z-axis**.
    far: S,
    /// The default active projection at the time of construction.
    default_active_projection: ActiveProjection,
}

impl<S> SwitchableFovSpec<S> {
    /// Construct a new switchable projection operation specification
    /// based on the vertical field of view angle `fovy`, the `near` plane, the 
    /// `far` plane, and aspect ratio `aspect`.
    #[inline]
    pub const fn new(
        fovy: Degrees<S>, 
        aspect: S, 
        near: S, 
        far: S, 
        default_active_projection: ActiveProjection) -> Self 
    {
        Self {
            fovy: fovy,
            aspect: aspect,
            near: near,
            far: far,
            default_active_projection: default_active_projection,
        }
    }
}

impl<S> fmt::Display for SwitchableFovSpec<S> 
where 
    S: fmt::Display
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "SwitchableFovSpec [fovy={}, aspect={}, near={}, far={}]",
            self.fovy, self.aspect, self.near, self.far
        )
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwitchableFovProjection<S> {
    /// The vertical field of view angle of the camera model viewport.
    fovy: Degrees<S>,
    /// The ratio of the horizontal width to the vertical height.
    aspect: S,
    /// The position of the near plane along the **negative z-axis**.
    near: S,
    /// The position of the far plane along the **negative z-axis**.
    far: S,
    /// The underlying matrix that implements the orthographic projection.
    orthographic_matrix: Matrix4x4<S>,
    /// The underlying matrix that implements the perspective projection.
    perspective_matrix: Matrix4x4<S>,
    /// The current active projection.
    active_projection: ActiveProjection,
}


impl<S> SwitchableFovProjection<S> 
where 
    S: SimdScalarFloat 
{
    /// Get the underlying matrix implementing the orthographic camera model.
    #[inline]
    pub fn to_matrix(&self) -> &Matrix4x4<S> {
        if self.active_projection.is_perspective() {
            &self.perspective_matrix
        } else {
            &self.orthographic_matrix
        }
    }
}

impl<S> fmt::Display for SwitchableFovProjection<S> 
where 
    S: fmt::Display 
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "SwitchableFovProjection [orthographic_matrix={}, perspective_matrix={}]",
            self.orthographic_matrix, self.perspective_matrix
        )
    }
}

impl<S> CameraModel for SwitchableFovProjection<S> 
where 
    S: SimdScalarFloat
{
    type Scalar = S;
    type Spec = SwitchableFovSpec<S>;
    type Projection = Matrix4x4<S>;

    #[inline]
    fn from_spec(spec: &Self::Spec) -> Self {
        let orthographic_matrix = Matrix4x4::from_orthographic_fov(
            spec.fovy, 
            spec.aspect, 
            spec.near,
            spec.far
        );
        let perspective_matrix = Matrix4x4::from_perspective_fov(
            spec.fovy, 
            spec.aspect, 
            spec.near,
            spec.far
        );

        Self {
            fovy: spec.fovy,
            aspect: spec.aspect,
            near: spec.near,
            far: spec.far,
            orthographic_matrix: orthographic_matrix,
            perspective_matrix: perspective_matrix,
            active_projection: spec.default_active_projection,
        }
    }

    #[inline]
    fn projection(&self) -> &Self::Projection {
        &self.to_matrix()
    }

    fn update_viewport(&mut self, width: usize, height: usize) {
        let width_float = num_traits::cast::<usize, S>(width).unwrap();
        let height_float = num_traits::cast::<usize, S>(height).unwrap();
        self.aspect = width_float / height_float;
        self.orthographic_matrix = Matrix4x4::from_orthographic_fov(
            self.fovy, 
            self.aspect, 
            self.near, 
            self.far
        );
        self.perspective_matrix = Matrix4x4::from_perspective_fov(
            self.fovy, 
            self.aspect, 
            self.near, 
            self.far
        );
    }

    fn top_left_eye(&self) -> Vector3<Self::Scalar> {
        unimplemented!()
    }

    fn top_right_eye(&self) -> Vector3<Self::Scalar> {
        unimplemented!()
    }

    fn bottom_left_eye(&self) -> Vector3<Self::Scalar> {
        unimplemented!()
    }

    fn bottom_right_eye(&self) -> Vector3<Self::Scalar> {
        unimplemented!()
    }

    fn set_active(&mut self, projection: ActiveProjection) {
        self.active_projection = projection
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SwitchableSpec<S> {
    /// The horizontal position of the left-hand plane in camera space.
    /// The left-hand plane is a plane parallel to the **yz-plane** at
    /// the origin.
    left: S,
    /// The horizontal position of the right-hand plane in camera space.
    /// The right-hand plane is a plane parallel to the **yz-plane** at
    /// the origin.
    right: S,
    /// The vertical position of the **bottom plane** in camera space.
    /// The bottom plane is a plane parallel to the **xz-plane** at the origin.
    bottom: S,
    /// The vertical position of the **top plane** in camera space.
    /// the top plane is a plane parallel to the **xz-plane** at the origin.
    top: S,
    /// The distance along the **negative z-axis** of the **near plane** from the eye.
    /// The near plane is a plane parallel to the **xy-plane** at the origin.
    near: S,
    /// the distance along the **negative z-axis** of the **far plane** from the eye.
    /// The far plane is a plane parallel to the **xy-plane** at the origin.
    far: S,
    /// The default active projection at the time of construction.
    default_active_projection: ActiveProjection,
}

impl<S> SwitchableSpec<S> {
    #[inline]
    pub const fn new(
        left: S,
        right: S,
        bottom: S,
        top: S,
        near: S, 
        far: S, 
        default_active_projection: ActiveProjection) -> Self 
    {
        Self {
            left: left,
            right: right,
            bottom: bottom,
            top: top,
            near: near,
            far: far,
            default_active_projection: default_active_projection,
        }
    }
}

impl<S> fmt::Display for SwitchableSpec<S> 
where 
    S: fmt::Display
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "SwitchableSpec [left={}, right={}, bottom={}, top={}, near={}, far={}]",
            self.left, self.right, self.bottom, self.top, self.near, self.far
        )
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwitchableProjection<S> {
    left: S,
    right: S,
    bottom: S,
    top: S,
    near: S,
    far: S,
    orthographic_matrix: Matrix4x4<S>,
    perspective_matrix: Matrix4x4<S>,
    /// The current active projection.
    active_projection: ActiveProjection,
}


impl<S> SwitchableProjection<S> 
where 
    S: SimdScalarFloat 
{
    /// Get the underlying matrix implementing the orthographic camera model.
    #[inline]
    pub fn to_matrix(&self) -> &Matrix4x4<S> {
        if self.active_projection.is_perspective() {
            &self.perspective_matrix
        } else {
            &self.orthographic_matrix
        }
    }
}

impl<S> fmt::Display for SwitchableProjection<S> 
where 
    S: fmt::Display 
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "SwitchableProjection [orthographic_matrix={}, perspective_matrix={}]",
            self.orthographic_matrix, self.perspective_matrix
        )
    }
}

impl<S> CameraModel for SwitchableProjection<S> 
where 
    S: SimdScalarFloat
{
    type Scalar = S;
    type Spec = SwitchableSpec<S>;
    type Projection = Matrix4x4<S>;

    #[inline]
    fn from_spec(spec: &Self::Spec) -> Self {
        let orthographic_matrix = Matrix4x4::from_orthographic(
            spec.left, 
            spec.right, 
            spec.bottom,
            spec.top,
            spec.near,
            spec.far
        );
        let perspective_matrix = Matrix4x4::from_perspective(
            spec.left, 
            spec.right, 
            spec.bottom,
            spec.top,
            spec.near,
            spec.far
        );

        Self {
            left: spec.left,
            right: spec.right,
            bottom: spec.bottom,
            top: spec.top,
            near: spec.near,
            far: spec.far,
            orthographic_matrix: orthographic_matrix,
            perspective_matrix: perspective_matrix,
            active_projection: spec.default_active_projection,
        }
    }

    #[inline]
    fn projection(&self) -> &Self::Projection {
        &self.to_matrix()
    }

    fn update_viewport(&mut self, width: usize, height: usize) {
        /*
        let width_float = num_traits::cast::<usize, S>(width).unwrap();
        let height_float = num_traits::cast::<usize, S>(height).unwrap();
        */
        unimplemented!()
    }

    fn top_left_eye(&self) -> Vector3<Self::Scalar> {
        Vector3::new(self.left, self.top, -self.near)
    }

    fn top_right_eye(&self) -> Vector3<Self::Scalar> {
        Vector3::new(self.right, self.top, -self.near)
    }

    fn bottom_left_eye(&self) -> Vector3<Self::Scalar> {
        Vector3::new(self.left, self.bottom, -self.near)
    }

    fn bottom_right_eye(&self) -> Vector3<Self::Scalar> {
        Vector3::new(self.right, self.bottom, -self.near)
    }

    fn set_active(&mut self, projection: ActiveProjection) {
        self.active_projection = projection
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

impl<S> fmt::Display for CameraAttitudeSpec<S> 
where 
    S: fmt::Display
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "CameraAttitudeSpec [position={}, forward={}, right={} up={}, axis={}]",
            self.position, self.forward, self.right, self.up, self.axis
        )
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
            &(spec.position)
        );
        let rotation_matrix = Matrix4x4::new(
             spec.right.x,    spec.right.y,    spec.right.z,   S::zero(),
             spec.up.x,       spec.up.y,       spec.up.z,      S::zero(),
            -spec.forward.x, -spec.forward.y, -spec.forward.z, S::zero(),
             S::zero(),       S::zero(),       S::zero(),      S::one()
        );
        let view_matrix = translation_matrix * rotation_matrix;
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
pub struct Camera<S, M> {
    /// The camera's model for mapping light rays to normalized device
    /// coordinates.
    model: M,
    /// The position and orientation of the camera in world space.
    attitude: CameraAttitude<S>,
}

impl<S, M> Camera<S, M> 
where 
    S: SimdScalarFloat,
    M: CameraModel<Scalar = S>,
{
    /// Construct a new camera.
    pub fn new(model_spec: &M::Spec, attitude_spec: &CameraAttitudeSpec<S>) -> Self {
        Self {
            model: <M as CameraModel>::from_spec(model_spec),
            attitude: CameraAttitude::from_spec(attitude_spec),
        }
    }

    /// Update the camera model based on changes to the viewport's dimensions.
    pub fn update_viewport(&mut self, width: usize, height: usize) {
        self.model.update_viewport(width, height);
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
    pub fn projection(&self) -> &M::Projection {
        self.model.projection()
    }

    pub fn top_left_eye(&self) -> Vector3<S> {
        self.model.top_left_eye()
    }

    pub fn top_right_eye(&self) -> Vector3<S> {
        self.model.top_right_eye()
    }

    pub fn bottom_left_eye(&self) -> Vector3<S> {
        self.model.bottom_left_eye()
    }

    pub fn bottom_right_eye(&self) -> Vector3<S> {
        self.model.bottom_right_eye()
    }

    pub fn set_active(&mut self, projection: ActiveProjection) {
        self.model.set_active(projection);
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
mod attitude_tests {
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
            0_f64,  0_f64,  5_f64, 1_f64
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
            0_f64,  0_f64,  5_f64, 1_f64
        ).inverse().unwrap();
        let result = attitude.view_matrix_inv();

        assert_eq!(result, &expected);
    }
}

