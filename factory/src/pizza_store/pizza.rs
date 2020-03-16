use std::fmt;
use super::ingredient::*;


pub trait Pizza : fmt::Debug {
    fn prepare(&mut self);

    fn bake(&mut self) {
        println!("Bake for 25 minutes at 250");
    }

    fn cut(&mut self) {
        println!("Cutting into slices");
    }

    fn pack_in_box(&mut self) {
        println!("Place pizza in box");
    }
}