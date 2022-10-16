# kernel-density-estimation

Kernel Density Estimation in Rust

__Note:__ Currently only univariate distributions are supported but multivariate is a goal in the future!

## Examples

__[univariate](examples/univariate.rs)__ - This example showcases the core `pdf`, `cdf`, and `sample` functionalities for a univariate distribution.
```
cargo run --example univariate
```
![Univariate Distribution](assets/univariate.png)

__[kernel](examples/kernel.rs)__ - This example showcases each of the available kernel functions.
```
cargo run --example kernel
```
![Kernel Functions](assets/kernel.png)

## Roadmap

Refer to the [milestone issues](https://github.com/seatonullberg/kernel-density-estimation/issues) to see the direction the project is headed in future releases or [CHANGELOG.md](./CHANGELOG.md) to see the changes between each release.

## License

Distributed under the MIT License. See [LICENSE](./LICENSE) for more information.