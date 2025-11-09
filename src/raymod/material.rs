use crate::raymod::*;
pub use std::f64::consts::*;
use std::sync::Arc;


pub trait Pdf:Send + Sync {
    fn value(&self, hit: &HitInfo, direction: Vec3) -> f64;
    fn generate(&self, hit: &HitInfo) -> Vec3;
}

pub struct CosinePdf {}

impl CosinePdf {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Pdf for CosinePdf {
    fn value(&self, hit: &HitInfo, direction: Vec3) -> f64 {
        let cosine = direction.norm().dot(&hit.n);
        if cosine > 0.0 {
            cosine * FRAC_1_PI
        } else {
            0.0
        }
    }
    fn generate(&self, hit: &HitInfo) -> Vec3 {
        ONB::new(hit.n).local(Vec3::random_cosine_direction())
    }
}

pub struct MixturePdf {
    pub pdfs: [Arc<dyn Pdf>; 2],
}
impl MixturePdf {
    pub fn new(pdf0: Arc<dyn Pdf>, pdf1: Arc<dyn Pdf>) -> Self {
        Self { pdfs: [pdf0, pdf1] }
    }
}
impl Pdf for MixturePdf {
    fn value(&self, hit: &HitInfo, direction: Vec3) -> f64 {
        let pdf0 = self.pdfs[0].value(&hit, direction);
        let pdf1 = self.pdfs[1].value(&hit, direction);
        0.5 * pdf0 + 0.5 * pdf1
    }
    fn generate(&self, hit: &HitInfo) -> Vec3 {
        if random() < 0.5 {
            self.pdfs[0].generate(hit)
        } else {
            self.pdfs[1].generate(hit)
        }
    }
}

pub struct ShapePdf {
    pub shape: Arc<dyn Shape>,
    pub origin: Vec3,
}
impl ShapePdf {
    pub fn new(shape: Arc<dyn Shape>, origin: Vec3) -> Self {
        Self { shape, origin }
    }
}
impl Pdf for ShapePdf {
    fn value(&self, _hit: &HitInfo, direction: Vec3) -> f64 {
        self.shape.pdf_value(self.origin, direction)
    }
    fn generate(&self, _hit: &HitInfo) -> Vec3 {
        self.shape.random(self.origin)
    }
}

pub struct ScatterInfo {
    pub ray: Ray,
    pub albedo: Color,
    pub pdf:Option<Arc<dyn Pdf>>,
}

impl ScatterInfo {
    pub fn new(ray: Ray, albedo: Vec3,pdf:Option<Arc<dyn Pdf>>) -> Self {
        Self { ray, albedo, pdf }
    }
}

#[allow(unused)]
pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo>;
    fn emitted(&self, ray: &Ray, hit: &HitInfo) -> Color {Color::zero()}
    fn scattering_pdf(&self, _ray: &Ray, _hit: &HitInfo) -> f64 { 0.0 }
}

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color;
}

pub struct ColorTexture {
    color: Vec3,
}
impl ColorTexture {
    pub const fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for ColorTexture {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
    freq: f64,
}
impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>, freq: f64) -> Self {
        Self { odd, even, freq }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color {
        let sines = (p.x * self.freq).sin() * (p.y * self.freq).sin() * (p.z * self.freq).sin();
        //*ではなく+にすると斑点模様になる
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    pixels: Vec<Vec3>,
    width: usize,
    height: usize,
}

impl ImageTexture {
    pub fn new(path: &str) -> Self {
        let rgbimg = image::open(path).unwrap().to_rgb8();
        let (w, h) = rgbimg.dimensions();
        let mut image = vec![Vec3::zero(); (w * h) as usize];
        for (i, (_, _, pixel)) in image.iter_mut().zip(rgbimg.enumerate_pixels()) {
            *i = Color::from_rgb(pixel[0], pixel[1], pixel[2]);
        }
        Self {
            pixels: image,
            width: w as usize,
            height: h as usize,
        }
    }

    pub fn sample(&self, u: i64, v: i64) -> Color {
        let tu = if u < 0 {
            0
        } else if u as usize >= self.width {
            self.width - 1
        } else {
            u as usize
        };
        let tv = if v < 0 {
            0
        } else if v as usize >= self.height {
            self.height - 1
        } else {
            v as usize
        };
        self.pixels[tu + self.width * tv]
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Vec3) -> Color {
        let x = (u * self.width as f64) as i64;
        let y = ((1.0 - v) * self.height as f64) as i64;
        self.sample(x, y)
    }
}

pub struct DiffuseLight {
    pub emit: Box<dyn Texture>,
}
impl DiffuseLight {
    pub fn new(emit: Box<dyn Texture>) -> Self {
        Self { emit }
    }
}

#[allow(unused)]
impl Material for DiffuseLight {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        None
    }
    fn emitted(&self, ray: &Ray, hit: &HitInfo) -> Color {
        if ray.d.dot(&hit.n) < 0.0 {
            self.emit.value(hit.u, hit.v, hit.p)
        } else {
            Vec3::zero()
        }
    }
}


pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
    pub pdf:Arc<dyn Pdf>,
}
impl Lambertian {
    pub fn new(albedo: Box<dyn Texture>) -> Self {
        Self { albedo,pdf:Arc::new(CosinePdf::new()) }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let albedo = self.albedo.value(hit.u, hit.v, hit.p);
        Some(ScatterInfo::new(*ray, albedo,Some(Arc::clone(&self.pdf))))
    }
    fn scattering_pdf(&self, ray: &Ray, hit: &HitInfo) -> f64 {
        ray.d.norm().dot(&hit.n).max(0.0) * FRAC_1_PI
    }
}

pub struct Metal {
    pub albedo: Box<dyn Texture>,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Box<dyn Texture>, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter(&self,ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let mut reflected = ray.d.norm().reflect(hit.n);
        reflected = reflected + self.fuzz*Vec3::random_hemisphere() ;
        if reflected.dot(&hit.n) > 0.0 {
            let albedo = self.albedo.value(hit.u, hit.v, hit.p);
            Some(ScatterInfo::new(Ray::new(hit.p, reflected), albedo,None))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ri: f64,
}

impl Dielectric {
    pub const fn new(ri: f64) -> Self {
        Self { ri }
    }
    pub fn schlick(cosine: f64, ri: f64) -> f64 {
        let r0 = ((1.0 - ri) / (1.0 + ri)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let reflected = ray.d.reflect(hit.n);
        let (outward_normal, ni_over_nt, cosine) = {
            let dot = ray.d.dot(&hit.n);
            if dot > 0.0 {
                (-hit.n , self.ri, self.ri * dot / ray.d.length().sqrt())
            } else {
                (
                    hit.n,
                    1.0 / self.ri,
                    -self.ri * dot / ray.d.length().sqrt() ,
                )
            }
        };
        if let Some(refracted) = (-ray.d ).refract(outward_normal, ni_over_nt) {
            if Vec3::random_full().x > Self::schlick(cosine, self.ri) {
                return Some(ScatterInfo::new(
                    Ray::new(hit.p, refracted),
                    Vec3::new(1.0, 1.0, 1.0),
                    None,
                ));
            }
        }
        Some(ScatterInfo::new(
            Ray::new(hit.p, reflected),
            Vec3::new(1.0, 1.0, 1.0),
            None,
        ))
    }
}
