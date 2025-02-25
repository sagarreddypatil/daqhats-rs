use anyhow;
use daqhats::core::{hat_list, HatId, ScanOptions, ScanStatus, Mcc118, AInScanner};

fn main() -> anyhow::Result<(), anyhow::Error> {
    let avail_devices = hat_list(HatId::Mcc118);
    if avail_devices.len() == 0 {
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

    let mut dev = Mcc118::open(addr)?;
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