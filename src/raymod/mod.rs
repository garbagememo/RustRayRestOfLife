
mod bvh;
mod material;
mod optarg;
mod rayunit;
mod scene;
mod vec3;
mod quat;

pub use self::bvh::*;
pub use self::material::*;
pub use self::optarg::*;
pub use self::rayunit::*;
pub use self::scene::*;
pub use self::vec3::*;
pub use self::quat::*;

pub const EPS: f64 = 1e-6;
pub const EPS10: f64 = 1e-4;
pub const FRAC_SQRT_3: f64 = 1.732050807568877293527446341505872367;
pub const WIDE_ASPECT: f64 = 16.0 / 9.0;
pub const SQUARE_ASPECT:f64 =1.0;
