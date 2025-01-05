# Quantum Computing Simulator in Rust

## Overview

This project is a quantum computing simulation framework written in Rust. It provides functionalities to simulate quantum gates, implement common quantum algorithms, and visualize quantum states and operations. The framework is structured to be modular and extensible, making it a great foundation for experimenting with quantum computing concepts.

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

   ```sh
2.   cargo build

   ```sh
3.   cargo run


