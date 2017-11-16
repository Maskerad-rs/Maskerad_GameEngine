extern crate cgmath;
extern crate rand;

pub mod core;
pub mod systems;
pub mod gameplay;
pub mod game;


//In math :
//TODO: Line ?
//TODO: Ray ?
//TODO: Line segment ?
//TODO: Sphere (containing a center point C and a radius in a Vec4[Cx, Cy, Cz, r] to take advantage of SIMD) ?
//TODO: Planes (containing a a point P and a unit vector normal to the plane) ?
//TODO: Axis Aligned bounding box (contaigning 2 Points Pmin and Pmax) (aligned with the world space) ?
//TODO: Oriented bounding box (contaigning 2 Points Pmin and Pmax) (aligneted with the object space of the object it is bound to) ?