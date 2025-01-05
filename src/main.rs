mod quantum_gate;
mod quantum_algorithm;
mod visualization;

use quantum_algorithm::{shors_algorithm, grovers_algorithm};
use visualization::plot_quantum_state;

fn main() {
    // Example usage of Shor's algorithm
    let shors_result = shors_algorithm(15);
    println!("Shor's algorithm result: {:?}", shors_result);

    // Example usage of Grover's algorithm
    let grovers_result = grovers_algorithm(4, 2);
    println!("Grover's algorithm result: {:?}", grovers_result);

    // Example quantum state visualization for Grover's algorithm
    if let Some(target) = grovers_result {
        // Create a state vector with the target state having the highest amplitude
        let mut state = vec![0.5; 4];
        state[target] = 0.9; // Simulate the result of Grover's algorithm
        plot_quantum_state(&state);
    }
}