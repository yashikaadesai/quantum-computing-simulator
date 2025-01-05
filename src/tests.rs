#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hadamard_single() {
        let h = hadamard_single();
        // Add assertions to verify the correctness of the Hadamard gate
    }

    #[test]
    fn test_pauli_x() {
        let x = pauli_x();
        // Add assertions to verify the correctness of the Pauli-X gate
    }

    #[test]
    fn test_initialize_zero_state() {
        let state = initialize_zero_state(2);
        assert_eq!(state[0], 1.0);
        assert_eq!(state[1], 0.0);
        assert_eq!(state[2], 0.0);
        assert_eq!(state[3], 0.0);
    }

    #[test]
    fn test_measure_state() {
        let state = initialize_zero_state(2);
        let result = measure_state(&state);
        assert_eq!(result, 0);
    }
}