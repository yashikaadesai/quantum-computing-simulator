use plotters::prelude::*;

pub fn plot_quantum_state(state: &[f64]) {
    let root = BitMapBackend::new("quantum_state.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let max_val = *state.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let min_val = *state.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Quantum State", ("sans-serif", 50).into_font())
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..state.len(), min_val..max_val)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(
            state.iter().enumerate().map(|(i, &val)| {
                Rectangle::new([(i, 0.0), (i + 1, val)], GREEN.filled())
            }),
        )
        .unwrap();

    root.present().unwrap();
}