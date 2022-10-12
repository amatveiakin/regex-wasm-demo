## WASM regex size demo

This demo shows how `regex` crate contributes to WASM binary size.

### How-to

1. Go to `Cargo.toml` and select `regex` features
2. Build: `wasm-pack build`
3. Check out `pkg/regex_wasm_demo_bg.wasm`

### Results

For me the results with `regex` version 1.6.0 were as follows:

- Baseline (no regex): 1 KiB
- Limited Unicode support, no optimizations: 214 KiB
- Limited Unicode support, full optimizations: 338 KiB
- Full Unicode support, no optimizations: 500 KiB
- Full Unicode support, full optimizations (default features): 623 KiB


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.