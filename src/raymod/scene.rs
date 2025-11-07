use crate::raymod::*;
use std::sync::Arc;

//左上が原点なPNGフォーマット対応
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
        aspect_ratio: f64,
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
    pub fn simple_scene(&mut self) -> Camera {
        self.push(Box::new(Sphere::new(
            Vec3::new(0.6, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::new(Box::new(ColorTexture::new(Vec3::new(
                0.1, 0.2, 0.5,
            ))))),
        )));
        self.push(Box::new(Sphere::new(
            Vec3::new(-0.6, 0.0, -1.0),
            0.5,
            Arc::new(Metal::new(
                Box::new(ColorTexture::new(Vec3::new(0.8, 0.8, 0.8))),
                0.4,
            )),
        )));
        self.push(Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, 0.0),
            100.0,
            Arc::new(Lambertian::new(Box::new(CheckerTexture::new(
                Box::new(ColorTexture::new(Vec3::new(0.8, 0.8, 0.0))),
                Box::new(ColorTexture::new(Vec3::new(0.8, 0.2, 0.0))),
                10.0,
            )))),
        )));
        // simple_scene用カメラ
        let lookfrom = Vec3::new(0.0, 1.0, 4.0);
        let lookat = Vec3::new(0.0, 0.0, -1.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);

        let dist_to_focus = (lookfrom - lookat).length().sqrt();
        let aperture = 0.1;

        return Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            WIDE_ASPECT,
            aperture,
            dist_to_focus,
        );
    }

    pub fn texture_scene(&mut self) -> Camera {
        self.push(Box::new(Sphere::new(
            Vec3::new(0.6, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::new(Box::new(ImageTexture::new(
                "testimage.jpg",
            )))),
        )));
        self.push(Box::new(Sphere::new(
            Vec3::new(-0.6, 0.0, -1.0),
            0.5,
            Arc::new(Metal::new(
                Box::new(ColorTexture::new(Vec3::new(0.8, 0.8, 0.8))),
                0.4,
            )),
        )));
        self.push(Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, 0.0),
            100.0,
            Arc::new(Lambertian::new(Box::new(CheckerTexture::new(
                Box::new(ColorTexture::new(Vec3::new(0.8, 0.8, 0.0))),
                Box::new(ColorTexture::new(Vec3::new(0.8, 0.2, 0.0))),
                10.0,
            )))),
        )));
        // simple_scene用カメラ
        let lookfrom = Vec3::new(0.0, 1.0, 4.0);
        let lookat = Vec3::new(0.0, 0.0, -1.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);

        let dist_to_focus = (lookfrom - lookat).length().sqrt();
        let aperture = 0.1;

        return Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            WIDE_ASPECT,
            aperture,
            dist_to_focus,
        );
    }
    pub fn emitte_scene(&mut self) -> Camera {
        self.push(Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(DiffuseLight::new(Box::new(ColorTexture::new(Vec3::new(
                1.0, 1.0, 1.0,
            ))))),
        )));
        self.push(Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, 0.0),
            100.0,
            Arc::new(Lambertian::new(Box::new(CheckerTexture::new(
                Box::new(ColorTexture::new(Vec3::new(0.8, 0.8, 0.0))),
                Box::new(ColorTexture::new(Vec3::new(0.8, 0.2, 0.0))),
                10.0,
            )))),
        )));
        // simple_scene用カメラ
        let lookfrom = Vec3::new(0.0, 1.0, 4.0);
        let lookat = Vec3::new(0.0, 0.0, -1.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);

        let dist_to_focus = (lookfrom - lookat).length().sqrt();
        let aperture = 0.1;

        return Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            WIDE_ASPECT,
            aperture,
            dist_to_focus,
        );
    }
    pub fn emitte_squre_scene(&mut self) -> Camera {
        self.push(Box::new(Rect::new(
            0.0,2.0,0.0,-2.0,-2.0,RectAxisType::YZ,
            Arc::new(
                DiffuseLight::new(Box::new(ColorTexture::new(
                    Vec3::new(1.0, 1.0, 1.0,))))
            ),
        )));
        self.push(Box::new(Sphere::new(
            Vec3::new(0.0, 0.5, -1.0),
            0.5,
            Arc::new(
                Lambertian::new(Box::new(ColorTexture::new(
                    Vec3::new(0.5,0.5,0.5))))
            )
        )));
        self.push(Box::new(Sphere::new(
            Vec3::new(0.0, -100.1, 0.0),
            100.0,
            Arc::new(Lambertian::new(
                Box::new(ColorTexture::new(Vec3::new(0.8, 0.8, 0.8)))
            ))
        )));
        // simple_scene用カメラ
        let lookfrom = Vec3::new(0.0, 2.0, 13.0);
        let lookat = Vec3::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);

        let dist_to_focus = (lookfrom - lookat).length().sqrt();
        let aperture = 0.1;

        return Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            WIDE_ASPECT,
            aperture,
            dist_to_focus,
        );
    }

    pub fn cornellbox_scene(&mut self) -> Camera {
        let red = Color::new(0.64, 0.05, 0.05);
        let white = Color::new(0.73,0.73,0.73);
        let green = Color::new(0.12, 0.45, 0.15);

        //light
        self.push(Box::new(
            Rect::new(
                    213.0, 343.0, 227.0, 332.0, 554.0,RectAxisType::XZ,
                    Arc::new(
                            DiffuseLight::new(Box::new(ColorTexture::new(
                                Vec3::new(15.0, 15.0, 15.0,))))
                    ),
            )
        ));
        
        self.push(Box::new(
            Rect::new(
                0.0, 555.0, 0.0, 555.0, 555.0,RectAxisType::YZ,
                Arc::new(
                    Lambertian::new(Box::new(ColorTexture::new(green)))
                )
                )
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
            Rect::new(
                0.0, 555.0, 0.0, 555.0, 555.0,RectAxisType::XZ,
                Arc::new(
                    Lambertian::new(Box::new(ColorTexture::new(white)))
                )
                )
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
            Rect::new(
                0.0, 555.0, 0.0, 555.0, 555.0,RectAxisType::XY,
                Arc::new(
                    Lambertian::new(Box::new(ColorTexture::new(white)))
                )
                )
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
                            Lambertian::new(Box::new(ColorTexture::new(white)))
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

    pub fn cornell_scene(&mut self) -> Camera {
        let red = Color::new(0.64, 0.05, 0.05);
        let white = Color::new(0.73,0.73,0.73);
        let green = Color::new(0.12, 0.45, 0.15);

        //light
        self.push(Box::new(
            Rect::new(
                    213.0, 343.0, 227.0, 332.0, 554.0,RectAxisType::XZ,
                    Arc::new(
                            DiffuseLight::new(Box::new(ColorTexture::new(
                                Vec3::new(15.0, 15.0, 15.0,))))
                    ),
            )
        ));
        
        self.push(Box::new(
            Rect::new(
                0.0, 555.0, 0.0, 555.0, 555.0,RectAxisType::YZ,
                Arc::new(
                    Lambertian::new(Box::new(ColorTexture::new(green)))
                )
            )
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
            Rect::new(
                0.0, 555.0, 0.0, 555.0, 555.0,RectAxisType::XZ,
                Arc::new(
                    Lambertian::new(Box::new(ColorTexture::new(white)))
                )
            )
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
            Rect::new(
                0.0, 555.0, 0.0, 555.0, 555.0,RectAxisType::XY,
                Arc::new(
                    Lambertian::new(Box::new(ColorTexture::new(white)))
                )
            )
        ));

        
        let mut box_list1: Vec<Box<dyn Shape>> = Vec::new();
        
        box_list1.push(Box::new(
            RectAngle::new(
                Vec3::new(130.0, 0.0, 65.0),Vec3::new(295.0, 165.0, 230.0),
                Arc::new(
                    Lambertian::new(Box::new(ColorTexture::new(white)))
                )
            )
        ));
        
        box_list1.push(Box::new(
            RectAngle::new(
                Vec3::new(265.0, 0.0, 295.0),Vec3::new(430.0, 330.0, 460.0),
                Arc::new(
                    Lambertian::new(Box::new(ColorTexture::new(white)))
                )
            )
        ));
        self.push(Box::new(BVH::new(box_list1)));

        
        // cornelbox用カメラ
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

    pub fn random_scene(&mut self) -> Camera {
        self.push(Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(Lambertian::new(Box::new(ColorTexture::new(Vec3::new(
                0.5, 0.5, 0.5,
            ))))),
        )));

        let mut box_list1: Vec<Box<dyn Shape>> = Vec::new();
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random();
                let center = Vec3::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());
                if (center - Vec3::new(4.0, 0.2, 0.0)).length().sqrt() > 0.9 {
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Vec3::random().mult(Vec3::random());
                        box_list1.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Lambertian::new(Box::new(ColorTexture::new(albedo)))),
                        )));
                    } else if choose_mat < 0.95 {
                        // Metal
                        let fuzz = random_range(0.0, 0.5);
                        let albedo = Vec3::vec3_random_range(0.5, 1.0);
                        box_list1.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Metal::new(Box::new(ColorTexture::new(albedo)), fuzz)),
                        )));
                    } else {
                        // glass
                        box_list1.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Dielectric::new(1.5)),
                        )));
                    }
                }
            }
        }
        self.push(Box::new(BVH::new(box_list1)));

        self.push(Box::new(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Arc::new(Dielectric::new(1.5)),
        )));
        self.push(Box::new(Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Arc::new(Lambertian::new(Box::new(ColorTexture::new(Vec3::new(
                0.4, 0.2, 0.1,
            ))))),
        )));
        self.push(Box::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Arc::new(Metal::new(
                Box::new(ColorTexture::new(Vec3::new(0.7, 0.6, 0.5))),
                0.0,
            )),
        )));

        // Camera
        // random_scene用カメラ
        let lookfrom = Vec3::new(13.0, 2.0, 3.0);
        let lookat = Vec3::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = (lookfrom - lookat).length().sqrt();
        let aperture = 0.1;

        return Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            WIDE_ASPECT,
            aperture,
            dist_to_focus,
        );
    }
}
