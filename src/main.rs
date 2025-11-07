mod raymod;
use raymod::*;

use rayon::prelude::*;

#[allow(unused)]
use std::io::Write;

fn ray_color(r: &Ray, world: &dyn Shape, depth: i64, background: Vec3) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    let hit_info = world.hit(&r, EPS, f64::MAX);
    if let Some(hit) = hit_info {
        let emitted = hit.m.emitted(&r, &hit);
        let scatter_info = hit.m.scatter(r, &hit);
        if let Some(scatter) = scatter_info {
            emitted
                + scatter
                    .albedo
                    .mult(ray_color(&scatter.ray, world, depth - 1, background))
        } else {
            return emitted;
        }
    } else {
        return background;
    }
}

fn main() {
    let args = parameters();
    println!("{:?}", args);
    println!("sampling(use subpixel)={:?}",args.s*4);

    let mut w: usize = args.w;
    let mut h: usize = ((w as f64) / WIDE_ASPECT) as usize;
    let samps: usize = args.s;


    let MAX_DEPTH: i64 = 32;

    let mut world = ShapeList::new();
    let mut background =Vec3::new(0.7,0.8,1.0);
    //オリジナルはRayの関数だがとりあえず定数で
    let cam: Camera;
    match args.m {
        0 => {//デフォルトはゼロ
            w = args.w;
            h = ((w as f64) / SQUARE_ASPECT) as usize;
     	    background=Vec3::new(0.0,0.0,0.0);
            cam = world.cornellbox_scene();
        }
        1 => {
            cam = world.simple_scene();
	    }
        2 => {
            cam = world.random_scene();
        }
        3 => {
            cam = world.texture_scene();
        }
        4 => {
     	    background=Vec3::new(0.1,0.1,0.1);
            cam = world.emitte_scene();
        }
        5 =>{
     	    background=Vec3::new(0.1,0.1,0.1);
            cam = world.emitte_squre_scene();
        }
        6 => {
            w = args.w;
            h = ((w as f64) / SQUARE_ASPECT) as usize;
            cam = world.cornellbox_scene();
            background=Vec3::zero();
        }
        7 =>{
            w = args.w;
            h = ((w as f64) / SQUARE_ASPECT) as usize;
            cam = world.cornell_scene();
            background=Vec3::zero();
        }
        _ => {
            cam = world.simple_scene();
        }
    }

    let mut image = vec![Color::zero(); (w * h) as usize];
    let bands: Vec<(usize, &mut [Color])> = image.chunks_mut(w as usize).enumerate().collect();
    bands.into_par_iter().for_each(|(y, band)| {
        for x in 0..w {
            let mut r = Vec3::new(0.0, 0.0, 0.0);
            for _spp in 0..samps {
                for _sy in 0..2 {
                    for _sx in 0..2 {
                        let u = (x as f64 + (_sx as f64 + random()) / 4.0) / (w as f64);
                        let v = (y as f64 + (_sy as f64 + random()) / 4.0) / (h as f64);
                        let ray = cam.get_ray(u, v);
                        r = r + ray_color(&ray, &world, MAX_DEPTH, background)
                            / (samps as f64)
                            / 4.0;
                    }
                }
            }
            band[x as usize] = r;
        }
        if (y % 20) == 0 {
            print!("y={0}  :", y);
            println!("col={:?}", band[0]);
        };
    });

    //    save_ppm_file("image.ppm", image, w, h);
    save_png_file(&args.output, image, w, h);
}
