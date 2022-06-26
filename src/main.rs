#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod approx_eq;
pub mod drawings;
pub mod float;
pub mod rt;
pub mod tuple;

// TODO - Test patterns
use rt::{
    color::Color,
    matrix::Matrix,
    patterns::{Checkers, Gradient, Ring},
};

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
    if let Err(e) = drawings::camera::spheres() {
        println!("{:#?}", e);
    }
    //if let Err(e) = drawings::plane::spheres() {
    //    println!("{:#?}", e);
    //}
}
