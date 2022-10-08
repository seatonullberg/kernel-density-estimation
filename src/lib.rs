pub mod bandwidth;
mod internal;
pub mod kernel;

pub fn pdf(
    samples: &[f64],
    grid: &[f64],
    kernel: kernel::Kernel,
    bandwidth: bandwidth::Bandwidth,
) -> Vec<f64> {
    let h = bandwidth.bandwidth(samples);
    let n = samples.len() as f64;
    let prefactor = 1.0 / (n * h);
    let mut arg: f64;
    let mut pdf: Vec<f64> = vec![0.0; grid.len()];
    for x in samples.iter() {
        for (i, xi) in grid.iter().enumerate() {
            arg = (x - xi) / h;
            pdf[i] += kernel.eval(arg);
        }
    }
    pdf.iter().map(|x| x * prefactor).collect()
}
