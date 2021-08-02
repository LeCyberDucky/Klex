use std::any::{Any};

use anyhow::{Result};
use iced_native;
use iced_wgpu;
use petgraph::{graph::NodeIndex, Direction, Graph};

use crate::layer::InteractiveLayer;
use crate::ui;

pub struct InteractiveLayerGraph {
    pub layers: Graph<Box<dyn InteractiveLayer<ui::InternalMessage, iced_wgpu::Renderer>>, ()>, 
    selected_layer: NodeIndex,
}

impl InteractiveLayerGraph {
    pub fn new() -> Self {
        Self {
            layers: Graph::new(),
            selected_layer: NodeIndex::new(0),
        }
    }

    pub fn add_layer_with_children(
        &mut self,
        layer: Box<dyn InteractiveLayer<ui::InternalMessage, iced_wgpu::Renderer>>,
        parent_nodes: Vec<NodeIndex>,
        child_nodes: Vec<NodeIndex>,
    ) {
        let new_node = self.layers.add_node(layer);

        for parent in parent_nodes {
            self.layers.add_edge(parent, new_node, ());
        }

        for child in child_nodes {
            self.layers.add_edge(new_node, child, ());
        }
    }

    pub fn add_layer(&mut self, layer: Box<dyn InteractiveLayer<ui::InternalMessage, iced_wgpu::Renderer>>, parent_nodes: Vec<NodeIndex>) {
        self.add_layer_with_children(layer, parent_nodes, vec![])
    }

    pub fn compute_layer(&mut self, layer: NodeIndex) -> Result<()> {
        let input: Vec<Option<&dyn Any>> = self.layers
        .neighbors_directed(layer, Direction::Incoming)
        .map(|neighbor| self.layers[neighbor].output())
        .collect();

        let (output, state_changes) = self.layers[layer].compute(&input)?;
        self.layers[layer].update(output, state_changes);
        Ok(())
    }
}

impl Default for InteractiveLayerGraph {
    fn default() -> Self {
        Self::new()
    }
}
