pub mod core;

use std::sync::mpsc;
use std::thread::JoinHandle;

#[derive(Copy, Clone, Debug)]
pub struct ScanOptions {
    pub channel_mask: u8,
    pub sample_rate_per_channel: f64,

    pub scale_data: bool,
    pub calibrate_data: bool,
    pub external_clock: bool,
    pub external_trigger: bool,
}

impl ScanOptions {
    pub fn channel_count(&self) -> usize {
        self.channel_mask.count_ones() as usize
    }
}

pub fn scan_channels<T: core::AInScanner + std::marker::Send + 'static>(mut dev: T, opts: ScanOptions) -> Result<(JoinHandle<T>, Vec<mpsc::Receiver<f64>>), core::ErrorCode> {
    let mut low_opts = core::ScanOptions::CONTINUOUS;
    if !opts.scale_data {//             low_opts |= core::ScanOptions::NOSCALEDATA;
    }
    if !opts.calibrate_data {
        low_opts |= core::ScanOptions::NOCALIBRATEDATA;
    }
    if opts.external_clock {
        low_opts |= core::ScanOptions::EXTCLOCK;
    }
    if opts.external_trigger {
        low_opts |= core::ScanOptions::EXTTRIGGER;
    }

    dev.a_in_scan_start(opts.channel_mask, 0, opts.sample_rate_per_channel, low_opts)?;

    let n_ch = opts.channel_count();
    let channels = (0..n_ch).map(|_| mpsc::channel::<f64>()).collect::<Vec<_>>();

    let senders = channels.iter().map(|(tx, _)| tx.clone()).collect::<Vec<_>>();
    let receivers = channels.into_iter().map(|(_, rx)| rx).collect::<Vec<_>>();

    let handle = std::thread::spawn(move || {
        let mut read_buf = vec![0.0; n_ch];

        loop {
            // read 1 sample per channel, infinite timeout
            match dev.a_in_scan_read(1, 0.0, &mut read_buf) {
                Ok((status, samples_read)) => {
                    let mut end_loop = false;
                    if status.contains(core::ScanStatus::HW_OVERRUN) {
                        eprintln!("hardware overrun detected");
                        end_loop = true;
                    }
                    if status.contains(core::ScanStatus::BUFFER_OVERRUN) {
                        eprintln!("buffer overrun detected");
                        end_loop = true;
                    }
                    if !status.contains(core::ScanStatus::RUNNING) {
                        eprintln!("scan stopped unexpectedly");
                        end_loop = true;
                    }
                    if samples_read == 0 {
                        eprintln!("no samples read");
                        end_loop = true;
                    }

                    if end_loop {
                        break;
                    }

                    for i in 0..n_ch {
                        if let Err(_) = senders[i].send(read_buf[i]) {
                            // receiver was dropped, stop thread
                            break;
                        }
                    }
                },
                Err(err) => {
                    eprintln!("error reading samples: {:}", err);
                    break;
                }
            };
        }

        // stop scan and cleanup
        if let Err(err) = dev.a_in_scan_stop() {
            eprintln!("error stopping scan: {:?}", err);
        }
        if let Err(err) = dev.a_in_scan_cleanup() {
            eprintln!("error cleaning up scan: {:?}", err);
        }

        dev
    });

    Ok((handle, receivers))
}
