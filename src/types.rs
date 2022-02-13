
use crate::constants::{WIDTH, HEIGHT};
pub type Image = [[u32; WIDTH]; HEIGHT];

#[derive(Debug, Clone, Copy)]
pub struct Window {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}