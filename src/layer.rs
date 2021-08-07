use std::any::{self, Any};

use anyhow::{anyhow, Context, Result};
use iced_graphics::{Backend, Renderer};
use iced_native::Widget;

pub trait Layer {
    fn compute(
        &self,
        input: &[Option<&dyn Any>],
    ) -> Result<(Option<Box<dyn Any>>, Option<Box<dyn Any>>)>;

    fn update(
        &mut self,
        output: Option<Box<dyn Any>>,
        state_updates: Option<Box<dyn Any>>,
    ) -> Result<()>;

    fn output(&self) -> Option<&dyn Any>;
}

pub trait InteractiveLayer<Message, RenderBackend: Backend>:
    Layer + Widget<Message, Renderer<RenderBackend>>
{
}

pub mod primitive {
    use super::interactive::InterLayer;
    use super::*;

    use ndarray::array;

    use crate::element::{BinaryImage, GrayAlphaImage, GrayImage, RgbaImage};

    pub struct Convert<A, B> {
        operation: fn(&A) -> Result<B>,
        output: Option<B>,
    }

    impl Convert<RgbaImage, GrayAlphaImage> {
        pub fn new() -> Self {
            Self {
                operation: Self::compute,
                output: None,
            }
        }

        pub fn compute(input: &RgbaImage) -> Result<GrayAlphaImage> {
            // Colourimetric conversion to grayscale - Linear luminance
            // https://en.wikipedia.org/wiki/Grayscale#Converting_colour_to_greyscale

            let data = input.data().map(|pixel| {
                let normed_colors = pixel.data().map(|&c| c as f64 / 255.0);
                let linear_colors = normed_colors.map(|&c| {
                    if c <= 0.04045 {
                        c / 12.92
                    } else {
                        ((c + 0.55) / 1.055).powf(2.4)
                    }
                });

                let linear_luminance = 0.2126 * linear_colors[0]
                    + 0.7152 * linear_colors[1]
                    + 0.0722 * linear_colors[2];
                let linear_luminance = (255.0 * linear_luminance).round() as u8;

                array![linear_luminance, pixel.data()[3]]
            });

            Ok(GrayAlphaImage::new(data, input.width(), input.height()))
        }
    }

    impl Default for Convert<RgbaImage, GrayAlphaImage> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Convert<BinaryImage, GrayImage> {
        pub fn new() -> Self {
            Self {
                operation: Self::compute,
                output: None,
            }
        }

        pub fn compute(input: &BinaryImage) -> Result<GrayImage> {
            let data = input
                .data()
                .map(|&pixel| if pixel { u8::MAX } else { u8::MIN });
            Ok(GrayImage::new(data, input.width(), input.height()))
        }
    }

    impl Default for Convert<BinaryImage, GrayImage> {
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

            let output = Some(Box::new((self.operation)(input)?) as Box<dyn Any>);
            let state_updates = None;
            Ok((output, state_updates))
        }

