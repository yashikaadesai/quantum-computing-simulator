use crate::quantum_gate::{hadamard, pauli_x, cnot};
use ndarray::prelude::*;

pub fn shors_algorithm(n: u32) -> Option<u32> {
    // Simplified version of Shor's algorithm for demonstration purposes
    Some(n / 2)
}

pub fn grovers_algorithm(n: usize, target: usize) -> Option<usize> {
    let mut state = Array1::from_elem(n, 1.0 / (n as f64).sqrt());
    let hadamard_gate = hadamard();

    for _ in 0..((n as f64).sqrt() as usize) {
        // Apply Grover's iteration
        state = hadamard_gate.dot(&state);
        // Apply oracle for the target state
        state[target] = -state[target];
        state = hadamard_gate.dot(&state);
    }

    state.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).map(|(i, _)| i)
}