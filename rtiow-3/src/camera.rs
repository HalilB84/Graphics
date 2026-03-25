use crate::color::*;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::ScatterRecord;
use crate::pdf::CosinePDF;
use crate::pdf::HittablePDF;
use crate::pdf::MixturePDF;
use crate::pdf::PDF;
use crate::ray::Ray;
use crate::utils::random_double;
use crate::vec3::{Point3, Vec3};
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i64,
    pub samples_per_pixel: i64,
    pub max_depth: i64,
    pub background: Color,
    image_height: i64,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i64,
        samples: i64,
        depth: i64,
        fov: f64,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Camera {
        Camera {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            samples_per_pixel: samples,
            max_depth: depth,
            background: Color::new(0.0, 0.0, 0.0),
            image_height: 0,
            pixel_samples_scale: 0.,
            center: Point3::new(0., 0., 0.),
            pixel00_loc: Point3::new(0., 0., 0.),
            pixel_delta_u: Vec3::new(0., 0., 0.),
            pixel_delta_v: Vec3::new(0., 0., 0.),
            vfov: fov,
            lookfrom: Point3::new(0., 0., 0.),
            lookat: Point3::new(0., 0., -1.),
            vup: Vec3::new(0., 1., 0.),
            defocus_angle: defocus_angle,
            focus_dist: focus_dist,
            defocus_disk_u: Vec3::new(0., 0., 0.),
            defocus_disk_v: Vec3::new(0., 0., 0.),
        }
    }

    pub fn render(
        &mut self,
        world: &dyn Hittable,
        lights: Rc<dyn Hittable>,
    ) -> std::io::Result<()> {
        self.initialize();

        let mut file = File::create("image.ppm")?;
        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height)?;

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color: Color = Color::new(0., 0., 0.);

                let index = self.image_width * j + i;

                if index % 1000 == 0 {
                    println!(
                        "currently processing {}/{}",
                        index,
                        self.image_width * self.image_height
                    );
                }

                for _sample in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color =
                        pixel_color + self.ray_color(&r, self.max_depth, world, lights.clone());
                }

                write_color(&(pixel_color * self.pixel_samples_scale), &mut file).unwrap();
            }
        }
        Ok(())
    }

    fn initialize(&mut self) -> () {
        //we do give an aspect ratio, but that aspect ratio is not guranteed to result in a valid image pixel wise. This calculation below is a little different than our aspect
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i64;
        self.image_height = self.image_height.max(1);

        self.pixel_samples_scale = 1. / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();

        //times focus_dist because tan = o/a = h = vh2 / focal__dist
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        //ok so what we have here is the vectors that show the camera orientation
        //w is the direction unit vector looking at lookfrom from lookat
        //u is the right unit vector perpendicular to the plane formed by vup and w -> note that vup is a dummy vector just to calculate u and breaks if vup and w are parallel v is the up vector perpendicular to w and u -> this is the actual up direction of the camera
        //note that this orientation of w relative to the vup vector chooes where the top left of the viewport ends in world space. When I say left in the cornell box it is also relative to the camera, if the camera was looking from +z it would be to the right
        let w = Vec3::unit_vector(self.lookfrom - self.lookat);
        let u = Vec3::unit_vector(Vec3::cross(self.vup, w));
        let v = Vec3::cross(w, u);

        //these are the vectors that form the viewport
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        //vectors between pixels in the viewport
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        //the point location of the top left corner of the viewport, remember that w is a unit vector just for direction!
        let viewport_upper_left =
            self.center - (self.focus_dist * w) - viewport_u / 2. - viewport_v / 2.;

        //actual center of the pixel
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (self.defocus_angle.to_radians() / 2.0).tan();
        self.defocus_disk_u = defocus_radius * u;
        self.defocus_disk_v = defocus_radius * v;
    }

    //helper function to get where the ray starts withing the defocus disk and the direction of the ray
    fn get_ray(&self, i: i64, j: i64) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    //tweaking out the ray within the pixel so it looks less pixelated as things are blended
    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.)
    }

    //the p vector here is treated as a scalar for the defocus disk vectors
    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn ray_color(
        &self,
        r: &Ray,
        depth: i64,
        world: &dyn Hittable,
        lights: Rc<dyn Hittable>,
    ) -> Color {
        if depth <= 0 {
            return Color::new(0., 0., 0.);
        }

        let mut rec = HitRecord::new();

        //because of floating point errors we have a 0.001 min to ensure rays
        //dont self intersect. If starting ray is below the sphere it will hit itself
        if !world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            return self.background;
        }

        let mut srec = ScatterRecord::new();
        let color_from_emission = rec.mat.emitted(&rec, rec.u, rec.v, rec.p);

        //if scatter returns false it means that the material doesnt scatter the ray for some
        //reason. In this case we only case about the emission if there is any.
        if !rec.mat.scatter(r, &rec, &mut srec) {
            return color_from_emission;
        }

        if srec.skip_pdf {
            return srec.attenuation
                * self.ray_color(&srec.skip_ray, depth - 1, world, lights.clone());
        }

        //(I think) what the book fails to mention is that we haven't come up with
        //a better single PDF that recuces noise, the actual improvement happens in MIS
        //only lights PDF is broken becaues it voilates the rules of monte carlo (f(x) > 0 -> p(x) > 0)
        let light_ptr = Rc::new(HittablePDF::new(lights.clone(), rec.p));
        //        let mixed_pdf = MixturePDF::new(light_ptr, srec.pdf);

        let scattered = Ray::new(rec.p, light_ptr.generate(), r.time());
        let pdf = light_ptr.value(scattered.direction());

        //as long as the pdf and the scatter generation match, it will converge to the same result with varying speeds.
        //When scatter pdf changes you are changing how the material reacts. (I think)
        let scatter_pdf = rec.mat.scatter_pdf(&r, &rec, &scattered);
        //pdf = scatter_pdf; //we implicitly had this this in book 1&2

        let color_from_scatter = srec.attenuation
            * scatter_pdf
            * self.ray_color(&scattered, depth - 1, world, lights.clone())
            / pdf;

        color_from_emission + color_from_scatter
    }
}
