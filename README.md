# bullet-rs

[Bullet3](https://github.com/bulletphysics/bullet3) high level Rust interface. Inspired by [bulletrs](https://github.com/not-fl3/bulletrs/).

## Building

Install [rustup](https://rustup.rs/).

Clone and compile project:

```bash
git clone https://github.com/HaronK/bullet-rs.git
cd ./bullet-rs
git submodule update --init --recursive
cargo build --release
```

See also [bullet-sys](https://github.com/HaronK/bullet-sys) for additional required dependencies.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
