use ndarray::prelude::*;

pub struct QuantumCircuit {
    pub num_qubits: usize,
    pub gates: Vec<(Array2<f64>, Vec<usize>)>,
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
        let mut state = Array1::from_elem(1 << self.num_qubits, 0.0);
        state[0] = 1.0; // Initialize to |0...0>

        for (gate, qubits) in &self.gates {
            let full_gate = self.expand_gate(gate, qubits);
            state = full_gate.dot(&state);
        }

        state
    }

    fn expand_gate(&self, gate: &Array2<f64>, qubits: &Vec<usize>) -> Array2<f64> {
        let mut full_gate = Array2::eye(1 << self.num_qubits);

        for (_i, &qubit) in qubits.iter().enumerate() {
            let mut expanded_gate = Array2::eye(1 << self.num_qubits);
            let stride = 1 << (self.num_qubits - qubit - 1);

            for j in 0..stride {
                for k in 0..(1 << qubits.len()) {
                    let row = j + (k / 2) * stride * 2;
                    let col = j + (k % 2) * stride;
                    expanded_gate[[row, col]] = gate[[k / 2, k % 2]];
                }
            }

            full_gate = full_gate.dot(&expanded_gate);
        }

        full_gate
    }
}