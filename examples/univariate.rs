use mvkde::prelude::*;

use plotly::common::Mode;
use plotly::{Plot, Scatter};

fn main() {
    let samples: Vec<f64> = vec![
        0.9, 1.0, 1.1, 4.8, 4.9, 5.0, 5.1, 5.2, 8.25, 8.5, 8.75, 9.0, 9.25, 9.5, 9.75,
    ];
    let x1 = samples.clone();
    let y1 = vec![0.0; samples.len()];

    let kde = KernelDensityEstimator::new(samples);

    let dataset: Vec<f64> = (0..101).into_iter().map(|x| x as f64 * 0.1).collect();
    let x2 = dataset.clone();
    let y2 = kde.pdf(dataset, Scott, Epanechnikov);

    let trace1 = Scatter::new(x1, y1).mode(Mode::Markers).name("Data");
    let trace2 = Scatter::new(x2, y2).mode(Mode::Lines).name("PDF");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.show();
}
