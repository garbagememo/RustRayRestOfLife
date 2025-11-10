mod raymod;
use raymod::*;
use std::sync::Arc;

use rayon::prelude::*;

#[allow(unused)]
use std::io::Write;


fn main() {
    let args = parameters();
    println!("{:?}", args);
    println!("sampling(use subpixel)={:?}",args.s*4);

    let samps: usize = args.s;
    let MAX_DEPTH: i64 = 32;

    let mut scene: Box<dyn Scene>;
    
    let mut w: usize = args.w;
    let mut h: usize = ((w as f64) / WIDE_ASPECT) as usize;

   match args.m {
        0 => {//デフォルトはゼロ
            //---cornellbox
            h=((w as f64)/SQUARE_ASPECT) as usize;
            scene =Box::new( CornellBoxScene::new() );
        }
        1 => {
            //----Random_scene
            h=((w as f64)/WIDE_ASPECT) as usize;
            scene = Box::new( RandomScene::new() );
	    }
        _ => {
            //---cornellbox
            h=((w as f64)/SQUARE_ASPECT) as usize;
            scene =Box::new( CornellBoxScene::new() );
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
                        let ray = scene.get_ray(u, v);
                        r = r + scene.ray_color(&ray,MAX_DEPTH,)
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
