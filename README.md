# Polyhedral parallel mesher

[![Build Status](https://github.com/baldraven/blue_noise/actions/workflows/build.yml/badge.svg)](https://github.com/baldraven/blue_noise/actions/workflows/build.yml/)
[![Rust Tests](https://github.com/baldraven/blue_noise/actions/workflows/rust-test.yml/badge.svg)](https://github.com/baldraven/blue_noise/actions/workflows/rust-test.yml/)

Polyhedral parallel mesher is an attempt at making a polyhedral and parallel mesher, in 2 steps :
- Massively generating points
- Forming polyhedral cells from those points 

It is planned to use [Honeycomb](https://github.com/LIHPC-Computational-Geometry/honeycomb) for its mesh structure.
It currently only supports 2D.

## Quickstart

You can build and run the project with `Cargo` after cloning the repository:
```
$ cargo run -- --help
```
The help flag will show you all the configuration options.

Default settings will generate Poisson-disk distributed points, in a 10\*10 box, with a minimal distance of 1 between points.
It will then generate a Voronoï diagram using the Jump Flooding Algorithm (with `wgpu`), with resolution 512\*512 pixels, and display the result on a Javascript visualisation.

<p align="center">
  <img src="https://i.imgur.com/KG1w3Dw.png" />
</p>
https://imgur.com/01Shudbhttps://imgur.com/undefinedhttps://imgur.com/KG1w3Dw
## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your preference.

The [SPDX](https://spdx.dev) license identifier for this project is `MIT OR Apache-2.0`.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as 
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
