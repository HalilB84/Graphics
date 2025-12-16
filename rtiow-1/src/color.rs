use std::fs::File;
use std::io::prelude::*;

use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

//gamma/srgb shi because humans see colors nonlinearly and that stuff
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }

    0.0
}

pub fn write_color(pixel_color: &Color, file: &mut File) -> std::io::Result<()> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity: Interval = Interval::new(0.000, 0.999);

    let rbyte: u32 = (256. * intensity.clamp(r)) as u32;
    let gbyte: u32 = (256. * intensity.clamp(g)) as u32;
    let bbyte: u32 = (256. * intensity.clamp(b)) as u32;

    writeln!(file, "{} {} {}", rbyte, gbyte, bbyte)?;

    Ok(())
}
