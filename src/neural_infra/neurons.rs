use std::rc::Rc;
use std::f32::consts::E;

pub trait BaseNeuron{
    fn calculate_activation(&mut self) -> f32;
}


#[derive(Clone)]
pub struct InputNode{
    pub value: f32,
    pub to_relations: Vec<Rc<Relation>>
}

impl BaseNeuron for InputNode{
    fn calculate_activation(&mut self) -> f32 {
       self.value
    }
}

fn calculate_activation_of_relations(relations: &Vec<Rc<Relation>>, bias: f32) -> f32{
    let mut value: f32 = 0.0;

    for relation_rc in relations{
        let relation: &Relation = relation_rc.as_ref();
        let from_node = relation.from.as_ref();
        value += from_node.calculate_activation() * relation.weight;
    };
    value += bias;
    value = sigmoid(value);
    return value;
}

pub struct OutputNeuron{
    pub label: String,
    pub bias: f32,
    pub from_relations: Vec<Rc<Relation>>
}

impl OutputNeuron {
    pub fn calculate_cost(&self, expected_value: f32) -> f32 {
        // Basic distance calculation, without square root to save cpu cost
        return (self.calculate_activation() - expected_value).powi(2)
    }
}

impl BaseNeuron for OutputNeuron{
    fn calculate_activation(&mut self) -> f32 {
        calculate_activation_of_relations(&self.from_relations, self.bias)
    }
}

pub struct Neuron{
    pub bias: f32,
    pub from_relations: Vec<Rc<Relation>>,
    pub activation: Option<f32>
}

// TODO: Understand this better...
fn sigmoid(x: f32) -> f32 {
    let myE = E;
    1.0 / (1.0 + myE.powf(-x))
}

fn transfer_derivative(output: f32) -> f32 {
    output  * (1.0 - output)
}



impl BaseNeuron for Neuron{
    fn calculate_activation(&mut self) -> f32{
        match self.activation{
            Some(value) => value,
            None => {
                let activation = calculate_activation_of_relations(&self.from_relations, self.bias);
                self.activation = Some(activation);
                return activation
            }
        }
    }
}

pub struct Relation{
    from: Rc<dyn BaseNeuron>,
    to: Rc<dyn BaseNeuron>,
    weight: f32,
}
