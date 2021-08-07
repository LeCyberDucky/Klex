use anyhow::{Context, Result};
use iced::{Application, Settings};
use image::{self, GrayImage};
use petgraph::graph::NodeIndex;

use klex::{
    element::{BinaryImage, RgbaImage},
    layer::{
        interactive,
        primitive::{Convert, InputFile, Threshold},
    },
    layer_graph::InteractiveLayerGraph,
};

fn main() -> Result<()> {
    let mut layer_graph = InteractiveLayerGraph::new();
    let layer = Box::new(InputFile::<RgbaImage>::new_interactive("Tulips.jpg".into()));
    layer_graph.add_layer(layer, vec![]);
    // let layer = Box::new(InputFile::<RgbaImage>::new("Tulips.jpg".into()));
    // layer_graph.add_layer(layer, vec![]);
    // let layer = Box::new(Convert::<RgbaImage, GrayImage>::new());
    // layer_graph.add_layer(layer, vec![0.into()]);
    // let layer = Box::new(Threshold::<GrayImage, BinImage, u8>::new(100, std::cmp::Ordering::Greater));
    // layer_graph.add_layer(layer, vec![1.into()]);
    // let layer = Box::new(Convert::<BinImage, GrayImage>::new());
    // layer_graph.add_layer(layer, vec![2.into()]);
    // layer_graph.compute_layer(0.into())?;
    // layer_graph.compute_layer(1.into())?;
    // layer_graph.compute_layer(2.into())?;
    // layer_graph.compute_layer(3.into())?;

    // // println!("The final layer is some: {}", layer_graph.layers[NodeIndex::new(3)].ou.is_some());

    // if let Some(output) = layer_graph.layers[NodeIndex::new(3)].output() {
    //     let image = output.downcast_ref::<GrayImage>().context("Can't cast to image :|")?;
    //     image.save("GrayTulips.jpg")?;
    // }

    klex::ui::UI::run(Settings::default());

    Ok(())
}
