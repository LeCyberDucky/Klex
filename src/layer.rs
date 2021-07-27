use std::any::{self, Any};

use anyhow::{Context, Result};
use iced_native::Widget;

pub trait Layer {
    fn compute(
        &mut self,
        input: &[&Option<Box<dyn Any>>],
        output: &mut Option<Box<dyn Any>>,
    ) -> Result<()>; 
}

pub trait InteractiveLayer<Message, Renderer: iced_native::Renderer>: Layer + Widget<Message, Renderer> {
    fn interact(&self) {
        // Default implementation for layers that don't provide special user interation. Can be overwritten to allow for layer-specific user interaction
        todo!();
    }
}


pub mod primitive {
    use super::*;

    use image::{GrayImage, RgbaImage};

    use crate::entity::{self, BinaryImage};

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

    impl Default for Convert<RgbaImage, GrayImage> {
        fn default() -> Self {
            Self::new()
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

    impl Default for Convert<BinaryImage, GrayImage> {
        fn default() -> Self {
            Self::new()
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

    // https://github.com/hecrj/iced/blob/master/examples/bezier_tool/src/main.rs
    // https://docs.rs/iced_native/0.4.0/iced_native/widget/trait.Widget.html

    impl<A: 'static, B: 'static, Message, Renderer: iced_native::Renderer> Widget<Message, Renderer> for Convert<A, B> {
        fn width(&self) -> iced::Length {
        todo!()
    }

        fn height(&self) -> iced::Length {
        todo!()
    }

        fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        todo!()
    }

        fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) -> Renderer::Output {
        todo!()
    }

        fn hash_layout(&self, state: &mut iced_native::Hasher) {
        todo!()
    }
    }
    
    impl<A: 'static, B: 'static, Message, Renderer: iced_native::Renderer> InteractiveLayer<Message, Renderer> for Convert<A, B> {}
    

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
            *output = Some(Box::new((self.operation)(self)?));
            Ok(())
        }
    }

    impl<A: 'static, Message, Renderer: iced_native::Renderer> Widget<Message, Renderer> for InputFile<A> {
        fn width(&self) -> iced::Length {
        todo!()
    }

        fn height(&self) -> iced::Length {
        todo!()
    }

        fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        todo!()
    }

        fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) -> Renderer::Output {
        todo!()
    }

        fn hash_layout(&self, state: &mut iced_native::Hasher) {
        todo!()
    }
    }

    impl<A: 'static, Message, Renderer: iced_native::Renderer> InteractiveLayer<Message, Renderer> for InputFile<A> {}

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
            *output = Some(Box::new((self.operation)(self, input)));
            Ok(())
        }
    }

    impl<A: 'static, B: 'static, T, Message, Renderer: iced_native::Renderer> Widget<Message, Renderer> for Threshold<A, B, T> {
        fn width(&self) -> iced::Length {
        todo!()
    }

        fn height(&self) -> iced::Length {
        todo!()
    }

        fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        todo!()
    }

        fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) -> Renderer::Output {
        todo!()
    }

        fn hash_layout(&self, state: &mut iced_native::Hasher) {
        todo!()
    }
    }
    
    impl<A: 'static, B: 'static, T, Message, Renderer: iced_native::Renderer> InteractiveLayer<Message, Renderer> for Threshold<A, B, T> {}



    pub struct TransformAffine<A> {
        operation: fn(&A) -> Result<A>,
    }
}
