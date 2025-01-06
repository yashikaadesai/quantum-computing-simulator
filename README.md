# Quantum Computing Simulator in Rust

## Overview

This project is a quantum computing simulation framework written in Rust. While I was learning Rust, I worked on this project to apply my new skills. It is rough, but it provides functionalities to simulate quantum gates, implement common quantum algorithms, and visualize quantum states and operations. The framework is structured to be modular and extensible, making it a great foundation for experimenting with quantum computing concepts.

## Features

- **Quantum Gate Simulation**: Implementations of common quantum gates such as the Hadamard gate, Pauli-X gate, and CNOT gate.
- **Quantum Algorithms**: Simplified versions of Shor's algorithm and Grover's algorithm for demonstration purposes.
- **Quantum Circuit Simulation**: A structure to represent and simulate quantum circuits.
- **Visualization Tools**: Visualization of quantum states using bar graphs to illustrate amplitude distributions.

## Project Structure

- `src/quantum_gate.rs`: Contains implementations of quantum gates.
- `src/quantum_algorithm.rs`: Contains implementations of quantum algorithms.
- `src/quantum_circuit.rs`: Defines the structure and simulation of quantum circuits.
- `src/quantum_state.rs`: Functions for initializing and measuring quantum states.
- `src/visualization.rs`: Functions for visualizing quantum states.
- `src/main.rs`: The entry point of the application, demonstrating the usage of the framework.

## Getting Started

### Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- Cargo (comes with Rust installation)

### Running the Project

1. Clone the repository:
   ```sh
   git clone https://github.com/yashikaadesai/quantum-computing-simulator.git
   cd quantum-computing-simulator
   cargo build
   cargo run

### Acknowledgement

- ndarray: For numerical arrays.
- plotters: For creating visualizations.
- rand: For random number generation.

## Example Usage

### Simulating a Quantum Circuit

Here's a step-by-step example of how to create and simulate a simple quantum circuit using the framework:

1. **Add a Hadamard Gate to a Single Qubit**

   This example demonstrates how to initialize a quantum circuit with 2 qubits, add a Hadamard gate to the first qubit, and simulate the circuit.

   ```rust
   use quantum_computing_simulator::quantum_gate::hadamard_single;
   use quantum_computing_simulator::quantum_circuit::QuantumCircuit;

   fn main() {
       // Create a new quantum circuit with 2 qubits
       let mut circuit = QuantumCircuit::new(2);

       // Add a Hadamard gate to the first qubit
       circuit.add_gate(hadamard_single(), vec![0]);

       // Simulate the circuit
       let final_state = circuit.simulate();

       // Print the final state
       println!("Final state after circuit simulation: {:?}", final_state);
   }
