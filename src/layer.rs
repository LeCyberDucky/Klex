use std::any::{self, Any};

use anyhow::{Context, Result};

pub trait Layer {
    fn compute(
        &mut self,
        input: &[&Option<Box<dyn Any>>],
        output: &mut Option<Box<dyn Any>>,
    ) -> Result<()>; // Individual implementation necessary for every struct implementing this
}

pub trait InteractiveLayer: Layer {
    fn interact(&self) {
        // Default implementation for layers that don't provide special user interation. Can be overwritten to allow for layer-specific user interaction
        todo!();
    }
}


pub mod primitive {
    use super::*;

    use image::{GrayImage, RgbaImage};

    use crate::entity::{self, BinaryImage, Point};

    pub struct Convert<A, B> {
        operation: fn(&A) -> Result<B>,
    }

    impl Convert<RgbaImage, GrayImage> {
        pub fn new() -> Self {
            Self {
                operation: Self::compute,
            }
        }

        pub fn compute(input: &RgbaImage) -> Result<GrayImage> {
            Ok(image::imageops::colorops::grayscale(input))
        }
    }

    impl Convert<BinaryImage, GrayImage> {
        pub fn new() -> Self {
            Self {
                operation: Self::compute,
            }
        }

        pub fn compute(input: &BinaryImage) -> Result<GrayImage> {
            let data = input.data().iter().map(|&pixel| if pixel {u8::MAX} else {u8::MIN}).collect();
            GrayImage::from_vec(input.width(), input.height(), data).context("Data cannot be converted to GrayImage")
        }
    }
    
    impl<A: 'static, B: 'static> Layer for Convert<A, B> {
        fn compute(
            &mut self,
            input: &[&Option<Box<dyn Any>>],
            output: &mut Option<Box<dyn Any>>,
        ) -> Result<()> {
            let input = input[0]; // Convert only expects input from a single source layer
            let input = input.as_ref().context("Empty input")?;
            let input = input.downcast_ref::<A>().context(format!(
                "Casting failed. Expected input of type {:#?}",
                any::type_name::<A>()
            ))?;
            *output = Some(Box::new((self.operation)(input)?));
            Ok(())
        }
    }
    
    impl<A: 'static, B: 'static> InteractiveLayer for Convert<A, B> {}
    

    pub struct Convolve {}

    pub struct InputFile<A> {
        file_path: std::path::PathBuf,
        operation: fn(&Self) -> Result<A>,
    }

    impl InputFile<RgbaImage> {
        pub fn new(file_path: std::path::PathBuf) -> Self {
            Self {
                file_path,
                operation: Self::compute,
            }
        }
    
        pub fn compute(&self) -> Result<RgbaImage> {
            Ok(image::open(&self.file_path)?.into_rgba8())
        }
    }

    impl<A: 'static> Layer for InputFile<A> {
        fn compute(
            &mut self,
            _input: &[&Option<Box<dyn Any>>], // This layer does not depend on other layers
            output: &mut Option<Box<dyn Any>>,
        ) -> Result<()> {
            *output = Some(Box::new((self.operation)(&self)?));
            Ok(())
        }
    }

    impl<A: 'static> InteractiveLayer for InputFile<A> {}

    pub struct Threshold<A, B, T> {
        threshold: T,
        ordering: std::cmp::Ordering,
        operation: fn(&Self, input: &A) -> B,
    }

impl Threshold<GrayImage, entity::BinaryImage, u8> {
    pub fn new(threshold: u8, ordering: std::cmp::Ordering) -> Self {
        Self {
            threshold,
            ordering,
            operation: Self::compute,
        }

    }

    pub fn compute(&self, input: &GrayImage) -> entity::BinaryImage {
        let data = input.pixels().map(|pixel| pixel.0[0].cmp(&self.threshold) == self.ordering).collect();
        entity::BinaryImage::new(input.width(), input.height(), data)
    }
}

    

    impl<A: 'static, B: 'static, T> Layer for Threshold<A, B, T> {
        fn compute(&mut self, input: &[&Option<Box<dyn Any>>], output: &mut Option<Box<dyn Any>>) -> Result<()> {
            let input = input[0]; // Threshold only expects input from a single source layer
            let input = input.as_ref().context("Empty input")?;
            let input = input.downcast_ref::<A>().context(format!(
                "Casting failed. Expected input of type {:#?}",
                any::type_name::<A>()
            ))?;
            *output = Some(Box::new((self.operation)(&self, input)));
            Ok(())
        }
    }
    
    impl<A: 'static, B: 'static, T> InteractiveLayer for Threshold<A, B, T> {}



    pub struct TransformAffine<A> {
        operation: fn(&A) -> Result<A>,
    }
}
