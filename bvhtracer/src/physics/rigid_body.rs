use cglinalg::{
    SimdScalarFloat,
    Matrix3x3,
    Matrix4x4,
    Vector3,
    Quaternion,
    Magnitude,
};


// Internal function that checks the validity of an inverse inertia tensor.
#[inline]
fn _check_inverse_inertia_tensor<S: SimdScalarFloat>(iit_world: &Matrix3x3<S>) {
    assert!(iit_world.is_finite());
}

// Inline function that creates a transform matrix from a
// position and orientation.
#[inline]
fn _calculate_transform_matrix<S: SimdScalarFloat>(
    transform: &mut Matrix4x4<S>, 
    position: &Vector3<S>, 
    orientation: &Quaternion<S>,
) {
    let zero = S::zero();
    let one = S::one();
    let two = one + one;
    let qs = orientation.s;
    let qx = orientation.v.x;
    let qy = orientation.v.y;
    let qz = orientation.v.z;

    transform[0][0] = one - two * orientation.v.y * orientation.v.y - two * orientation.v.z * orientation.v.z;
    transform[0][1] = two * orientation.v.x * orientation.v.y + two * orientation.s * orientation.v.z;
    transform[0][2] = two * orientation.v.x * orientation.v.z - two * orientation.s * orientation.v.y;
    transform[0][3] = zero;
    
    transform[1][0] = two * orientation.v.x * orientation.v.y - two * orientation.s * orientation.v.z;
    transform[1][1] = one - two * orientation.v.x * orientation.v.x - two * orientation.v.z * orientation.v.z;
    transform[1][2] = two * orientation.v.y * orientation.v.z + two * orientation.s * orientation.v.x;
    transform[1][3] = zero;

    transform[2][0] = two * orientation.v.x * orientation.v.z + two * orientation.s * orientation.v.y;
    transform[2][1] = two * orientation.v.y * orientation.v.z - two * orientation.s * orientation.v.x;
    transform[2][2] = one - two * orientation.v.x * orientation.v.x - two * orientation.v.y * orientation.v.y;
    transform[2][3] = zero;

    transform[3][0] = position.x;
    transform[3][1] = position.y;
    transform[3][2] = position.z;
    transform[3][3] = one;
}

