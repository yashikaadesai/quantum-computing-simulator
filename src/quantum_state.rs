use ndarray::prelude::*;
use rand::Rng;

pub fn initialize_zero_state(n: usize) -> Array1<f64> {
    let mut state = Array1::zeros(1 << n);
    state[0] = 1.0;
    state
}

pub fn measure_state(state: &Array1<f64>) -> usize {
    let mut rng = rand::thread_rng();
    let mut cumulative_prob = 0.0;
    let random_value: f64 = rng.gen();

    for (index, amplitude) in state.iter().enumerate() {
        cumulative_prob += amplitude.abs().powi(2);
        if random_value < cumulative_prob {
            return index;
        }
    }

    state.len() - 1
}