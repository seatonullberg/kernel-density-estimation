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
    let kde5 = KernelDensityEstimator::new(observations.clone(), &bandwidth, Quartic);
    let kde6 = KernelDensityEstimator::new(observations.clone(), &bandwidth, Triweight);
    let kde7 = KernelDensityEstimator::new(observations.clone(), &bandwidth, Tricube);
    let kde8 = KernelDensityEstimator::new(observations.clone(), &bandwidth, Cosine);
    let kde9 = KernelDensityEstimator::new(observations.clone(), &bandwidth, Logistic);
    let kde10 = KernelDensityEstimator::new(observations.clone(), &bandwidth, Sigmoid);
    //let kde11 = KernelDensityEstimator::new(observations.clone(), &bandwidth, Silverman);

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
    let x6 = dataset.clone();
    let y6 = kde5.pdf(&dataset);
    let x7 = dataset.clone();
    let y7 = kde6.pdf(&dataset);
    let x8 = dataset.clone();
    let y8 = kde7.pdf(&dataset);
    let x9 = dataset.clone();
    let y9 = kde8.pdf(&dataset);

    let x10 = dataset.clone();
    let y10 = kde9.pdf(&dataset);
    let x11 = dataset.clone();
    let y11 = kde10.pdf(&dataset);
    //let x12 = dataset.clone();
    //let y12 = kde11.pdf(&dataset);

    // Plot the observations and each of the PDFs.
    let trace1 = Scatter::new(x1, y1).mode(Mode::Markers).name("Data");
    let trace2 = Scatter::new(x2, y2).mode(Mode::Lines).name("Epanechnikov");
    let trace3 = Scatter::new(x3, y3).mode(Mode::Lines).name("Normal");
    let trace4 = Scatter::new(x4, y4).mode(Mode::Lines).name("Uniform");
    let trace5 = Scatter::new(x5, y5).mode(Mode::Lines).name("Triangular");
    let trace6 = Scatter::new(x6, y6).mode(Mode::Lines).name("Quartic");
    let trace7 = Scatter::new(x7, y7).mode(Mode::Lines).name("Triweight");
    let trace8 = Scatter::new(x8, y8).mode(Mode::Lines).name("Tricube");
    let trace9 = Scatter::new(x9, y9).mode(Mode::Lines).name("Cosine");
    let trace10 = Scatter::new(x10, y10).mode(Mode::Lines).name("Logistic");
    let trace11 = Scatter::new(x11, y11).mode(Mode::Lines).name("Sigmoid");
    //let trace12 = Scatter::new(x11, y11).mode(Mode::Lines).name("Silverman");

    // Render the plot.
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.add_trace(trace5);
    plot.add_trace(trace6);
    plot.add_trace(trace7);
    plot.add_trace(trace8);
    plot.add_trace(trace9);
    plot.add_trace(trace10);
    plot.add_trace(trace11);
    //plot.add_trace(trace12);
    plot.show();
}
