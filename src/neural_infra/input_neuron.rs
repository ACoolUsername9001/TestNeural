use std::rc::Rc;

use super::{relation::Relation, neuron::BaseNeuron};

#[derive(Clone)]
pub struct InputNode {
    pub value: f32,
    pub to_relations: Vec<Rc<Relation>>,
}

impl InputNode {
    fn calculate_error(&self) -> f32 {
        0.0
    }
}

impl BaseNeuron for InputNode {
    fn calculate_activation(&self) -> f32 {
        self.value
    }
}