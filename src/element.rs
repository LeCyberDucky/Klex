use std::ops::{Deref, DerefMut};

use anyhow::{Context, Result};
use ndarray;

pub struct Line {}

pub struct Point {}

#[derive(Clone)]
pub struct Image<PixelKind> {
    data: ndarray::Array2<PixelKind>,
    width: usize,
    height: usize,
}

impl<PixelKind> Image<PixelKind> {
    pub fn new(data: ndarray::Array2<PixelKind>, width: usize, height: usize) -> Result<Self> {
        let data_shape = (data.len_of(ndarray::Axis(0)), data.len_of(ndarray::Axis(1)));
        (data_shape == (height, width)).then_some(
        Self {
            data,
            width,
            height,
        }).context(format!("Unable to create image. Data has shape ({}, {}). Expected shape ({}, {})", data_shape.0, data_shape.1, height, width))
    }

    // pub fn from_shape_vec(data: Vec<PixelKind>, width: usize, height: usize) -> Result<Self> {
    //     let data = ndarray::Array2::from_shape_vec((height, width), data)?;

    //     Ok(Self::new(data, width, height))
    // }

    pub fn data(&self) -> &ndarray::Array2<PixelKind> {
        &self.data
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

impl<T> Image<pixel::RGBA<T>> {
    pub fn pixels(&self) -> ndarray::iter::Iter<'_, pixel::RGBA<T>, ndarray::Dim<[usize; 2]>> {
        self.data.iter()
    }
}

pub struct BinaryAlphaImage(Image<(bool, u8)>);

pub struct BinaryImage(Image<bool>);
impl Deref for BinaryImage {
    type Target = Image<bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for BinaryImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BinaryImage {
    pub fn new(data: ndarray::Array2<bool>, width: usize, height: usize) -> Result<Self> {
        Ok(Self(Image::new(data, width, height)?))
    }
}

pub struct RgbaImage(Image<pixel::RGBA<u8>>);
impl Deref for RgbaImage {
    type Target = Image<pixel::RGBA<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for RgbaImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl RgbaImage {
    pub fn new(data: ndarray::Array2<pixel::RGBA<u8>>, width: usize, height: usize) -> Result<Self> {
        Ok(Self(Image::new(data, width, height)?))
    }

    pub fn open<P>(file_path: P) -> Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let image = image::open(file_path)?.into_rgba8();

        let data = image
            .pixels()
            .map(|pixel| pixel::RGBA::new(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]))
            .collect();

        let data = ndarray::Array2::from_shape_vec(
            (image.height() as usize, image.width() as usize),
            data,
        )?;

        Ok(Self::new(
            data,
            image.width() as usize,
            image.height() as usize,
        )?)
    }

    pub fn handle(&self) -> iced_native::widget::image::Handle {
        let pixels = self
            .pixels()
            .map(|pixel| [pixel.b(), pixel.g(), pixel.r(), pixel.a()])
            .flatten()
            .collect();
        let image_handle = iced_native::widget::image::Handle::from_pixels(
            self.width() as u32,
            self.height() as u32,
            pixels,
        );
        image_handle
    }
}

pub struct RgbImage(Image<pixel::RGB<u8>>);
impl Deref for RgbImage {
    type Target = Image<pixel::RGB<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for RgbImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct GrayAlphaImage(Image<(u8, u8)>); // Newtype pattern, to be able to distinguish
                                                       // between different types of images that have the same underlying representation
impl Deref for GrayAlphaImage {
    type Target = Image<(u8, u8)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for GrayAlphaImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl GrayAlphaImage {
    pub fn new(data: ndarray::Array2<(u8, u8)>, width: usize, height: usize) -> Result<Self> {
        Ok(Self(Image::new(data, width, height)?))
    }
}

pub struct GrayImage(Image<u8>);
impl Deref for GrayImage {
    type Target = Image<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for GrayImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl GrayImage {
    pub fn new(data: ndarray::Array2<u8>, width: usize, height: usize) -> Result<Self> {
        Ok(Self(Image::new(data, width, height)?))
    }

    // pub fn from_shape_vec(data: Vec<u8>, width: usize, height: usize) -> Result<Self> {
    //     Ok(GrayImage(Image::<u8>::from_shape_vec(data, width, height)?))
    // }
}

mod pixel {
    #[derive(Clone)]
    pub struct RGB<T> {
        data: ndarray::Array1<T>,
    }

    impl<T: Copy> RGB<T> {
        pub fn r(&self) -> T {
            self.data[0]
        }

        pub fn g(&self) -> T {
            self.data[1]
        }

        pub fn b(&self) -> T {
            self.data[2]
        }

        pub fn data(&self) -> &ndarray::Array1<T> {
            &self.data
        }
    }

    #[derive(Clone)]
    pub struct RGBA<T> {
        data: ndarray::Array1<T>,
    }

    impl<T: Copy> RGBA<T> {
        pub fn new(r: T, g: T, b: T, a: T) -> Self {
            Self {
                data: ndarray::array![r, g, b, a],
            }
        }

        pub fn r(&self) -> T {
            self.data[0]
        }

        pub fn g(&self) -> T {
            self.data[1]
        }

        pub fn b(&self) -> T {
            self.data[2]
        }

        pub fn a(&self) -> T {
            self.data[3]
        }

        pub fn data(&self) -> &ndarray::Array1<T> {
            &self.data
        }
    }
}
