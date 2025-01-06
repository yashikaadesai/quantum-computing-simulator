use crate::quantum_gate::{hadamard, pauli_x, cnot};
use ndarray::prelude::*;

pub fn shors_algorithm(n: u32) -> Option<u32> {
    Some(n / 2)
}

pub fn grovers_algorithm(n: usize, target: usize) -> Option<usize> {
    let mut state = Array1::from_elem(n, 1.0 / (n as f64).sqrt());
    let hadamard_gate = hadamard((n as f64).log2() as usize);

    // Number of iterations (sqrt(n) is optimal)
    let iterations = (n as f64).sqrt() as usize;

    for _ in 0..iterations {
        // Apply Hadamard gates
        state = hadamard_gate.dot(&state);
        state[target] = -state[target];
        state = hadamard_gate.dot(&state);
        let mean_amplitude = state.sum() / (n as f64);
        state = state.mapv(|amplitude| 2.0 * mean_amplitude - amplitude);
    }

    // Find the index with the highest amplitude
    state.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).map(|(i, _)| i)
}