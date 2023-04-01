use std::rc::Rc;

use super::neuron::BaseNeuron;

pub struct Relation {
    pub(crate) from: Rc<dyn BaseNeuron>,
    to: Rc<dyn BaseNeuron>,
    pub(crate) weight: f32,
}