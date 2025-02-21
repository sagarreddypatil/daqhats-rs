use anyhow;
use std::panic;
use daqhats::core::{hat_list, HatId, Mcc118};
use daqhats::ScanOptions;

fn main() -> anyhow::Result<(), anyhow::Error> {
    let avail_devices = hat_list(HatId::Mcc118);
    if avail_devices.len() == 0 {
        return Err(anyhow::Error::msg("No MCC 118 devices found"));
    }

    let addr = avail_devices[0].address;
    let dev = Mcc118::open(addr)?;

    let opts = ScanOptions {
        channel_mask: 0b1111,
        sample_rate_per_channel: 1000.0,

        scale_data: true,
        calibrate_data: true,
        external_clock: false,
        external_trigger: false,
    };

    // scan for 30 seconds
    println!("Scanning for 30 seconds");
    let start = std::time::Instant::now();

    // ownership of dev is moved into the thread
    let (handle, receivers) = daqhats::scan_channels(dev, opts)?;

    while std::time::Instant::now() - start < std::time::Duration::from_secs(30) {
        for (i, rx) in receivers.iter().enumerate() {
            if let Ok(val) = rx.try_recv() {
                println!("Channel {}: {}", i, val);
            }
        }
    }

    // you must drop receivers to signal the thread to stop
    drop(receivers);

    // the thread returns the device back
    let mut dev = handle.join().map_err(|e| panic::resume_unwind(e)).unwrap();

    // do anything else with dev here
    dev.blink_led(5)?; // example

    Ok(())
}
