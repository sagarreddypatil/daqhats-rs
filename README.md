# MCC DAQ HAT Library for Raspberry Pi - Rust Bindings

Incomplete Rust bindings for the [MCC DAQ HAT Library for Raspberry Pi](https://github.com/mccdaq/daqhats) v1.5.0.0

The daqhats library must be installed to build and use this library.

Currently only supports the MCC 118.

To cross compile a project that uses this library, you can copy `Dockerfile` and `dev-container.sh` to your project directory and run `./dev-container.sh` to start a container in your current directory, running on aarch64 (emulated if your host isn't aarch64). The container installs the daqhats library and the Rust toolchain.

## Continuous Scan Example

```rust
use anyhow;
use daqhats::{hat_list, HatId, ScanOptions, ScanStatus, Mcc118};

fn main() -> anyhow::Result<(), anyhow::Error> {
    let avail_devices = hat_list(HatId::Mcc118);
    if avail_devices.len() == 0 {
        println!("No MCC 118 devices found");
        return Err(anyhow::Error::msg("No MCC 118 devices found"));
    }

    let addr = avail_devices[0].address;

    let channel_mask: u8 = 0b1111;
    let num_channels = channel_mask.count_ones() as u8;
    let samples_per_channel: u32 = 0; // 0 only allowed for continuous scan, auto calculate buffer size

    let mut read_buf: [f64; 8000] = [0.0; 8000]; // 1000 samples * 8 possible channels
    let scan_rate: f64 = 1000.0; // Hz
    let _actual_scan_rate = Mcc118::a_in_scan_actual_rate(num_channels, scan_rate);

    let opts = ScanOptions::CONTINUOUS;

    let dev = Mcc118::open(addr)?;
    dev.a_in_scan_start(channel_mask, samples_per_channel, scan_rate, opts)?;

    println!("Internal data buffer size: {}", dev.a_in_scan_buffer_size()?);
    println!("Starting scan");

    let mut total_samples_read = 0;

    loop {
        // -1 means read all available samples and return immediately
        // timeout of 5.0 seconds is ignored
        let (status, samples_read) = dev.a_in_scan_read(-1, 5.0, &mut read_buf)?;

        if status.contains(ScanStatus::HW_OVERRUN) {
            println!("Hardware overrun detected");
        }

        if status.contains(ScanStatus::BUFFER_OVERRUN) {
            println!("Buffer overrun detected");
        }


        if !status.contains(ScanStatus::RUNNING) {
            break;
        }

        total_samples_read += samples_read;
        println!("Samples read: {}", total_samples_read);
    }

    println!("Stopping scan");
    dev.a_in_scan_stop()?;
    dev.a_in_scan_cleanup()?;

    Ok(())
}
```