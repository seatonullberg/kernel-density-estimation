# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2024-03-22

### Added 

- Many new kernel functions added by @ndcroos in #4 resolves #1.

### Fixed

- Fix potential API incompatibility with `f64` feature (#3).

### Removed

- `f64` feature.

## [0.1.0] - 2022-10-16
Initial release with support for univariate distributions.

Bandwidth Selection
* Scott's Rule
* Silverman's Rule

Kernel Functions
* Epanechnikov
* Normal (Gaussian)
* Uniform