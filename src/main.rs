use anyhow::Result;
use image::{self, GrayImage, RgbaImage};

use klex::{entity::{BinaryImage}, layer::{primitive::{Convert, InputFile, Threshold}}};
use klex::layer_graph::InteractiveLayerGraph;

fn main() -> Result<()> {
    let mut layers = InteractiveLayerGraph::new();
    let layer = Box::new(InputFile::<RgbaImage>::new("Tulips.jpg".into()));
    layers.add_layer(layer, vec![]);
    let layer = Box::new(Convert::<RgbaImage, GrayImage>::new());
    layers.add_layer(layer, vec![0.into()]);
    let layer = Box::new(Threshold::<GrayImage, BinaryImage, u8>::new(100, std::cmp::Ordering::Greater));
    layers.add_layer(layer, vec![1.into()]);
    let layer = Box::new(Convert::<BinaryImage, GrayImage>::new());
    layers.add_layer(layer, vec![2.into()]);
    layers.compute_layer(0.into())?;
    layers.compute_layer(1.into())?;
    layers.compute_layer(2.into())?;
    layers.compute_layer(3.into())?;

    println!("The final layer is some: {}", layers.layer_output[3].is_some());

    if let Some(output) = &layers.layer_output[3] {
        let output = output.downcast_ref::<GrayImage>();
        if let Some(image) = output {
            image.save("GrayTulips.jpg")?;
        }
    }
    Ok(())
}
