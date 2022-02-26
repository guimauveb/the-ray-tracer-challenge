#![feature(generic_const_exprs)]

pub mod approx_eq;
pub mod drawings;
pub mod float;
pub mod primitive;
pub mod rt;

use drawings::{clock, projectile};

fn main() {
    if let Err(e) = projectile::launch_projecticle() {
        println!("{:#?}", e);
    }
    if let Err(e) = clock::draw_clock() {
        println!("{:#?}", e);
    }
}