        fn update(
            &mut self,
            output: Option<Box<dyn Any>>,
            state_updates: Option<Box<dyn Any>>,
        ) -> Result<()> {
            self.output = output
                .map(|content| content.downcast::<B>().map(|inner_content| *inner_content))
                .transpose()
                .map_err(|_| {
                    anyhow!(
                        "Casting failed. Expected input of type {:#?}",
                        any::type_name::<B>()
                    )
                })?;

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

    pub struct Convolve {}

    // #############
    // #############
    // # InputFile #
    // #############
    // #############
    pub struct InputFile<A> {
        file_path: std::path::PathBuf,
        operation: fn(&Self) -> Result<A>,
        output: Option<A>,
    }

    impl InputFile<RgbaImage> {
        pub fn new(file_path: std::path::PathBuf) -> Self {
            Self {
                file_path,
                operation: Self::compute,
                output: None,
            }
        }

        pub fn new_interactive(file_path: std::path::PathBuf) -> InterLayer<Self, RgbaImage> {
            InterLayer::new(Self::new(file_path))
        }

        pub fn compute(&self) -> Result<RgbaImage> {
            // Ok(image::open(&self.file_path)?.into_rgba8())
            RgbaImage::open(&self.file_path)
        }

        // fn width(&self) -> Option<usize> {
        //     // Some(self.output.as_ref()?.dimensions().0 as usize)
        //     Some(self.output.as_ref()?.width())
        // }

        // fn height(&self) -> Option<usize> {
        //     // Some(self.output.as_ref()?.dimensions().1 as usize)
        //     Some(self.output.as_ref()?.height())
        // }
    }

    impl<A: 'static> Layer for InputFile<A> {
        fn compute(
            &self,
            _input: &[Option<&dyn Any>], // This layer does not depend on other layers
        ) -> Result<(Option<Box<dyn Any>>, Option<Box<dyn Any>>)> {
            let output = Some(Box::new((self.operation)(self)?) as Box<dyn Any>);
            let state_updates = None;
            Ok((output, state_updates))
        }

        fn update(
            &mut self,
            output: Option<Box<dyn Any>>,
            state_updates: Option<Box<dyn Any>>,
        ) -> Result<()> {
            self.output = output
                .map(|content| content.downcast::<A>().map(|inner_content| *inner_content))
                .transpose()
                .map_err(|_| {
                    anyhow!(
                        "Casting failed. Expected input of type {:#?}",
                        any::type_name::<A>()
                    )
                })?;

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

    impl Threshold<GrayImage, BinaryImage, u8> {
        pub fn new(threshold: u8, ordering: std::cmp::Ordering) -> Self {
            Self {
                threshold,
                ordering,
                operation: Self::compute,
                output: None,
            }
        }

        pub fn compute(&self, input: &GrayImage) -> BinaryImage {
            let data = input
                .data()
                .map(|pixel| pixel.cmp(&self.threshold) == self.ordering);
            BinaryImage::new(data, input.width(), input.height())
        }
    }

    impl<A: 'static, B: 'static, T> Layer for Threshold<A, B, T> {
        fn compute(
            &self,
            input: &[Option<&dyn Any>],
        ) -> Result<(Option<Box<dyn Any>>, Option<Box<dyn Any>>)> {
            let input = input[0]; // Threshold only expects input from a single source layer
            let input = input.context("Empty input")?;
            let input = input.downcast_ref::<A>().context(format!(
                "Casting failed. Expected input of type {:#?}",
                any::type_name::<A>()
            ))?;

            let output = Some(Box::new((self.operation)(self, input)) as Box<dyn Any>);
            let state_updates = None;
            Ok((output, state_updates))
        }

        fn update(
            &mut self,
            output: Option<Box<dyn Any>>,
            state_updates: Option<Box<dyn Any>>,
        ) -> Result<()> {
            self.output = output
                .map(|content| content.downcast::<B>().map(|inner_content| *inner_content))
                .transpose()
                .map_err(|_| {
                    anyhow!(
                        "Casting failed. Expected input of type {:#?}",
                        any::type_name::<B>()
                    )
                })?;

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

    pub struct TransformAffine<A> {
        operation: fn(&A) -> Result<A>,
    }
}

pub mod interactive {
    use std::hash::Hash;
    use std::marker::PhantomData;

    use iced_graphics::Primitive;

    use crate::element;

    use super::*;

    struct Cache {}

    pub struct InterLayer<A: Layer, T> {
        layer: A,
        // cache: Option<Geometry> // https://docs.rs/iced/0.3.0/iced/widget/canvas/struct.Cache.html https://github.com/hecrj/iced/blob/master/graphics/src/widget/canvas/cache.rs
        cache: Cache,
        width: Option<usize>,
        height: Option<usize>,
        output_type: PhantomData<T>, // Used to group together different layers that have the same output and thus the same interactive behavior. Interactive layers based on layers that input an RGBA image or convert something to an RGBA image shouldn't need different impls, as their interactive behavior should be the same in both cases
    }

    impl<A: Layer, T> InterLayer<A, T> {
        pub fn new(layer: A) -> Self {
            Self {
                layer,
                cache: Cache {},
                width: None,
                height: None,
                output_type: PhantomData,
            }
        }

        fn width(&self) -> Option<usize> {
            self.width
        }

        fn height(&self) -> Option<usize> {
            self.height
        }
    }

    impl<A: Layer, T> Layer for InterLayer<A, T> {
        fn compute(
            &self,
            input: &[Option<&dyn Any>],
        ) -> Result<(Option<Box<dyn Any>>, Option<Box<dyn Any>>)> {
            self.layer.compute(input)
        }

        fn update(
            &mut self,
            output: Option<Box<dyn Any>>,
            state_updates: Option<Box<dyn Any>>,
        ) -> Result<()> {
            self.layer.update(output, state_updates)
        }

        fn output(&self) -> Option<&dyn Any> {
            self.layer.output()
        }
    }

    impl<A: Layer, T, Message, RenderBackend: Backend> InteractiveLayer<Message, RenderBackend>
        for InterLayer<A, T>
    {
    }

    impl<A: Layer, T, Message, RenderBackend: Backend> Widget<Message, Renderer<RenderBackend>>
        for InterLayer<A, T>
    {
        default fn width(&self) -> iced::Length {
            iced::Length::Shrink
        }

        default fn height(&self) -> iced::Length {
            iced::Length::Shrink
        }

        default fn layout(
            &self,
            _renderer: &Renderer<RenderBackend>,
            _limits: &iced_native::layout::Limits,
        ) -> iced_native::layout::Node {
            iced_native::layout::Node::new(iced_native::Size::new(
                self.width().unwrap_or(0) as f32,
                self.height().unwrap_or(0) as f32,
            ))
        }

        default fn draw(
            &self,
            _renderer: &mut Renderer<RenderBackend>,
            _defaults: &<Renderer<RenderBackend> as iced_native::Renderer>::Defaults,
            _layout: iced_native::Layout<'_>,
            _cursor_position: iced::Point,
            _viewport: &iced::Rectangle,
        ) -> <Renderer<RenderBackend> as iced_native::Renderer>::Output {
            todo!()
        }

        default fn hash_layout(&self, _state: &mut iced_native::Hasher) {
            todo!()
        }
    }

    // https://github.com/hecrj/iced/blob/master/native/src/widget/image.rs
    impl<A: Layer, Message, RenderBackend: Backend> Widget<Message, Renderer<RenderBackend>>
        for InterLayer<A, element::RgbaImage>
    {
        fn draw(
            &self,
            _renderer: &mut Renderer<RenderBackend>,
            _defaults: &<Renderer<RenderBackend> as iced_native::Renderer>::Defaults,
            _layout: iced_native::Layout<'_>,
            _cursor_position: iced::Point,
            _viewport: &iced::Rectangle,
        ) -> <Renderer<RenderBackend> as iced_native::Renderer>::Output {
            let mut output = (Primitive::None, iced_native::mouse::Interaction::Idle);

            if let Some(input) = self.output() {
                let input = input.downcast_ref::<element::RgbaImage>();
                if let Some(content) = input {
                    let image_handle = content.handle();
                    output.0 = Primitive::Image {
                        handle: image_handle,
                        bounds: iced_graphics::Rectangle::new(
                            iced_graphics::Point::new(0.0, 0.0),
                            iced_graphics::Size::new(
                                content.width() as f32,
                                content.height() as f32,
                            ),
                        ),
                    };
                }
            }

            output
        }

        fn hash_layout(&self, state: &mut iced_native::Hasher) {
            // Not too sure about this. See https://github.com/hecrj/iced/issues/977
            struct Marker;
            std::any::TypeId::of::<Marker>().hash(state);

            self.width().unwrap_or(0).hash(state);
            self.height().unwrap_or(0).hash(state);
        }
    }
}
