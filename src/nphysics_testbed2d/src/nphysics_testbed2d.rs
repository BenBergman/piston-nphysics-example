#![warn(non_camel_case_types)]

extern crate num;
extern crate rand;
extern crate time;
extern crate nalgebra as na;
extern crate ncollide;
extern crate nphysics;


pub use testbed::Testbed;

mod testbed;
mod engine;

mod objects {
    pub mod ball;
}
