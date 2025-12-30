#![allow(dead_code)]

mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod quad;
mod ray;
mod sphere;
mod texture;
mod triangle;
mod utils;
mod vec3;

use bvh::BvhNode;
use camera::Camera;
use color::Color;
use hittable::{RotateY, Translate};
use hittable_list::HittableList;
use material::DiffuseLight;
use material::{Dialectric, Material};
use material::{Lambertian, Metal};
use quad::*;
use sphere::Sphere;
use std::rc::Rc;
use texture::{CheckerTexture, ImageTexture, NoiseTexture};
use triangle::*;
use utils::{random_double, random_double_range};
use vec3::{Point3, Vec3};

fn till_final() {
    let mut world: HittableList = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));

    let material_left = Rc::new(Dialectric::new(1.5, 1.0));
    let material_bubble = Rc::new(Dialectric::new(1.0 / 1.5, 1.0));

    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut cam = Camera::new(16.0 / 9.0, 300, 200, 50, 50.0, 0.0, 10.0);
    cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    cam.lookat = Point3::new(0.0, 0.0, -1.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);
    cam.background = Color::new(0.7, 0.8, 1.);

    cam.render(&world).unwrap();
}

fn bouncing_spheres() {
    let mut world: HittableList = HittableList::new();

    let checker = Rc::new(CheckerTexture::new_color(
        1.0,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new_tex(checker)),
    )));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0., random_double_range(0., 0.5), 0.);
                    world.add(Rc::new(Sphere::new_to(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random();
                    let fuzz = random_double_range(0.0, 0.2);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    sphere_material = Rc::new(Dialectric::new(1.5, 0.1));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dialectric::new(1.5, 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world = HittableList::new_list(Rc::new(BvhNode::new(world)));

    let mut cam = Camera::new(16.0 / 9.0, 500, 100, 50, 20.0, 0.6, 10.0);

    cam.lookfrom = Point3::new(13., 2., 3.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);
    cam.background = Color::new(0.7, 0.8, 1.);

    cam.render(&world).unwrap();
}

fn checkered_spheres() {
    let mut world: HittableList = HittableList::new();

    let checker = Rc::new(CheckerTexture::new_color(
        1.0,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new_tex(checker.clone())),
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new_tex(checker)),
    )));

    world = HittableList::new_list(Rc::new(BvhNode::new(world)));

    let mut cam = Camera::new(16.0 / 9.0, 500, 100, 50, 20.0, 0., 10.0);

    cam.lookfrom = Point3::new(13., 2., 3.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);
    cam.background = Color::new(0.7, 0.8, 1.);

    cam.render(&world).unwrap();
}

fn earth() {
    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Rc::new(Lambertian::new_tex(earth_texture));

    let globe = Rc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let world = HittableList::new_list(globe);

    let mut cam = Camera::new(16.0 / 9.0, 500, 100, 50, 20.0, 0., 10.0);

    cam.lookfrom = Point3::new(0., 0., 12.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);
    cam.background = Color::new(0.7, 0.8, 1.);

    cam.render(&world).unwrap();
}

fn perlin_spheres() {
    let pertext = Rc::new(NoiseTexture::new(4.0));

    let mut world: HittableList = HittableList::new();

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new_tex(pertext.clone())),
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new_tex(pertext)),
    )));

    let mut cam = Camera::new(16.0 / 9.0, 500, 100, 50, 20.0, 0., 10.0);

    cam.lookfrom = Point3::new(13., 2., 3.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);
    cam.background = Color::new(0.7, 0.8, 1.);

    cam.render(&world).unwrap();
}

fn quads() {
    let mut world: HittableList = HittableList::new();

    let left_red = Rc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let right_blue = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Rc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Rc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Rc::new(Lambertian::new_tex(earth_texture));

    world.add(Rc::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        earth_surface,
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let mut cam = Camera::new(1.0, 500, 100, 50, 80.0, 0., 10.0);

    cam.lookfrom = Point3::new(0., 0., 9.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);
    cam.background = Color::new(0.7, 0.8, 1.);

    cam.render(&world).unwrap();
}

fn simple_light() {
    let pertext = Rc::new(NoiseTexture::new(4.0));

    let mut world: HittableList = HittableList::new();

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new_tex(pertext.clone())),
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new_tex(pertext)),
    )));

    let difflight = Rc::new(DiffuseLight::new(Color::new(4., 4., 4.)));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight,
    )));

    let mut cam = Camera::new(16.0 / 9.0, 500, 100, 50, 20.0, 0., 10.0);

    cam.lookfrom = Point3::new(26., 3., 6.);
    cam.lookat = Point3::new(0., 2., 0.);
    cam.vup = Vec3::new(0., 1., 0.);
    cam.background = Color::new(0., 0., 0.);

    cam.render(&world).unwrap();
}

