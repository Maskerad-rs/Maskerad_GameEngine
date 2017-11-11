extern crate cgmath;
extern crate rand;
pub mod core;

//In math :
//TODO: Line ?
//TODO: Ray ?
//TODO: Line segment ?
//TODO: Sphere (containing a center point C and a radius in a Vec4[Cx, Cy, Cz, r] to take advantage of SIMD) ?
//TODO: Planes (containing a a point P and a unit vector normal to the plane) ?
//TODO: Axis Aligned bounding box (contaigning 2 Points Pmin and Pmax) (aligned with the world space) ?
//TODO: Oriented bounding box (contaigning 2 Points Pmin and Pmax) (aligneted with the object space of the object it is bound to) ?
//TODO: Random number generator (using the 'rand' crate with the Xorshift (very fast, but not the best randomness) number generator ?)

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}