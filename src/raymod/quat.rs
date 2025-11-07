use crate::raymod::*;


/// A quaternion
pub struct Quat(Vec3, f64);

 #[allow(dead_code)]
impl Quat {
    /// Construct from x,y,z and w elements
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Quat(Vec3{x,y,z}, w)
    }

    /// Construct from a unit vector and radians
    pub fn from_rot(v: Vec3, rad: f64) -> Self {
        let (s, c) = (rad * 0.5).sin_cos();
        Quat(v * s, c)
    }

    /// Construct x-axis rotation
    pub fn from_rot_x(rad: f64) -> Self {
        let (s, c) = (rad * 0.5).sin_cos();
        Quat::new(s, 0.0, 0.0, c)
    }

    /// Construct y-axis rotation
    pub fn from_rot_y(rad: f64) -> Self {
        let (s, c) = (rad * 0.5).sin_cos();
        Quat::new(0.0, s, 0.0, c)
    }

    /// Construct z-axis rotation
    pub fn from_rot_z(rad: f64) -> Self {
        let (s, c) = (rad * 0.5).sin_cos();
        Quat::new(0.0, 0.0, s, c)
    }

    /// Construct a unit quaternion
    pub const fn unit() -> Self {
        Quat::new(0.0, 0.0, 0.0, 1.0)
    }

    /// Construct a zero quaternion
    pub const fn zero() -> Self {
        Quat::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Compute the conjugate of a quaternion
    pub fn conj(&self) -> Self {
        Quat(-self.0, self.1)
    }

    /// Compute the dot product of two quaternion
    pub fn dot(&self, rhs: Self) -> f64 {
        self.0.dot(&rhs.0) + self.1 * rhs.1
    }

    /// Compute the length of vector
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Compute the squared length of vector
    pub fn length_squared(&self) -> f64 {
        self.0.length() + self.1.powi(2)
    }

    /// Returns normalized this quaternion
    pub fn normalize(&self) -> Self {
        let recip = self.length().recip();
        Quat(self.0 * recip, self.1 * recip)
    }

    /// Returns as array
    pub fn to_array(&self) -> [f64; 4] {
        [self.0.x, self.0.y, self.0.z, self.1]
    }

    /// Rotate a 3D vector
    pub fn rotate(&self, p: Vec3) -> Vec3 {
        let [x1, y1, z1, w1] = self.to_array();
        let x2=p.x; let y2=p.y;let z2 = p.z;
        let x = (w1 * x2 + y1 * z2) - (z1 * y2);
        let y = (w1 * y2 + z1 * x2) - (x1 * z2);
        let z = (w1 * z2 + x1 * y2) - (y1 * x2);
        let w = (x1 * x2 + y1 * y2) - (z1 * z2);
        Vec3::new(
            ((w * x1 + x * w1) - y * z1) + z * y1,
            ((w * y1 + y * w1) - z * x1) + x * z1,
            ((w * z1 + z * w1) - x * y1) + y * x1,
        )
    }

    /// Compute linear interpolation between two quaternions
    pub fn lerp(&self, _other: Quat, _t: f64) -> Self {
        todo!()
    }

    /// Compute spherical linear interpolation between two quaternions
    pub fn slerp(&self, _other: Quat, _t: f64) -> Self {
        todo!()
    }
}

/// Multiply
impl std::ops::Mul<Quat> for Quat {
    type Output = Self;
    fn mul(self, rhs: Quat) -> Self {
        let [x1, y1, z1, w1] = self.to_array();
        let [x2, y2, z2, w2] = rhs.to_array();
        Quat::new(
            w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2,
            w1 * y2 + y2 * w2 + z1 * x2 - x1 * z2,
            w1 * z2 + z1 * w2 + x1 * y2 - y1 * x2,
            w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2,
        )
    }
}


pub struct Translate {
    pub shape: Box<dyn Shape>,
    pub offset: Vec3,
}
impl Translate {
    pub fn new(shape: Box<dyn Shape>, offset: Vec3) -> Self {
        Self { shape, offset }
    }
}
impl Shape for Translate {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let moved_ray = Ray::new(ray.o - self.offset, ray.d);
        if let Some(hit) = self.shape.hit(&moved_ray, t0, t1) {
            Some(HitInfo { p: hit.p + self.offset, ..hit })
        } else {
            None
        }
    }
    fn bounding_box(&self) -> Option<AABB> {
        if let Some(aabb)=self.shape.bounding_box() {
            let min = aabb.min +self.offset;
            let max = aabb.max +self.offset;
            Some(AABB{min,max})
        }else{
            None
        }
    }
}
pub struct Rotate {
    pub shape: Box<dyn Shape>,
    pub quat: Quat,
}
impl Rotate {
    pub fn new(shape: Box<dyn Shape>, axis: Vec3, angle: f64) -> Self {
        Self { shape, quat: Quat::from_rot(axis, angle.to_radians()) }
    }
}
impl Shape for Rotate {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let revq = self.quat.conj();
        let rotated_ray = Ray::new(revq.rotate(ray.o), revq.rotate(ray.d));
        if let Some(hit) = self.shape.hit(&rotated_ray, t0, t1) {
            Some(HitInfo { p: self.quat.rotate(hit.p), n: self.quat.rotate(hit.n), ..hit })
        } else {
            None
        }
    }
    fn bounding_box(&self) -> Option<AABB> {
        if let Some(aabb)=self.shape.bounding_box() {
            let mut min=Vec3::new( f64::INFINITY, f64::INFINITY, f64::INFINITY);
            let mut max=Vec3::new(-f64::INFINITY,-f64::INFINITY,-f64::INFINITY);
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2{
                        let x=i as f64 * aabb.max.x + (1-i)as f64 * aabb.min.x;
                        let y=j as f64 * aabb.max.y + (1-j)as f64 * aabb.min.y;
                        let z=k as f64 * aabb.max.z + (1-k)as f64 * aabb.min.z;
                        let tester = self.quat.rotate(Vec3{x,y,z});
                        min.x=min.x.min(tester.x);
                        max.x=max.x.max(tester.x);
                        min.y=min.y.min(tester.y);
                        max.y=max.y.max(tester.y);
                        min.z=min.z.min(tester.z);
                        max.z=max.z.max(tester.z);
                    }
                }
            }
            Some(AABB{min,max})
        }else{
            None
        }
    }
}
