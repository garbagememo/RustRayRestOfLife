use crate::raymod::*;

use std::f64::consts::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct Ray {
    pub o: Vec3,
    pub d: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Ray {
        Ray { o, d }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.o + self.d * t
    }
}

pub struct HitInfo {
    pub t: f64,
    pub p: Vec3,
    pub n: Vec3,
    pub m: Arc<dyn Material>,
    pub u: f64,
    pub v: f64,
}

impl HitInfo {
    pub fn new(t: f64, p: Vec3, n: Vec3, m: Arc<dyn Material>, u: f64, v: f64) -> Self {
        Self { t, p, n, m, u, v }
    }
}



pub trait Shape: Sync {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo>;
    fn bounding_box(&self) -> Option<AABB>;
}

//法線逆転用
pub struct FlipFace {
    pub shape: Box<dyn Shape>,
}
impl FlipFace {
    pub fn new(shape: Box<dyn Shape>) -> Self {
        Self { shape }
    }
}
impl Shape for FlipFace {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        if let Some(hit) = self.shape.hit(ray, t0, t1) {
            Some(HitInfo { n: -hit.n, ..hit })
        } else {
            None
        }
    }
    fn bounding_box(&self) -> Option<AABB> {
        self.shape.bounding_box() 
    }
}


pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub const fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
    fn uv(p: Vec3) -> (f64, f64) {
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        (1.0 - (phi + PI) / (2.0 * PI), (theta + PI / 2.0) / PI)
    }
}

impl Shape for Sphere {
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let oc = r.o - self.center;
        let a = r.d.dot(&r.d);
        let b = r.d.dot(&oc) * 2.0;
        let c = oc.dot(&oc) - self.radius * self.radius;
        let d = b * b - 4.0 * a * c;

        if d > 0.0 {
            let root = d.sqrt();
            let temp = (-b - root) / (2.0 * a);
            if temp < t1 && temp > t0 {
                let p = r.at(temp);
                let n = (p - self.center) / self.radius;
                let (u, v) = Self::uv(n);
                return Some(HitInfo::new(temp, p, n, Arc::clone(&self.material), u, v));
            }
            let temp = (-b + root) / (2.0 * a);
            if temp < t1 && temp > t0 {
                let p = r.at(temp);
                let n = (p - self.center) / self.radius;
                let (u, v) = Self::uv(n);
                return Some(HitInfo::new(temp, p, n, Arc::clone(&self.material), u, v));
            }
        }
        None
    }
    fn bounding_box(&self) -> Option<AABB> {
        let radius = Vec3::new(self.radius, self.radius, self.radius);
        let min = self.center - radius;
        let max = self.center + radius;
        Some(AABB { min, max })
    }
}

pub enum RectAxisType {
    XY,
    XZ,
    YZ,
}
pub struct Rect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub axis: RectAxisType,
    pub material: Arc<dyn Material>,
}

impl Rect {
    pub fn new(_x0: f64,_x1: f64,_y0: f64,_y1: f64, k: f64,
               axis: RectAxisType,material: Arc<dyn Material>,
    ) -> Self {
    let x0 = _x0.min(_x1);let x1 = _x1.max(_x0);
    let y0 = _y0.min(_y1);let y1 = _y1.max(_y0);
        Self {x0,x1,y0, y1,k,axis, material,}
    }
}

