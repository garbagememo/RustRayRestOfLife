use std::fs;
use std::io::Write;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::ops::{Index, IndexMut};

pub fn random() -> f64 {
    rand::random::<f64>()
}

pub fn random_range(a: f64, b: f64) -> f64 {
    random() * (b - a) + a
}

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Color = Vec3;

#[allow(dead_code)]
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    pub fn xaxis() -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
    pub fn yaxis() -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }
    pub fn zaxis() -> Vec3 {
        Vec3::new(0.0, 0.0, 1.0)
    }

    pub fn mult(&self, b: Vec3) -> Vec3 {
        Vec3::new(self.x * b.x, self.y * b.y, self.z * b.z)
    }
    pub fn norm(mut self) -> Vec3 {
        let l = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x = self.x * l;
        self.y = self.y * l;
        self.z = self.z * l;
        self
    }
    pub fn dot(&self, b: &Vec3) -> f64 {
        return self.x * b.x + self.y * b.y + self.z * b.z;
    }
    pub fn length(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
    pub fn random() -> Vec3 {
        return Vec3::new(random(), random(), random());
    }
    pub fn random_full() -> Vec3 {
        let x = random();
        return Vec3::new(x, x, x);
    }
    pub fn vec3_random_range(a: f64, b: f64) -> Vec3 {
        return Vec3::new(random_range(a, b), random_range(a, b), random_range(a, b));
    }
    pub fn random_hemisphere() -> Vec3 {
        loop {
            let point = Vec3::vec3_random_range(-1.0, 1.0);
            if point.length() < 1.0 {
                return point;
            }
        }
    }
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
            if p.length() < 1.0 {
                return p;
            }
        }
    }
    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        *self - normal * 2.0 * self.dot(&normal)
    }
    pub fn refract(&self, normal: Self, ni_over_nt: f64) -> Option<Vec3> {
        let uv = self.norm();
        let dt = uv.dot(&normal);
        let d = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if d > 0.0 {
            Some((uv - normal * dt) * (-1.0 * ni_over_nt) - normal * d.sqrt())
        } else {
            None
        }
    }
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Rem for Vec3 {
    type Output = Vec3;
    fn rem(self, rhs: Self) -> Self {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

// ----------------------------------------------------
// 読み取りアクセス (Vec3[i]) を可能にする実装
// ----------------------------------------------------
impl Index<usize> for Vec3 {
    // 参照が返す型を指定
    type Output = f64;

    // インデックス i に対応するフィールドの参照を返す
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds: {}", i), // 範囲外のインデックスはパニック
        }
    }
}

// ----------------------------------------------------
// 書き込みアクセス (Vec3[i] = value) を可能にする実装
// ----------------------------------------------------
impl IndexMut<usize> for Vec3 {
    // インデックス i に対応するフィールドの可変参照を返す
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds: {}", i),
        }
    }
}

fn clamp(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    }
}

fn to_int(x: f64) -> u8 {
    (clamp(x).powf(1.0 / 2.2) * 255.0 + 0.5) as u8
}

#[allow(dead_code)]
fn save_ppm_file(filename: &str, image: Vec<Color>, width: usize, height: usize) {
    let mut f = fs::File::create(filename).unwrap();
    writeln!(f, "P3\n{} {}\n{}", width, height, 255).unwrap();
    for i in 0..(width * (height)) {
        write!(
            f,
            "{} {} {} ",
            to_int(image[i as usize].x),
            to_int(image[i as usize].y),
            to_int(image[i as usize].z)
        )
        .unwrap();
    }
}

pub fn save_png_file(filename: &str, out_image: Vec<Color>, width: usize, height: usize) {
    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let i: usize = (x as usize) + (y as usize) * width;
        let r = to_int(out_image[i].x);
        let g = to_int(out_image[i].y);
        let b = to_int(out_image[i].z);
        *pixel = image::Rgb([r, g, b]);
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(filename).unwrap();
}
