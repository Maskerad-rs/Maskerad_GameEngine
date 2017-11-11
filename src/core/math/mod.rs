//At this time, we use the 'cgmath' crate to get the fundamentals math types and functions.
//The idea, for the moment, is to wrap the cgmath's type with simple typedefs.

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