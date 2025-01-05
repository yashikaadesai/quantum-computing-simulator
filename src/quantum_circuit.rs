use ndarray::prelude::*;
use crate::quantum_gate::{hadamard, pauli_x, cnot};

pub struct QuantumCircuit {
    num_qubits: usize,
    gates: Vec<(Array2<f64>, Vec<usize>)>,
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            gates: Vec::new(),
        }
    }

    pub fn add_gate(&mut self, gate: Array2<f64>, qubits: Vec<usize>) {
        self.gates.push((gate, qubits));
    }

    pub fn simulate(&self) -> Array1<f64> {
        let mut state = Array1::from_elem(1 << self.num_qubits, 1.0 / (1 << self.num_qubits) as f64);
        
        for (gate, qubits) in &self.gates {
            // Apply the gate to the state vector (left as an exercise)
        }
        
        state
    }
}