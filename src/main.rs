mod quantum_gate;
mod quantum_algorithm;
mod visualization;
mod quantum_state;
mod quantum_circuit;

use crate::quantum_gate::hadamard_single;
use crate::quantum_algorithm::{shors_algorithm, grovers_algorithm};
use crate::visualization::plot_quantum_state;
use crate::quantum_state::{initialize_zero_state, measure_state};
use crate::quantum_circuit::QuantumCircuit;

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

    // Initialize quantum state
    let zero_state = initialize_zero_state(2);
    println!("Zero state: {:?}", zero_state);

    // Measure quantum state
    let measurement = measure_state(&zero_state);
    println!("Measurement result: {}", measurement);

    // Create and simulate a quantum circuit
    let mut circuit = QuantumCircuit::new(2);
    circuit.add_gate(hadamard_single(), vec![0]);
    let final_state = circuit.simulate();
    println!("Final state after circuit simulation: {:?}", final_state);
}