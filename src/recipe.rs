use std::any::{self, Any};

use anyhow::{Context, Result};

use crate::entity::{Line};
use crate::layer::Layer;

pub struct CannyEdge {}

// impl Layer for CannyEdge {
//     fn compute(&mut self, input: &Option<Box<dyn Any>>, output: &mut Option<Box<dyn Any>>) -> Result<()> {
//         todo!()
//     }
// }

// // pub struct CannyEdge {}

// // // impl Layer<&Image, Line> for CannyEdge {
// // //     fn compute(&self, input: &Image) -> Result<Line> {
// // //         let dings = Ok(Line {});
// // //         dings
// // //     }

// // //     fn display(&self) -> Option<Image> {
// // //         todo!()
// // //     }
// // // }

// // pub struct HarrisCorners {}

// // pub struct EllipseDetector {}

// pub struct CannyEdge{}

// impl Layer for CannyEdge {
//     fn compute(&mut self, input: &Box<dyn std::any::Any>) -> anyhow::Result<()> {
//         todo!()
//     }

//     fn interact(&self) {
//         todo!();
//     }
// }

// pub struct HarrisCorners{}

// pub struct EllipseDetector{}
