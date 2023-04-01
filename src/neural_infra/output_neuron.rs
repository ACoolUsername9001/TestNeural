use std::rc::Rc;

use super::{math, relation::Relation, neuron::BaseNeuron, common};

pub struct OutputNeuron {
    pub label: String,
    pub bias: f32,
    pub from_relations: Vec<Rc<Relation>>,
}

impl OutputNeuron {
    pub fn calculate_cost(&mut self, expected_value: f32) -> f32 {
        // Basic distance calculation, without square root to save cpu cost
        return (self.calculate_activation() - expected_value).powi(2);
    }

    pub fn calculate_error(&mut self, expected_value: f32) -> f32 {
        math::calculate_output_neuron_error(self.calculate_activation(), expected_value)
    }
}

impl BaseNeuron for OutputNeuron {
    fn calculate_activation(&self) -> f32 {
        common::calculate_activation_of_relations(&self.from_relations, self.bias)
    }
}