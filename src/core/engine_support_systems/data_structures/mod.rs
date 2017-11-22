pub mod threadpools;

use std::fmt;

//TODO: A join_all(&self) function ? Like threadpool send a shut_down message, and workers have a loop like this : while (!shut_down) instead of loop{}.
pub trait ThreadPool : fmt::Debug {
    fn number_of_thread(&self) -> usize;
}