// Internal function to do an inertia tensor transform by a quaternion.
// Note that the implementation of this function was created by an
// automated code-generator and optimizer.
#[inline]
fn _transform_inertia_tensor<S: SimdScalarFloat>(
    iit_world: &mut Matrix3x3<S>, 
    q: &Quaternion<S>, 
    iit_body: &Matrix3x3<S>, 
    rot_mat: &Matrix4x4<S>
) {
    let m00 = rot_mat[0][0] * iit_body[0][0] + rot_mat[1][0] * iit_body[0][1] + rot_mat[2][0] * iit_body[0][2]; // + rot_mat[3][0] * iit_body[0][3];
    let m10 = rot_mat[0][0] * iit_body[1][0] + rot_mat[1][0] * iit_body[1][1] + rot_mat[2][0] * iit_body[1][2]; // + rot_mat[3][0] * iit_body[1][3];
    let m20 = rot_mat[0][0] * iit_body[2][0] + rot_mat[1][0] * iit_body[2][1] + rot_mat[2][0] * iit_body[2][2]; // + rot_mat[3][0] * iit_body[2][3];
    // let m30 = rot_mat[0][0] * iit_body[3][0] + rot_mat[1][0] * iit_body[3][1] + rot_mat[2][0] * iit_body[3][2]; // + rot_mat[3][0] * iit_body[3][3];
    let m01 = rot_mat[0][1] * iit_body[0][0] + rot_mat[1][1] * iit_body[0][1] + rot_mat[2][1] * iit_body[0][2]; // + rot_mat[3][1] * iit_body[0][3];
    let m11 = rot_mat[0][1] * iit_body[1][0] + rot_mat[1][1] * iit_body[1][1] + rot_mat[2][1] * iit_body[1][2]; // + rot_mat[3][1] * iit_body[1][3];
    let m21 = rot_mat[0][1] * iit_body[2][0] + rot_mat[1][1] * iit_body[2][1] + rot_mat[2][1] * iit_body[2][2]; // + rot_mat[3][1] * iit_body[2][3];
    // let m31 = rot_mat[0][1] * iit_body[3][0] + rot_mat[1][1] * iit_body[3][1] + rot_mat[2][1] * iit_body[3][2]; // + rot_mat[3][1] * iit_body[3][3];
    let m02 = rot_mat[0][2] * iit_body[0][0] + rot_mat[1][2] * iit_body[0][1] + rot_mat[2][2] * iit_body[0][2]; // + rot_mat[3][2] * iit_body[0][3];
    let m12 = rot_mat[0][2] * iit_body[1][0] + rot_mat[1][2] * iit_body[1][1] + rot_mat[2][2] * iit_body[1][2]; // + rot_mat[3][2] * iit_body[1][3];
    let m22 = rot_mat[0][2] * iit_body[2][0] + rot_mat[1][2] * iit_body[2][1] + rot_mat[2][2] * iit_body[2][2]; // + rot_mat[3][2] * iit_body[2][3];
    // let m32 = rot_mat[0][2] * iit_body[3][0] + rot_mat[1][2] * iit_body[3][1] + rot_mat[2][2] * iit_body[3][2]; // + rot_mat[3][2] * iit_body[3][3];
    // let m03 = rot_mat[0][3] * iit_body[0][0] + rot_mat[1][3] * iit_body[0][1] + rot_mat[2][3] * iit_body[0][2]; // + rot_mat[3][3] * iit_body[0][3];
    // let m13 = rot_mat[0][3] * iit_body[1][0] + rot_mat[1][3] * iit_body[1][1] + rot_mat[2][3] * iit_body[1][2]; // + rot_mat[3][3] * iit_body[1][3];
    // let m23 = rot_mat[0][3] * iit_body[2][0] + rot_mat[1][3] * iit_body[2][1] + rot_mat[2][3] * iit_body[2][2]; // + rot_mat[3][3] * iit_body[2][3];
    // let m33 = rot_mat[0][3] * iit_body[3][0] + rot_mat[1][3] * iit_body[3][1] + rot_mat[2][3] * iit_body[3][2]; // + rot_mat[3][3] * iit_body[3][3];

    iit_world[0][0] = m00 * rot_mat[0][0] + m10 * rot_mat[1][0] + m20 * rot_mat[2][0]; // + m30 * rot_mat[3][0];
    iit_world[1][0] = m00 * rot_mat[0][1] + m10 * rot_mat[1][1] + m20 * rot_mat[2][1]; // + m30 * rot_mat[3][1];
    iit_world[2][0] = m00 * rot_mat[0][2] + m10 * rot_mat[1][2] + m20 * rot_mat[2][2]; // + m30 * rot_mat[3][2];
    // iit_world[3][0] = m00 * rot_mat[0][3] + m10 * rot_mat[1][3] + m20 * rot_mat[2][3] + m30 * rot_mat[3][3];
    iit_world[0][1] = m01 * rot_mat[0][0] + m11 * rot_mat[1][0] + m21 * rot_mat[2][0]; // + m31 * rot_mat[3][0];
    iit_world[1][1] = m01 * rot_mat[0][1] + m11 * rot_mat[1][1] + m21 * rot_mat[2][1]; // + m31 * rot_mat[3][1];
    iit_world[2][1] = m01 * rot_mat[0][2] + m11 * rot_mat[1][2] + m21 * rot_mat[2][2]; // + m31 * rot_mat[3][2];
    // iit_world[3][1] = m01 * rot_mat[0][3] + m11 * rot_mat[1][3] + m21 * rot_mat[2][3] + m31 * rot_mat[3][3];
    iit_world[0][2] = m02 * rot_mat[0][0] + m12 * rot_mat[1][0] + m22 * rot_mat[2][0]; // + m32 * rot_mat[3][0];
    iit_world[1][2] = m02 * rot_mat[0][1] + m12 * rot_mat[1][1] + m22 * rot_mat[2][1]; // + m32 * rot_mat[3][1];
    iit_world[2][2] = m02 * rot_mat[0][2] + m12 * rot_mat[1][2] + m22 * rot_mat[2][2]; // + m32 * rot_mat[3][2];
    // iit_world[3][2] = m02 * rot_mat[0][3] + m12 * rot_mat[1][3] + m22 * rot_mat[2][3] + m32 * rot_mat[3][3];
    // iit_world[0][3] = m03 * rot_mat[0][0] + m13 * rot_mat[1][0] + m23 * rot_mat[2][0] + m33 * rot_mat[3][0];
    // iit_world[1][3] = m03 * rot_mat[0][1] + m13 * rot_mat[1][1] + m23 * rot_mat[2][1] + m33 * rot_mat[3][1];
    // iit_world[2][3] = m03 * rot_mat[0][2] + m13 * rot_mat[1][2] + m23 * rot_mat[2][2] + m33 * rot_mat[3][2];
    // iit_world[3][3] = m03 * rot_mat[0][3] + m13 * rot_mat[1][3] + m23 * rot_mat[2][3] + m33 * rot_mat[3][3];
}