fn cornell_box() {
    let mut world: HittableList = HittableList::new();

    let red = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(Color::new(15., 15., 15.)));

    world.add(Rc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box1 = boxx(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        white.clone(),
    );

    box1 = Rc::new(RotateY::new(box1, 15.));
    box1 = Rc::new(Translate::new(box1, Vec3::new(265., 0., 295.)));
    world.add(box1);

    let mut box2 = boxx(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white.clone(),
    );

    box2 = Rc::new(RotateY::new(box2, -18.));
    box2 = Rc::new(Translate::new(box2, Vec3::new(130., 0., 65.)));
    world.add(box2);

    world = HittableList::new_list(Rc::new(BvhNode::new(world)));

    let mut cam = Camera::new(1.0, 300, 100, 50, 40.0, 0.0, 10.0);

    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world).unwrap();
}

fn cornell_smoke() {
    let mut world: HittableList = HittableList::new();

    let red = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(Color::new(7., 7., 7.)));

    world.add(Rc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light.clone(),
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box1 = boxx(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        white.clone(),
    );

    box1 = Rc::new(RotateY::new(box1, 15.));
    box1 = Rc::new(Translate::new(box1, Vec3::new(265., 0., 295.)));

    let mut box2 = boxx(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white.clone(),
    );

    box2 = Rc::new(RotateY::new(box2, -18.));
    box2 = Rc::new(Translate::new(box2, Vec3::new(130., 0., 65.)));

    world.add(Rc::new(constant_medium::ConstantMedium::new(
        box1,
        0.01,
        Color::new(0., 0., 0.),
    )));
    world.add(Rc::new(constant_medium::ConstantMedium::new(
        box2,
        0.01,
        Color::new(1., 1., 1.),
    )));

    world = HittableList::new_list(Rc::new(BvhNode::new(world)));

    let mut cam = Camera::new(1.0, 600, 200, 50, 40.0, 0.0, 10.0);

    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world).unwrap();
}

