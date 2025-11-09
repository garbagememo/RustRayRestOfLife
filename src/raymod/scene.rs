use crate::raymod::*;
use std::sync::Arc;

//左上が原点なPNGフォーマット対応
#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub upper_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f64,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,//このアスペクト比16:9は固定
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).norm();
        let u = (vup % w).norm();
        let v = w % u;

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let upper_left_corner = origin - horizontal / 2.0 + vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            upper_left_corner,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = rd.x*self.u + rd.y*self.v ;
        Ray::new(
            self.origin + offset,
            self.upper_left_corner + s * self.horizontal - t * self.vertical - self.origin - offset,
        )
    }
}

#[allow(dead_code)]
impl ShapeList {
    pub fn cornell_mirror_box_scene(&mut self) -> Camera {
        let red = Color::new(0.64, 0.05, 0.05);
        let white = Color::new(0.73,0.73,0.73);
        let green = Color::new(0.12, 0.45, 0.15);

        //light
        self.push(Box::new(
            FlipFace::new(Box::new(
                Rect::new(
                    213.0, 343.0, 227.0, 332.0, 554.0,RectAxisType::XZ,
                    Arc::new(
                        DiffuseLight::new(Box::new(ColorTexture::new(
                            Vec3::new(15.0, 15.0, 15.0,))))
                    ),
                )
            ))
        ));
        
        self.push(Box::new(
            FlipFace::new(Box::new(
                Rect::new(
                    0.0, 555.0, 0.0, 555.0, 555.0,RectAxisType::YZ,
                    Arc::new(
                        Lambertian::new(Box::new(ColorTexture::new(green)))
                    )
                )
            ))
        ));
        
        self.push(Box::new(
            Rect::new(
                0.0, 555.0, 0.0, 555.0, 0.0,RectAxisType::YZ,
                Arc::new(
                    Lambertian::new(Box::new(ColorTexture::new(red)))
                )
            )
        ));
        
        self.push(Box::new(
            FlipFace::new(Box::new(
                Rect::new(
                    0.0, 555.0, 0.0, 555.0, 555.0,RectAxisType::XZ,
                    Arc::new(
                        Lambertian::new(Box::new(ColorTexture::new(white)))
                    )
                )
            ))
        ));
        self.push(Box::new(
            Rect::new(
                0.0, 555.0, 0.0, 555.0, 0.0,RectAxisType::XZ,
                Arc::new(
                    Lambertian::new(Box::new(ColorTexture::new(white)))
                )
            )
        ));
        self.push(Box::new(
            FlipFace::new(Box::new(
                Rect::new(
                    0.0, 555.0, 0.0, 555.0, 555.0,RectAxisType::XY,
                    Arc::new(
                        Lambertian::new(Box::new(ColorTexture::new(white)))
                    )
                )
            ))
        ));

        let mut box_list1: Vec<Box<dyn Shape>> = Vec::new();
      
        box_list1.push(Box::new(
            Translate::new(Box::new(
                Rotate::new(Box::new(
                    RectAngle::new(
                        Vec3::new(0.0, 0.0, 0.0),Vec3::new(165.0, 165.0, 165.0),
                        Arc::new(
                            Lambertian::new(Box::new(ColorTexture::new(white)))
                        )
                        ))
                    ,Vec3::new(0.0,1.0,0.0),-18.0)
                )
                ,Vec3::new(130.0, 0.0, 65.0)
            )    
        ));
        
        box_list1.push(Box::new(
            Translate::new(Box::new(
                Rotate::new(Box::new(
                    RectAngle::new(
                        Vec3::new(0.0, 0.0, 0.0),Vec3::new(165.0, 330.0, 165.0),
                        Arc::new(
                            Metal::new(Box::new(ColorTexture::new(white)),0.0)
                        )
                    )),
                    Vec3::new(0.0,1.0,0.0),15.0)
            ),Vec3::new(265.0, 0.0, 295.0) )    
        ));
        self.push(Box::new(BVH::new(box_list1)));

        
        // simple_scene用カメラ
        let lookfrom = Vec3::new(278.0, 278.0, -800.0);
        let lookat = Vec3::new(278.0, 278.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);

        let dist_to_focus = (lookfrom - lookat).length().sqrt();
        let aperture = 0.1;

        return Camera::new(
            lookfrom,
            lookat,
            vup,
            40.0,
            SQUARE_ASPECT,
            aperture,
            dist_to_focus,
        );
    }

}
pub struct CornellBoxScene {
    pub cam:Camera,
    pub world: ShapeList,
    pub light:Arc<dyn Shape>,
}

impl CornellBoxScene {
    pub fn new() -> Self{
        let mut world = ShapeList::new();
        let cam = world.cornell_mirror_box_scene();
        let light = Arc::new(Rect::new(
                            213.0, 343.0, 227.0, 332.0, 554.0,RectAxisType::XZ,
                            Arc::new(Lambertian::new(Box::new( ColorTexture::new(Vec3::zero()) )))
                        ));
        Self { cam,world,light }
    }
    pub fn ray_color(&self,r: &Ray,depth: i64, background: Vec3) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        let black=Box::new(ColorTexture::new(Vec3::zero()));
        let hit_info = self.world.hit(&r, EPS, f64::MAX);
        if let Some(hit) = hit_info {
            let emitted = hit.m.emitted(&r, &hit);
            let scatter_info = hit.m.scatter(r, &hit);
            if let Some(scatter) = scatter_info {
                if let Some(pdf)=scatter.pdf {
                    let shape_pdf = Arc::new(ShapePdf::new(Arc::clone(&self.light), hit.p));
                    let pdf = MixturePdf::new(shape_pdf, Arc::clone(&pdf) );
                    let new_ray = Ray::new(hit.p,pdf.generate(&hit));
                    let spdf_value = pdf.value(&hit,new_ray.d);
                    if spdf_value > 0.0 {
                        let pdf_value = hit.m.scattering_pdf(&new_ray, &hit);
                        let albedo = scatter.albedo * pdf_value;
                        emitted
                            + albedo.mult(self.ray_color(&new_ray, depth-1, background)) /spdf_value

                    } else {
                        emitted
                    }
                }else{
                    emitted
                        + scatter.albedo.mult(self.ray_color(&scatter.ray, depth-1, background)) 
                }
            } else {
                emitted
            }
        } else { // hit=None
            return background;
        }
    }

}

