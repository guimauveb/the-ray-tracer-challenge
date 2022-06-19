#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod approx_eq;
pub mod drawings;
pub mod float;
pub mod rt;
pub mod tuple;

use rt::{color::Color, matrix::Matrix, patterns::Ring};

fn main() {
    //if let Err(e) = drawing::projectile::launch_projecticle() {
    //    println!("{:#?}", e);
    //}
    //if let Err(e) = drawings::clock::draw_clock() {
    //    println!("{:#?}", e);
    //}
    // if let Err(e) = drawings::ray_sphere::ray_sphere_hit() {
    //     println!("{:#?}", e);
    // }
    // if let Err(e) = drawings::ray_sphere_3d::ray_sphere_hit() {
    //     println!("{:#?}", e);
    // }
    if let Err(e) = drawings::camera::spheres(Some(
        Ring::new(
            Color::white(),
            Color::black(),
            Some(Matrix::<4>::scaling(0.1, 0.1, 0.1)),
        )
        .into(),
    )) {
        println!("{:#?}", e);
    }
    //if let Err(e) = drawings::plane::spheres() {
    //    println!("{:#?}", e);
    //}
}
