# MCC DAQ HAT Library for Raspberry Pi - Rust Bindings

Incomplete Rust bindings for the [MCC DAQ HAT Library for Raspberry Pi](https://github.com/mccdaq/daqhats) v1.5.0.0

The daqhats library must be installed to build and use this library.

Currently only supports the MCC 118.

To cross compile a project that uses this library, you can copy `Dockerfile` and `dev-container.sh` to your project directory and run `./dev-container.sh` to start a container in your current directory, running on aarch64 (emulated if your host isn't aarch64). The container installs the daqhats library and the Rust toolchain.

## Continuous Scan Example

See `examples/mcc118_continuous.rs`
