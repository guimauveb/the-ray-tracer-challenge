#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod approx_eq;
pub mod drawings;
pub mod float;
pub mod primitive;
pub mod rt;

use drawings::{ray_sphere, ray_sphere_3d};

fn main() {
    //if let Err(e) = projectile::launch_projecticle() {
    //    println!("{:#?}", e);
    //}
    //if let Err(e) = clock::draw_clock() {
    //    println!("{:#?}", e);
    //}
    if let Err(e) = ray_sphere::ray_sphere_hit() {
        println!("{:#?}", e);
    }
    if let Err(e) = ray_sphere_3d::ray_sphere_hit() {
        println!("{:#?}", e);
    }
}
