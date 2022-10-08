use mvkde::bandwidth::Bandwidth;
use mvkde::kernel::Kernel;
use mvkde::pdf;

use plotly::common::Mode;
use plotly::{Plot, Scatter};

fn main() {
    let samples: Vec<f64> = vec![
        0.9, 1.0, 1.1, 4.8, 4.9, 5.0, 5.1, 5.2, 8.25, 8.5, 8.75, 9.0, 9.25, 9.5, 9.75,
    ];
    let grid: Vec<f64> = (0..101).into_iter().map(|x| x as f64 * 0.1).collect();
    let kernel = Kernel::Epanechnikov;
    let bandwidth = Bandwidth::Silverman;
    let samples = samples.as_slice();
    let grid = grid.as_slice();
    let res = pdf(samples, grid, kernel, bandwidth);

    let trace1 = Scatter::new(samples.to_vec(), vec![0.0; samples.len()])
        .mode(Mode::Markers)
        .name("Data");
    let trace2 = Scatter::new(grid.to_vec(), res.to_vec())
        .mode(Mode::Lines)
        .name("PDF");
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.show();
}