impl Shape for Rect {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let mut origin = ray.o;
        let mut direction = ray.d;
        let mut axis = Vec3::zaxis();
        match self.axis {
            RectAxisType::XY => {}
            RectAxisType::XZ => {
                origin = Vec3::new(origin.x, origin.z, origin.y);
                direction = Vec3::new(direction.x, direction.z, direction.y);
                axis = Vec3::yaxis();
            }
            RectAxisType::YZ => {
                origin = Vec3::new(origin.y, origin.z, origin.x);
                direction = Vec3::new(direction.y, direction.z, direction.x);
                axis = Vec3::xaxis();
            }
        }
        let t = (self.k - origin.z) / direction.z;
        if t < t0 || t > t1 {
            return None;
        }
        let x = origin.x + t * direction.x;
        let y = origin.y + t * direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        Some(HitInfo::new(
            t,
            ray.at(t),
            axis,
            Arc::clone(&self.material),
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        let min:Vec3;
        let max:Vec3;
        match self.axis {
            RectAxisType::XY => {
                min = Vec3::new(self.x0, self.y0, self.k - EPS10);
                max = Vec3::new(self.x1, self.y1, self.k + EPS10);
            }
            RectAxisType::XZ =>{
                min = Vec3::new(self.x0, self.k - EPS10, self.y0);
                max = Vec3::new(self.x1, self.k + EPS10, self.y1);
            }
            RectAxisType::YZ => {
                min = Vec3::new(self.k - EPS10, self.x0, self.y0);
                max = Vec3::new(self.k + EPS10, self.x1, self.y1);
            }
        }
        Some(AABB { min, max })
    }
}

pub struct RectAngle {
    p_min:Vec3,
    p_max:Vec3,
    shapes:ShapeList,
}

impl RectAngle {
    pub fn new(a: Vec3, b: Vec3, material: Arc<dyn Material>) -> Self{
        let p_min=Vec3::new(a.x.min(b.x),a.y.min(b.y),a.z.min(b.z));
        let p_max=Vec3::new(a.x.max(b.x),a.y.max(b.y),a.z.max(b.z));
        let mut shapes = ShapeList::new();
        shapes.push(Box::new(Rect::new(p_min.x,p_max.x,p_min.y,p_max.y,p_max.z,RectAxisType::XY,Arc::clone(&material)) ));
        shapes.push(Box::new(
            FlipFace::new(Box::new(
                Rect::new(p_min.x,p_max.x,p_min.y,p_max.y,p_min.z,RectAxisType::XY,Arc::clone(&material))
            ))
        ));
        shapes.push(Box::new(Rect::new(p_min.x,p_max.x,p_min.z,p_max.z,p_max.y,RectAxisType::XZ,Arc::clone(&material)) ));
        shapes.push(Box::new(
            FlipFace::new(Box::new(
                Rect::new(p_min.x,p_max.x,p_min.z,p_max.z,p_min.y,RectAxisType::XZ,Arc::clone(&material))
            ))
        ));
        shapes.push(Box::new(Rect::new(p_min.y,p_max.y,p_min.z,p_max.z,p_max.x,RectAxisType::YZ,Arc::clone(&material)) ));
        shapes.push(Box::new(
            FlipFace::new(Box::new(
                Rect::new(p_min.y,p_max.y,p_min.z,p_max.z,p_min.x,RectAxisType::YZ,Arc::clone(&material))
            ))
        ));
        Self { p_min,p_max,shapes}
    }
}

impl Shape for RectAngle {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        self.shapes.hit(ray, t0, t1)
    }
    fn bounding_box(&self) -> Option<AABB> {
        let min=self.p_min;
        let max=self.p_max;
        Some(AABB { min, max })
    }
}

pub struct ShapeList {
    pub objects: Vec<Box<dyn Shape>>,
}

impl ShapeList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn push(&mut self, object: Box<dyn Shape>) {
        self.objects.push(object);
    }
}

impl Shape for ShapeList {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let mut hit_info: Option<HitInfo> = None;
        let mut closest_so_far = t1;
        for object in &self.objects {
            if let Some(info) = object.hit(ray, t0, closest_so_far) {
                closest_so_far = info.t;
                hit_info = Some(info);
            }
        }
        hit_info
    }
    fn bounding_box(&self) -> Option<AABB> {
        match &self.objects.first() {
            Some(first) => {
                match first.bounding_box() {
                    Some(bbox) => self.objects.iter().skip(1).try_fold(bbox, |acc, shape| {
                        match shape.bounding_box() {
                            Some(bbox) => Some(surrounding_box(&acc, &bbox)),
                            _ => None,
                        }
                    }),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}
