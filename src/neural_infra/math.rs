use std::f32::consts::E;

pub(crate) fn sigmoid(x: f32) -> f32 {
    let myE = E;
    1.0 / (1.0 + myE.powf(-x))
}

pub(crate) fn transfer_derivative(output: f32) -> f32 {
    output  * (1.0 - output)
}

pub(crate) fn calculate_output_neuron_error(output: f32, expected: f32) -> f32 {
    (output - expected) * transfer_derivative(output)
}

pub(crate) fn calculate_non_output_neuron_error(to_neuron_weight: f32, to_neuron_error: f32) -> f32 {
    to_neuron_weight  * to_neuron_error
}