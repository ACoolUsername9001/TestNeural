
mod training_data_mnist;
mod neural_infra;
use neural_infra::Network;

fn main() {
    Network::build(15, 4, 10, &["a", "b", "c"]);
}
