use std::any::{Any};

use anyhow::{Result};
use petgraph::{graph::NodeIndex, Direction, Graph};

use crate::layer::InteractiveLayer;

pub struct InteractiveLayerGraph {
    pub layers: Graph<Box<dyn InteractiveLayer>, ()>, // Store layers together with their corresponding output
    pub layer_output: Vec<Option<Box<dyn Any>>>,
    selected_layer: NodeIndex,
}

impl InteractiveLayerGraph {
    pub fn new() -> Self {
        Self {
            layers: Graph::new(),
            layer_output: Vec::new(),
            selected_layer: NodeIndex::new(0),
        }
    }

    pub fn add_layer_with_children(
        &mut self,
        layer: Box<dyn InteractiveLayer>,
        parent_nodes: Vec<NodeIndex>,
        child_nodes: Vec<NodeIndex>,
    ) {
        let new_node = self.layers.add_node(layer);
        self.layer_output.push(None);

        for parent in parent_nodes {
            self.layers.add_edge(parent, new_node, ());
        }

        for child in child_nodes {
            self.layers.add_edge(new_node, child, ());
        }
    }

    pub fn add_layer(&mut self, layer: Box<dyn InteractiveLayer>, parent_nodes: Vec<NodeIndex>) {
        self.add_layer_with_children(layer, parent_nodes, vec![])
    }

    pub fn compute_layer(&mut self, layer: NodeIndex) -> Result<()> {
        let input: Vec<&Option<Box<dyn Any>>> = self
            .layers
            .neighbors_directed(layer, Direction::Incoming)
            .map(|neighbor| &self.layer_output[neighbor.index()])
            .collect();

        let mut output = None;
        self.layers[layer].compute(&input, &mut output)?;
        self.layer_output[layer.index()] = output;
        Ok(())
    }
}

impl Default for InteractiveLayerGraph {
    fn default() -> Self {
        Self::new()
    }
}
