//At this time, we use the 'cgmath' crate to get the fundamentals math types and functions.
//The idea, for the moment, is to wrap the cgmath's type with simple typedefs.

//TODO: Line ?
//TODO: Ray ?
//TODO: Line segment ?
//TODO: Sphere (containing a center point C and a radius in a Vec4[Cx, Cy, Cz, r] to take advantage of SIMD) ?
//TODO: Planes (containing a a point P and a unit vector normal to the plane) ?
//TODO: Axis Aligned bounding box (contaigning 2 Points Pmin and Pmax) (aligned with the world space) ?
//TODO: Oriented bounding box (contaigning 2 Points Pmin and Pmax) (aligneted with the object space of the object it is bound to) ?

pub mod vector;
pub mod point;
pub mod matrix;
pub mod basis;
pub mod quaternion;
pub mod decomposed;
pub mod deg;
pub mod rad;
pub mod euler_angle;
pub mod ortho;
pub mod perspective;
pub mod perspective_fov;