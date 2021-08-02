use std::any::{self, Any};

use anyhow::{Context, Result, anyhow};
use iced_native::Widget;

pub trait Layer {
    fn compute(
        &self,
        input: &[Option<&dyn Any>],
    ) -> Result<(Option<Box<dyn Any>>, Option<Box<dyn Any>>)>; 

    fn update(&mut self, output: Option<Box<dyn Any>>, state_updates: Option<Box<dyn Any>>) -> Result<()>;

    fn output(&self) -> Option<&dyn Any>;
}

pub trait InteractiveLayer<Message, Renderer: iced_native::Renderer>: Layer + Widget<Message, Renderer> {}


pub mod primitive {
    use super::*;

    use std::hash::Hash;

    use iced::Length;
    use iced_native::{Size, layout::Node};
    use image::{GrayImage, RgbaImage};

    use crate::element::{self, BinImage};

    pub struct Convert<A, B> {
        operation: fn(&A) -> Result<B>,
        output: Option<B>
    }

    impl Convert<RgbaImage, GrayImage> {
        pub fn new() -> Self {
            Self {
                operation: Self::compute,
                output: None
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

    impl Convert<BinImage, GrayImage> {
        pub fn new() -> Self {
            Self {
                operation: Self::compute,
                output: None
            }
        }

        pub fn compute(input: &BinImage) -> Result<GrayImage> {
            let data = input.data().iter().map(|&pixel| if pixel {u8::MAX} else {u8::MIN}).collect();
            GrayImage::from_vec(input.width(), input.height(), data).context("Data cannot be converted to GrayImage")
        }
    }

    impl Default for Convert<BinImage, GrayImage> {
        fn default() -> Self {
            Self::new()
        }
    }
    
    impl<A: 'static, B: 'static> Layer for Convert<A, B> {
        fn compute(
        &self,
        input: &[Option<&dyn Any>],
    ) -> Result<(Option<Box<dyn Any>>, Option<Box<dyn Any>>)> {
        let input = input[0]; // Convert only expects input from a single source layer
        let input = input.context("Empty input")?;
        let input = input.downcast_ref::<A>().context(format!(
            "Casting failed. Expected input of type {:#?}",
            any::type_name::<A>()
        ))?;

        let output = Some(Box::new((self.operation)(input)?) as Box::<dyn Any>);
        let state_updates = None;
        Ok((output, state_updates))
    }

    fn update(&mut self, output: Option<Box<dyn Any>>, state_updates: Option<Box<dyn Any>>) -> Result<()> {
        self.output = output.map(|content|
            content.downcast::<B>().map(|inner_content| *inner_content)
        ).transpose().map_err(|_| anyhow!("Casting failed. Expected input of type {:#?}",
        any::type_name::<B>()))?;

        match state_updates {
            Some(_) => todo!(),
            None => (),
        }

        Ok(())
    }

    fn output(&self) -> Option<&dyn Any> {
        self.output.as_ref().map(|x| x as &dyn Any)
    }

    }
    

    // https://github.com/hecrj/iced/blob/master/examples/bezier_tool/src/main.rs
    // https://docs.rs/iced_native/0.4.0/iced_native/widget/trait.Widget.html

    impl<A: 'static, B: 'static, Message, Renderer: iced_native::Renderer> Widget<Message, Renderer> for Convert<A, B> {
        fn width(&self) -> Length {
        todo!()
    }

        fn height(&self) -> Length {
        todo!()
    }

        fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> Node {
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

    // #############
    // #############
    // # InputFile #
    // #############    
    // #############
    pub struct InputFile<A> {
        file_path: std::path::PathBuf,
        operation: fn(&Self) -> Result<A>,
        output: Option<A>
    }

    impl InputFile<RgbaImage> {
        pub fn new(file_path: std::path::PathBuf) -> Self {
            Self {
                file_path,
                operation: Self::compute,
                output: None
            }
        }
    
        pub fn compute(&self) -> Result<RgbaImage> {
            Ok(image::open(&self.file_path)?.into_rgba8())
        }

        fn width(&self) -> Option<usize> {
            Some(self.output.as_ref()?.dimensions().0 as usize)
        }

        fn height(&self) -> Option<usize> {
            Some(self.output.as_ref()?.dimensions().1 as usize)
        }
    }

    impl<A: 'static> Layer for InputFile<A> {
        fn compute(
        &self,
        _input: &[Option<&dyn Any>], // This layer does not depend on other layers
    ) -> Result<(Option<Box<dyn Any>>, Option<Box<dyn Any>>)> {
        let output = Some(Box::new((self.operation)(self)?) as Box::<dyn Any>);
        let state_updates = None;
        Ok((output, state_updates))
    }

    fn update(&mut self, output: Option<Box<dyn Any>>, state_updates: Option<Box<dyn Any>>) -> Result<()> {
        self.output = output.map(|content|
            content.downcast::<A>().map(|inner_content| *inner_content)
        ).transpose().map_err(|_| anyhow!("Casting failed. Expected input of type {:#?}",
        any::type_name::<A>()))?;

        match state_updates {
            Some(_) => todo!(),
            None => (),
        }

        Ok(())
    }

    fn output(&self) -> Option<&dyn Any> {
        self.output.as_ref().map(|x| x as &dyn Any)
    }
        
    }

    impl<A: 'static, Message, Renderer: iced_native::Renderer> InteractiveLayer<Message, Renderer> for InputFile<A> {}

    impl<A: 'static, Message, Renderer: iced_native::Renderer> Widget<Message, Renderer> for InputFile<A> {
        default fn width(&self) -> Length {
        Length::Shrink
    }

        default fn height(&self) -> Length {
        Length::Shrink
    }

        default fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> Node {
        todo!()
    }

        default fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) -> Renderer::Output {
        todo!()
    }

        default fn hash_layout(&self, state: &mut iced_native::Hasher) {
        todo!()
    }
    }


    // https://github.com/hecrj/iced/blob/master/native/src/widget/image.rs 
    impl<Message, Renderer: iced_native::Renderer> Widget<Message, Renderer> for InputFile<RgbaImage> {
        fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> Node {
        Node::new(Size::new(
            self.width().unwrap_or(0) as f32, self.height().unwrap_or(0) as f32))
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
        // Not too sure about this. See https://github.com/hecrj/iced/issues/977
        struct Marker; 
        std::any::TypeId::of::<Marker>().hash(state); 
        
        self.width().unwrap_or(0).hash(state);
        self.height().unwrap_or(0).hash(state);
    }
    }

    // impl<A, Message, Renderer: iced_native::Renderer> Widget<Message, Renderer> for InputFile<A> 
    // where InputFile<A>: Image {
    //     fn width(&self) -> Length {
    //     todo!()
    // }

    //     fn height(&self) -> Length {
    //     todo!()
    // }

    //     fn layout(
    //     &self,
    //     renderer: &Renderer,
    //     limits: &iced_native::layout::Limits,
    // ) -> iced_native::layout::Node {
    //     todo!()
    // }

    //     fn draw(
    //     &self,
    //     renderer: &mut Renderer,
    //     defaults: &Renderer::Defaults,
    //     layout: iced_native::Layout<'_>,
    //     cursor_position: iced::Point,
    //     viewport: &iced::Rectangle,
    // ) -> Renderer::Output {
    //     todo!()
    // }

    //     fn hash_layout(&self, state: &mut iced_native::Hasher) {
    //     todo!()
    // }
    // }

    // impl<Message, Renderer: iced_native::Renderer> Widget<Message, Renderer> for InputFile<RgbaImage> {
    //     fn width(&self) -> Length {
    //     Length::Shrink
    // }

    //     fn height(&self) -> Length {
    //     Length::Shrink
    // }

    //     fn layout(
    //     &self,
    //     renderer: &Renderer,
    //     limits: &iced_native::layout::Limits,
    // ) -> Node {
    //     Node::new(self.)
    // }

    //     fn draw(
    //     &self,
    //     renderer: &mut Renderer,
    //     defaults: &Renderer::Defaults,
    //     layout: iced_native::Layout<'_>,
    //     cursor_position: iced::Point,
    //     viewport: &iced::Rectangle,
    // ) -> Renderer::Output {
    //     todo!()
    // }

    //     fn hash_layout(&self, state: &mut iced_native::Hasher) {
    //     todo!()
    // }
    // } 



    // #############
    // #############
    // # Threshold #
    // #############    
    // #############
    pub struct Threshold<A, B, T> {
        threshold: T,
        ordering: std::cmp::Ordering,
        operation: fn(&Self, input: &A) -> B,
        output: Option<B>,
    }

    impl Threshold<GrayImage, element::BinImage, u8> {
        pub fn new(threshold: u8, ordering: std::cmp::Ordering) -> Self {
            Self {
                threshold,
                ordering,
                operation: Self::compute,
                output: None,
            }

        }

        pub fn compute(&self, input: &GrayImage) -> element::BinImage {
            let data = input.pixels().map(|pixel| pixel.0[0].cmp(&self.threshold) == self.ordering).collect();
            element::BinImage::new(input.width(), input.height(), data)
        }
    }

    

    impl<A: 'static, B: 'static, T> Layer for Threshold<A, B, T> {

        fn compute(&self, input: &[Option<&dyn Any>]) -> Result<(Option<Box<dyn Any>>, Option<Box<dyn Any>>)> {
            let input = input[0]; // Threshold only expects input from a single source layer
            let input = input.context("Empty input")?;
            let input = input.downcast_ref::<A>().context(format!(
                "Casting failed. Expected input of type {:#?}",
                any::type_name::<A>()
            ))?;
    
            let output = Some(Box::new((self.operation)(self, input)) as Box::<dyn Any>);
            let state_updates = None;
            Ok((output, state_updates))
        }

        fn update(&mut self, output: Option<Box<dyn Any>>, state_updates: Option<Box<dyn Any>>) -> Result<()> {
            self.output = output.map(|content|
                content.downcast::<B>().map(|inner_content| *inner_content)
            ).transpose().map_err(|_| anyhow!("Casting failed. Expected input of type {:#?}",
            any::type_name::<B>()))?;
    
            match state_updates {
                Some(_) => todo!(),
                None => (),
            }
    
            Ok(())
        }
    
        fn output(&self) -> Option<&dyn Any> {
            self.output.as_ref().map(|x| x as &dyn Any)
        }        
    }

    impl<A: 'static, B: 'static, T, Message, Renderer: iced_native::Renderer> Widget<Message, Renderer> for Threshold<A, B, T> {
        fn width(&self) -> Length {
        todo!()
    }

        fn height(&self) -> Length {
        todo!()
    }

        fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> Node {
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
