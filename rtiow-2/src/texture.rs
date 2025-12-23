use crate::{
    color::{self, *},
    perlin::Perlin,
    vec3::Point3,
};
use image::{ImageReader, Pixel, RgbImage};
use std::path::Path;
use std::rc::Rc;

//just returning a constant color
pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> SolidColor {
        SolidColor { albedo: albedo }
    }

    pub fn new_color(red: f64, green: f64, blue: f64) {
        SolidColor::new(Color::new(red, green, blue));
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        self.albedo
    }
}

//every point in space has a color value defined by scale
pub struct CheckerTexture {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> CheckerTexture {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even: even,
            odd: odd,
        }
    }

    pub fn new_color(scale: f64, c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture::new(
            scale,
            Rc::new(SolidColor::new(c1)),
            Rc::new(SolidColor::new(c2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let x_integer = (self.inv_scale * p.x()).floor() as i64;
        let y_integer = (self.inv_scale * p.y()).floor() as i64;
        let z_integer = (self.inv_scale * p.z()).floor() as i64;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            return self.even.value(u, v, p);
        } else {
            return self.odd.value(u, v, p);
        }
    }
}

//uses uv coordinates to sample color data images, creating an image texture on the object -> both spheres and quads
pub struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> ImageTexture {
        let image = ImageReader::open(filename)
            .expect("Failed")
            .decode()
            .expect("Failed")
            .to_rgb8();

        ImageTexture { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        if self.image.width() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        //might result in a out of bounds if u or v or exatly 1?
        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;

        let pixel = self.image.get_pixel(i, j);

        let color_scale = 1.0 / 255.0;

        let r_srgb = color_scale * pixel[0] as f64;
        let g_srgb = color_scale * pixel[1] as f64;
        let b_srgb = color_scale * pixel[2] as f64;

        //Color::new(color_scale * pixel[0] as f64, color_scale * pixel[1] as f64, color_scale * pixel[2] as f64)
        Color::new(r_srgb.powi(2), g_srgb.powi(2), b_srgb.powi(2))
    }
}

//perlin noise texture black magic
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale: scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        //Color::new(1., 1., 1.) * self.noise.turb(p, 7)
        return Color::new(0.5, 0.5, 0.5)
            * (1. + (self.scale * p.z() + 10. * self.noise.turb(p, 7)).sin());
    }
}
