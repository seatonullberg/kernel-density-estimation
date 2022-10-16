use kernel_density_estimation::prelude::*;

use plotly::color::NamedColor;
use plotly::common::{Marker, Mode};
use plotly::{Histogram, Plot, Scatter};

fn main() {
    let observations: Vec<f32> = vec![
        0.9, 1.0, 1.1, 4.8, 4.9, 5.0, 5.1, 5.2, 8.25, 8.5, 8.75, 9.0, 9.25, 9.5, 9.75,
    ];
    let x1 = observations.clone();
    let y1 = vec![0.0; observations.len()];

    let bandwidth = Scott;
    let kernel = Epanechnikov;
    let kde = KernelDensityEstimator::new(observations, bandwidth, kernel);

    let pdf_dataset: Vec<f32> = (0..101).into_iter().map(|x| x as f32 * 0.1).collect();
    let cdf_dataset = pdf_dataset.clone();
    let sample_dataset = cdf_dataset.clone();

    let x2 = pdf_dataset.clone();
    let y2 = kde.pdf(pdf_dataset.as_slice());

    let x3 = cdf_dataset.clone();
    let y3 = kde.cdf(cdf_dataset.as_slice());

    let x4 = kde.sample(sample_dataset.as_slice(), 10_000);

    let trace1 = Scatter::new(x1, y1)
        .mode(Mode::Markers)
        .marker(Marker::new().color(NamedColor::CornflowerBlue))
        .name("Data");
    let trace2 = Scatter::new(x2, y2)
        .mode(Mode::Lines)
        .marker(Marker::new().color(NamedColor::Black))
        .name("PDF");
    let trace3 = Scatter::new(x3, y3)
        .mode(Mode::Lines)
        .marker(Marker::new().color(NamedColor::YellowGreen))
        .name("CDF");
    let trace4 = Histogram::new(x4)
        .hist_norm(plotly::histogram::HistNorm::ProbabilityDensity)
        .marker(Marker::new().color(NamedColor::Bisque))
        .n_bins_x(100)
        .name("Histogram");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.show();
}