pub struct RigidBody<S> {
    inverse_mass: S,
    inverse_inertia_tensor: Matrix3x3<S>,
    linear_damping: S,
    angular_damping: S,
    position: Vector3<S>,
    orientation: Quaternion<S>,
    velocity: Vector3<S>,
    rotation: Vector3<S>,
    inverse_inertia_tensor_world: Matrix3x3<S>,
    motion: S,
    is_awake: bool,
    can_sleep: bool,
    /// Body space to world space.
    transform: Matrix4x4<S>,
    force_accumulator: Vector3<S>,
    torque_accumulator: Vector3<S>,
    acceleration: Vector3<S>,
    last_frame_acceleration: Vector3<S>,
}

impl<S> RigidBody<S>
where
    S: SimdScalarFloat,
{
    #[inline(always)]
    fn sleep_epsilon(&self) -> S {
        num_traits::cast(0.3).unwrap()
    }

    pub (crate) fn calculate_derived_data(&mut self) {
        self.orientation.normalize();

        // Calculate the transform matrix for the body.
        _calculate_transform_matrix(&mut self.transform, &self.position, &self.orientation);

        // Calculate the inertia tensor in world space.
        _transform_inertia_tensor(
            &mut self.inverse_inertia_tensor_world,
            &self.orientation, 
            &self.inverse_inertia_tensor,
            &self.transform
        );
    }

    /// Newton-Euler method.
    pub fn integrate(&mut self, duration: S) {
        if !self.is_awake {
            return;
        }

        // Calculate linear acceleration from force inputs.
        self.last_frame_acceleration = self.acceleration;
        self.last_frame_acceleration += self.force_accumulator * self.inverse_mass;

        // Calculate angular acceleration from torque inputs.
        let angular_acceleration = self.inverse_inertia_tensor_world * self.torque_accumulator;

        // Adjust velocities
        // Update linear velocity from both acceleration and impulse.
        self.velocity += self.last_frame_acceleration * duration;

        // Update angular velocity from both acceleration and impulse.
        self.rotation += angular_acceleration * duration;

        // Impose drag.
        self.velocity *= S::powf(self.linear_damping, duration);
        self.rotation *= S::powf(self.angular_damping, duration);

        // Adjust positions
        // Update linear position.
        self.position += self.velocity * duration;

        // Update angular position.
        let new_orientation = {
            let one_half = num_traits::cast(0.5_f64).unwrap();
            let mut _new_orientation = self.orientation.clone();
            let mut q = Quaternion::from_pure(self.rotation * duration);
            q = q * _new_orientation;
            _new_orientation.s   += q.s * one_half;
            _new_orientation.v.x += q.v.x * one_half;
            _new_orientation.v.y += q.v.y * one_half;
            _new_orientation.v.z += q.v.z * one_half;
            _new_orientation
        };
        self.orientation = new_orientation;

        // Normalise the orientation, and update the matrices with the new
        // position and orientation
        self.calculate_derived_data();

        // Clear accumulators.
        self.clear_accumulators();

        // Update the kinetic energy store, and possibly put the body to
        // sleep.
        if self.can_sleep {
            let current_motion = self.velocity.dot(&self.velocity) + self.rotation.dot(&self.rotation);
            let one_half = num_traits::cast(0.5_f64).unwrap();
            let bias = S::powf(one_half, duration);
            self.motion = bias * self.motion + (S::one() - bias) * current_motion;
            let ten: S = num_traits::cast(10_f64).unwrap();
            if self.motion < self.sleep_epsilon() {
                self.set_awake(false);
            } else if self.motion > ten * self.sleep_epsilon() {  
                self.motion = ten * self.sleep_epsilon();
            }
        }
    }

    fn set_mass(&mut self, mass: S) {
        assert!(mass != S::zero());
        self.inverse_mass = S::one() / mass;
    }

    pub fn get_mass(&self) -> S {
        if self.inverse_mass.is_zero() {
            S::max_value()
        } else {
            S::one() / self.inverse_mass
        }
    }

    fn set_inverse_mass(&mut self, inverse_mass: S) {
        self.inverse_mass = inverse_mass;
    }

    fn get_inverse_mass(&self) -> S {
        self.inverse_mass
    }

    pub fn has_finite_mass(&self) -> bool {
        self.inverse_mass >= S::zero()
    }

    fn set_inertia_tensor(&mut self, inertia_tensor: &Matrix3x3<S>) {
        self.inverse_inertia_tensor = inertia_tensor.inverse().unwrap();
        _check_inverse_inertia_tensor(&self.inverse_inertia_tensor);
    }

    fn get_inertia_tensor_mut(&self, output: &mut Matrix3x3<S>) {
        *output = self.inverse_inertia_tensor.inverse().unwrap();
    }

    fn get_inertia_tensor(&self) -> Matrix3x3<S> {
        let mut output = Matrix3x3::zero();
        self.get_inertia_tensor_mut(&mut output);
        
        output
    }
       
    fn get_inertia_tensor_world_mut(&self, output: &mut Matrix3x3<S>) {
        *output = self.inverse_inertia_tensor_world.inverse().unwrap();
    }

    fn get_inertia_tensor_world(&self) -> Matrix3x3<S> {
        let mut output = Matrix3x3::zero();
        self.get_inertia_tensor_world_mut(&mut output);

        output
    }

    fn set_inverse_inertia_tensor(&mut self, inverse_inertia_tensor: &Matrix3x3<S>) {
        _check_inverse_inertia_tensor(inverse_inertia_tensor);
        self.inverse_inertia_tensor = inverse_inertia_tensor.clone();
    }

    fn get_inverse_inertia_tensor(&self) -> &Matrix3x3<S> {
        &self.inverse_inertia_tensor
    }

    fn get_inverse_inertia_tensor_world(&self) -> &Matrix3x3<S> {
        &self.inverse_inertia_tensor_world
    }

    fn set_damping(&mut self, linear_damping: S, angular_damping: S) {
        self.linear_damping = linear_damping;
        self.angular_damping = angular_damping;
    }

    fn set_linear_damping(&mut self, linear_damping: S) {
        self.linear_damping = linear_damping;
    }

    fn get_linear_damping(&self) -> S {
        self.linear_damping
    }

    fn set_angular_damping(&mut self, angular_damping: S) {
        self.angular_damping = angular_damping;
    }

    pub fn get_angular_damping(&self) -> S {
        self.angular_damping
    }

    fn set_position(&mut self, position: &Vector3<S>) {
        self.position = *position;
    }

    pub fn get_position(&self) -> &Vector3<S> {
        &self.position
    }

    fn set_orientation(&mut self, orientation: &Quaternion<S>) {
        self.orientation = *orientation;
        self.orientation.normalize();
    }

    fn get_orientation_mut(&self, orientation: &mut Matrix3x3<S>) {
        orientation[0][0] = self.transform[0][0];
        orientation[0][1] = self.transform[0][1];
        orientation[0][2] = self.transform[0][2];
        
        orientation[1][0] = self.transform[1][0];
        orientation[1][1] = self.transform[1][1];
        orientation[1][2] = self.transform[1][2];

        orientation[2][0] = self.transform[2][0];
        orientation[2][1] = self.transform[2][1];
        orientation[2][2] = self.transform[2][2];
    }

    fn get_orientation(&self) -> &Quaternion<S> {
        &self.orientation
    }

    pub fn get_transform(&self) -> &Matrix4x4<S> {
        &self.transform
    }

    // world space to body space
    pub fn get_point_in_local_space(&self, point_world: &Vector3<S>) -> Vector3<S> {
        let transform_inverse = self.transform.inverse().unwrap();
        (transform_inverse * point_world.extend(S::one())).contract()
    }

    // body space to world space.
    pub fn get_point_in_world_space(&self, point_body: &Vector3<S>) -> Vector3<S> {
        (self.transform * point_body.extend(S::one())).contract()
    }
    
    // world space to body space.
    pub fn get_direction_in_local_space(&self, direction_world: &Vector3<S>) -> Vector3<S> {
        let transform_inverse = self.transform.inverse().unwrap();
        (transform_inverse * direction_world.extend(S::zero())).contract()
    }

    // body space to world space.
    pub fn get_direction_in_world_space(&self, direction_body: &Vector3<S>) -> Vector3<S> {
        (self.transform * direction_body.extend(S::zero())).contract()
    }

    fn set_velocity(&mut self, velocity: &Vector3<S>) {
        self.velocity = *velocity;
    }

    pub fn get_velocity(&self) -> &Vector3<S> {
        &self.velocity
    }

    pub fn add_velocity(&mut self, delta_velocity: &Vector3<S>) {
        self.velocity += delta_velocity;
    }

    fn set_rotation(&mut self, rotation: &Vector3<S>) {
        self.rotation = *rotation;
    }

    pub fn get_rotation(&self) -> &Vector3<S> {
        &self.rotation
    }

    pub fn add_rotation(&mut self, delta_rotation: &Vector3<S>) {
        self.rotation += delta_rotation;
    }

    pub fn get_awake(&self) -> bool {
        self.is_awake
    }

    fn set_awake(&mut self, awake: bool) {
        if awake {
            self.is_awake = true;
            // Add a bit of motion to avoid it falling asleep immediately.
            let two = num_traits::cast(2_f64).unwrap();
            self.motion = self.sleep_epsilon() * two;
        } else {
            self.is_awake = false;
            self.velocity = Vector3::zero();
            self.rotation = Vector3::zero();
        }
    }

    pub fn get_can_sleep(&self) -> bool {
        self.can_sleep
    }

    fn set_can_sleep(&mut self, can_sleep: bool) {
        self.can_sleep = can_sleep;
        if !self.can_sleep && !self.is_awake {
            self.set_awake(true);
        }
    }

    fn get_last_frame_acceleration(&self) -> &Vector3<S> {        
        &self.last_frame_acceleration
    }

    pub fn clear_accumulators(&mut self) {
        self.force_accumulator = Vector3::zero();
        self.torque_accumulator = Vector3::zero();
    }

    // Apply force to center of mass.
    pub fn apply_force(&mut self, force: &Vector3<S>) {
        self.force_accumulator += force;
        self.is_awake = true;
    }

    // The force is not applied to the center of mass, so it may split into a force and a torque.
    pub fn apply_force_at_point(&mut self, force_world: &Vector3<S>, point_world: &Vector3<S>) {
        let point_cm = point_world - self.position;
        self.force_accumulator += force_world;
        self.torque_accumulator += point_cm.cross(&force_world);
        self.is_awake = true;
    }

    // The force is not applied to the center of mass, so it may split into a force and a torque.
    pub fn apply_force_at_body_point(&mut self, force_world: &Vector3<S>, point_body: &Vector3<S>) {
        // Convert to coordinates relative to center of mass.
        let point_cm = self.get_point_in_world_space(point_body);
        self.apply_force_at_point(force_world, &point_cm);
    }

    pub fn apply_torque(&mut self, torque: &Vector3<S>) {
        self.torque_accumulator += torque;
        self.is_awake = true;
    }

    fn set_acceleration(&mut self, new_acceleration: &Vector3<S>) {
        self.acceleration = *new_acceleration;
    }

    pub fn get_acceleration_world(&self) -> Vector3<S> {
        self.acceleration
    }
}

