use kernel_density_estimation::prelude::*;

use plotly::common::Mode;
use plotly::{Plot, Scatter};

fn main() {
    // Create a distribution.
    let observations: Vec<f32> = vec![1.0, 1.2, 1.4, 1.6, 3.0, 3.4, 3.8];

    // Observations are plotted as points along the X axis.
    let x1 = observations.clone();
    let y1 = vec![0.0; observations.len()];

    // Each KDE uses a bandwidth of 1.
    let bandwidth = Box::new(|_: &[f32]| 1.0);

    // Initialize a KDE for each kernel type.
    let kde1 = KernelDensityEstimator::new(observations.clone(), &bandwidth, Epanechnikov);
    let kde2 = KernelDensityEstimator::new(observations.clone(), &bandwidth, Normal);
    let kde3 = KernelDensityEstimator::new(observations.clone(), &bandwidth, Uniform);
    let kde4 = KernelDensityEstimator::new(observations.clone(), &bandwidth, Triangular);

    // Create a grid of points to evaluate each KDE on.
    let dataset: Vec<f32> = (0..101)
        .into_iter()
        .map(|x| x as f32 * (5. / 100.))
        .collect();

    // Evaluate PDFs.
    let x2 = dataset.clone();
    let y2 = kde1.pdf(&dataset);
    let x3 = dataset.clone();
    let y3 = kde2.pdf(&dataset);
    let x4 = dataset.clone();
    let y4 = kde3.pdf(&dataset);
    let x5 = dataset.clone();
    let y5 = kde4.pdf(&dataset);

    // Plot the observations and each of the PDFs.
    let trace1 = Scatter::new(x1, y1).mode(Mode::Markers).name("Data");
    let trace2 = Scatter::new(x2, y2).mode(Mode::Lines).name("Epanechnikov");
    let trace3 = Scatter::new(x3, y3).mode(Mode::Lines).name("Normal");
    let trace4 = Scatter::new(x4, y4).mode(Mode::Lines).name("Uniform");
    let trace5 = Scatter::new(x4, y4).mode(Mode::Lines).name("Triangular");

    // Render the plot.
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.add_trace(trace5);
    plot.show();
}
