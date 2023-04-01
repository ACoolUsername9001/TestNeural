pub mod neurons;
use neurons::InputNode;
use neurons::OutputNeuron;
use neurons::Neuron;
use neurons::Relation;
use std::borrow::BorrowMut;
use std::rc::Rc;


pub(crate) struct Network {
    pub input_nodes: Vec<InputNode>,
    pub output_nodes: Vec<OutputNeuron>,
}

impl Network {
    // fn getNodesByLayer(layer_index: u32) -> [Node] {
        
    // }
    pub fn build(
        input_data_length: usize, 
        layer_count: u32, 
        node_count_per_layer: u32,
        output_labels: &[&str]
    ) -> Network{
        let input_layer = vec![InputNode{value: 0.0, to_relations: Vec::new()}; input_data_length];
        let mut output_layer = Vec::new();
        for &label in output_labels.into_iter(){
            output_layer.push(OutputNeuron{from_relations: Vec::new(), label: String::from(label), bias: 0.0});
        }
        let last_layer: Vec<Neuron> = Vec::new();
        for _ in 0..layer_count{
            let mut layer_neurons: Vec<Rc<Neuron>> = Vec::new();
            for _ in 0..node_count_per_layer{
                let current_neuron = Neuron{activation: Option::None, from_relations: Vec::new(), bias: 0.0};
                for mut neuron_rc in last_layer.into_iter(){
                    neuron_rc.from_relations.push(Rc::from(Relation{from: Rc::from(current_neuron), to: Rc::from(neuron_rc), weight: 0.0}));
                };
                layer_neurons.push(Rc::from(current_neuron));
            }
            // last_layer = layer_neurons;            
        };
        Network{input_nodes: input_layer, output_nodes: output_layer}
    }
    
    pub fn train(&self, data: (&[u8], &str)) {

    }

    // fn calculate_cost(&self, inputs: &[&i32], expected_outputs: &[i32]) -> i32 {
    //     let mut c = 0;
    //     let wow = self.output_nodes.into_iter().map(|letter| {c += 1; letter.as_ref().calculate_cost(expected_outputs[c])});
    //     let sum: i32 = wow.sum();
    //     let len: i32 = wow.len().try_into().unwrap();
    //     let average: i32 = sum / len;
    //     return average;
    // }

    fn validate_and_tweak(&self) {
        
    }

    fn test(&self) {

    }
    
    pub fn run(&self, input_data: u32) {
    }

    pub fn save(&self) {
        
    }
    
    // pub fn load() -> Network{

    // }
}