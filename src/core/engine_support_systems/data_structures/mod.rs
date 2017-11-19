pub mod threadpools;

use std::fmt;

pub trait ThreadPool : fmt::Debug {
    fn get_number_of_thread(&self) -> usize;
}