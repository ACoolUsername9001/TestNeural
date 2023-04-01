use std::rc::Rc;

use super::{math, relation::Relation};

pub(crate) fn calculate_error(output: f32, from_relations: &Vec<Rc<Relation>>, deltas: &Vec<f32>) -> f32 {
    from_relations
        .iter()
        .zip(deltas)
        .map(|(relation, delta)| math::calculate_non_output_neuron_error(relation.as_ref().weight, *delta))
        .sum::<f32>() * math::transfer_derivative(output)
}

pub(crate) fn calculate_activation_of_relations(from_relations: &Vec<Rc<Relation>>, bias: f32) -> f32 {
    let mut value: f32 = 0.0;

    for relation_rc in from_relations {
        let from_relation: &Relation = relation_rc.as_ref();
        let from_node = from_relation.from.as_ref();
        value += from_node.calculate_activation() * from_relation.weight;
    }
    value += bias;
    value = math::sigmoid(value);
    return value;
}