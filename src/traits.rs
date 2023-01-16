//! In this module are traits defined...
use crate::{
    structs::*,
    error::Result,
};


pub trait FunctionalDescription {
    /// What does this method?!
    fn get_something(data: &SomeData) -> Result<u8>;
}