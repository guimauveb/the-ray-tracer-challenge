pub mod approx_eq;
pub mod float;
pub mod primitive;
pub mod projectile;
pub mod rt;

fn main() {
    if let Err(e) = projectile::launch_projecticle() {
        println!("{:#?}", e);
    }
}
