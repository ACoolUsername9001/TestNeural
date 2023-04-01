use std::rc::Rc;

use super::{common};
use super::relation::Relation;



pub trait BaseNeuron {
    fn calculate_activation(&self) -> f32;
}

pub struct Neuron {
    pub bias: f32,
    pub from_relations: Vec<Rc<Relation>>,
    pub activation: Option<f32>,
}

impl BaseNeuron for Neuron {
    fn calculate_activation(&self) -> f32 {
        match self.activation {
            Some(value) => value,
            None => {
                let activation = common::calculate_activation_of_relations(&self.from_relations, self.bias);
                //self.activation = Some(activation);
                return activation;
            }
        }
    }
}


