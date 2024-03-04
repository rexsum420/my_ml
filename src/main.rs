pub mod lib;

use lib::network::Network;
use lib::activation::SIGMOID;

fn main() {
    let inputs = vec![
        vec![0.0, 0.0],
        vec![1.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 1.0],
    ];
    let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];

    let mut network = Network::new(vec![2, 4, 3, 1], SIGMOID, 0.5);

    println!("\r\nUntrained:");
    println!("0 and 0: {:?}", network.forward(vec![0.0, 0.0]));
    println!("1 and 0: {:?}", network.forward(vec![1.0, 0.0]));
    println!("0 and 1: {:?}", network.forward(vec![0.0, 1.0]));
    println!("1 and 1: {:?}", network.forward(vec![1.0, 1.0]));

    network.train(inputs, targets, 100000);

    println!("\r\nTrained:");
    println!("0 and 0: {:?}", network.forward(vec![0.0, 0.0]));
    println!("1 and 0: {:?}", network.forward(vec![1.0, 0.0]));
    println!("0 and 1: {:?}", network.forward(vec![0.0, 1.0]));
    println!("1 and 1: {:?}", network.forward(vec![1.0, 1.0]));
}