fn final_scene() {
    let mut boxes1: HittableList = HittableList::new();
    let ground = Rc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1., 101.);
            let z1 = z0 + w;

            boxes1.add(boxx(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world: HittableList = HittableList::new();

    world.add(Rc::new(BvhNode::new(boxes1)));

    let light = Rc::new(DiffuseLight::new(Color::new(7., 7., 7.)));
    world.add(Rc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light.clone(),
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));

    world.add(Rc::new(Sphere::new_to(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Rc::new(Dialectric::new(1.5, 0.0)),
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Rc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Rc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Rc::new(Dialectric::new(1.5, 0.0)),
    ));

    world.add(boundary.clone());

    world.add(Rc::new(constant_medium::ConstantMedium::new(
        boundary.clone(),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    let boundary2 = Rc::new(Sphere::new(
        Point3::new(0., 0., 0.),
        5000.0,
        Rc::new(Dialectric::new(1.5, 0.0)),
    ));

    world.add(Rc::new(constant_medium::ConstantMedium::new(
        boundary2,
        0.0001,
        Color::new(1., 1., 1.),
    )));

    let emat = Rc::new(Lambertian::new_tex(Rc::new(ImageTexture::new(
        "earthmap.jpg",
    ))));

    world.add(Rc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));

    let pertext = Rc::new(NoiseTexture::new(0.2));
    world.add(Rc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Rc::new(Lambertian::new_tex(pertext.clone())),
    )));

    let mut boxes2: HittableList = HittableList::new();
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Rc::new(Sphere::new(
            Vec3::random_range(0., 165.),
            10.0,
            white.clone(),
        )));
    }

    world.add(Rc::new(Translate::new(
        Rc::new(RotateY::new(Rc::new(BvhNode::new(boxes2)), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let mut cam = Camera::new(1.0, 800, 5000, 40, 40.0, 0.0, 10.0);

    cam.lookfrom = Point3::new(478.0, 278.0, -600.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world).unwrap();
}

fn cornell_car() {
    let mut world: HittableList = HittableList::new();

    let red = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(Color::new(15., 15., 15.)));
    let mirror = Rc::new(Metal::new(Vec3::new(0.4, 0.4, 0.4), 0.01));

    let box_width = 1050.0;
    let box_height = 600.0;
    let box_depth = 800.0;
    let center_x = box_width / 2.0;
    let center_y = box_height / 2.0;

    //left
    world.add(Rc::new(Quad::new(
        Vec3::new(box_width, 0.0, 0.0),
        Vec3::new(0.0, box_height, 0.0),
        Vec3::new(0.0, 0.0, box_depth),
        green.clone(),
    )));

    //right
    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, box_height, 0.0),
        Vec3::new(0.0, 0.0, box_depth),
        red.clone(),
    )));

    //lgiht
    world.add(Rc::new(Quad::new(
        Vec3::new(center_x + 260.0, box_height - 1.0, center_y + 105.0),
        Vec3::new(-520.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -210.0),
        light.clone(),
    )));

    //bottom
    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(box_width, 0.0, 0.0),
        Vec3::new(0.0, 0.0, box_depth),
        mirror.clone(),
    )));

    //top
    world.add(Rc::new(Quad::new(
        Vec3::new(box_width, box_height, box_depth),
        Vec3::new(-box_width, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -box_depth),
        white.clone(),
    )));

    //back
    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, box_depth),
        Vec3::new(box_width, 0.0, 0.0),
        Vec3::new(0.0, box_height, 0.0),
        white.clone(),
    )));

    let mut triangles = HittableList::new();

    let (models, _materials) = tobj::load_obj(
        "porsche_911_with_interior.obj",
        &tobj::OFFLINE_RENDERING_LOAD_OPTIONS,
    )
    .expect("Failed to load OBJ");

    let car_material = Rc::new(Metal::new(Vec3::new(0.1, 0.1, 0.1), 0.0));
    let scale = 200.0;
    let offset = Vec3::new(590.0, 28.0, 640.0); //completely madeup by just looking

    for (_i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        for idx in mesh.indices.chunks(3) {
            let v0 = idx[0] as usize;
            let v1 = idx[1] as usize;
            let v2 = idx[2] as usize;

            let a = Point3::new(
                mesh.positions[3 * v0] as f64 * scale + offset[0],
                mesh.positions[3 * v0 + 1] as f64 * scale + offset[1],
                mesh.positions[3 * v0 + 2] as f64 * scale + offset[2],
            );

            let b = Point3::new(
                mesh.positions[3 * v1] as f64 * scale + offset[0],
                mesh.positions[3 * v1 + 1] as f64 * scale + offset[1],
                mesh.positions[3 * v1 + 2] as f64 * scale + offset[2],
            );

            let c = Point3::new(
                mesh.positions[3 * v2] as f64 * scale + offset[0],
                mesh.positions[3 * v2 + 1] as f64 * scale + offset[1],
                mesh.positions[3 * v2 + 2] as f64 * scale + offset[2],
            );
            triangles.add(Rc::new(RotateY::new(
                Rc::new(Triangle::new(a, b, c, car_material.clone())),
                40.0,
            )));
        }
    }

    world.add(Rc::new(BvhNode::new(triangles)));

    let mut cam = Camera::new(16.0 / 9.0, 700, 20, 10, 40.0, 0.0, 10.0);
    cam.lookfrom = Point3::new(center_x, center_y, -800.0);
    cam.lookat = Point3::new(center_x, center_y, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.background = Color::new(0.7, 0.8, 1.);
    cam.defocus_angle = 0.0;
    cam.render(&world).unwrap();
}

fn main() -> () {
    //till_final();
    //bouncing_spheres();
    //checkered_spheres();
    //earth();
    //perlin_spheres();
    //quads();
    //simple_light();
    //cornell_box();
    //cornell_smoke();
    //final_scene();
    cornell_car();
}
