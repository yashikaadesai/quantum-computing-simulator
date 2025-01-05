mod quantum_gate;
mod quantum_algorithm;
mod visualization;
mod utils;

use quantum_algorithm::{shors_algorithm, grovers_algorithm};
use visualization::plot_quantum_state;

fn main() {
    // Example usage of Shor's algorithm
    let shors_result = shors_algorithm(15);
    println!("Shor's algorithm result: {:?}", shors_result);

    // Example usage of Grover's algorithm
    let grovers_result = grovers_algorithm(4, 2);
    println!("Grover's algorithm result: {:?}", grovers_result);

    // Example quantum state visualization
    let state = vec![0.5, 0.5, 0.5, 0.5];
    plot_quantum_state(&state);
}