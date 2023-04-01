mod neural_infra;
mod mnist_source;
use neural_infra::Network;

fn main() {
    Network::build(15, 4, 10, &["a", "b", "c"]);
}
