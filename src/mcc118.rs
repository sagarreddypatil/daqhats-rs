use crate::bindings;
use crate::core::{ErrorCode, ScanOptions, ScanStatus, TriggerMode, result_c_to_rs};

struct MCC118DeviceInfo {
    pub num_ai_channels: u8,
    pub ai_min_code: u16,
    pub ai_max_code: u16,
    pub ai_min_voltage: f64,
    pub ai_max_voltage: f64,
    pub ai_min_range: f64,
    pub ai_max_range: f64,
}

impl From<bindings::MCC118DeviceInfo> for MCC118DeviceInfo {
    fn from(info: bindings::MCC118DeviceInfo) -> Self {
        MCC118DeviceInfo {
            num_ai_channels: info.NUM_AI_CHANNELS,
            ai_min_code: info.AI_MIN_CODE,
            ai_max_code: info.AI_MAX_CODE,
            ai_min_voltage: info.AI_MIN_VOLTAGE,
            ai_max_voltage: info.AI_MAX_VOLTAGE,
            ai_min_range: info.AI_MIN_RANGE,
            ai_max_range: info.AI_MAX_RANGE,
        }
    }
}

struct MCC118 {
    pub address: u8,
}

impl MCC118 {
    pub fn open(address: u8) -> Result<MCC118, ErrorCode> {
        let res = unsafe { bindings::mcc118_open(address) };
        result_c_to_rs(res).map(|_| MCC118 { address })
    }

    pub fn close(self) -> Result<(), ErrorCode> {
        let res = unsafe { bindings::mcc118_close(self.address) };
        result_c_to_rs(res)
    }

    pub fn is_open(&self) -> () {
        let res = unsafe { bindings::mcc118_is_open(self.address) == 1 };
        if !res {
            panic!("mcc118 device at address {} is not open", self.address);
        }
    }

    pub fn blink_led(&self, count: u8) -> Result<(), ErrorCode> {
        let res = unsafe { bindings::mcc118_blink_led(self.address, count) };
        result_c_to_rs(res)
    }

    pub fn firmware_version(&self) -> Result<(u16, u16), ErrorCode> {
        let mut version = 0;
        let mut boot_version = 0;
        let res = unsafe { bindings::mcc118_firmware_version(self.address, &mut version, &mut boot_version) };
        result_c_to_rs(res).map(|_| (version, boot_version))
    }

    pub fn serial(&self) -> Result<String, ErrorCode> {
        let mut serial = [0u8; 9];
        let res = unsafe { bindings::mcc118_serial(self.address, serial.as_mut_ptr()) };
        result_c_to_rs(res).map(|_| String::from_utf8_lossy(&serial).into_owned())
    }

    pub fn calibration_date(&self) -> Result<String, ErrorCode> {
        let mut date = [0u8; 9];
        let res = unsafe { bindings::mcc118_calibration_date(self.address, date.as_mut_ptr()) };
        result_c_to_rs(res).map(|_| String::from_utf8_lossy(&date).into_owned())
    }

    pub fn calibration_coefficient_read(&self, index: u8) -> Result<(f64, f64), ErrorCode> {
        let mut slope = 0.0;
        let mut offset = 0.0;
        let res = unsafe { bindings::mcc118_calibration_coefficient_read(self.address, index, &mut slope, &mut offset) };
        result_c_to_rs(res).map(|_| (slope, offset))
    }

    pub fn calibration_coefficient_write(&self, index: u8, slope: f64, offset: f64) -> Result<(), ErrorCode> {
        let res = unsafe { bindings::mcc118_calibration_coefficient_write(self.address, index, slope, offset) };
        result_c_to_rs(res)
    }

    pub fn a_in_read(&self, channel: u8, options: ScanOptions) -> Result<f64, ErrorCode> {
        let mut value = 0.0;
        let res = unsafe { bindings::mcc118_a_in_read(self.address, channel, options.bits(), &mut value) };
        result_c_to_rs(res).map(|_| value)
    }

    pub fn trigger_mode(&self, mode: TriggerMode) -> Result<(), ErrorCode> {
        let res = unsafe { bindings::mcc118_trigger_mode(self.address, mode as u8) };
        result_c_to_rs(res)
    }

    pub fn a_in_scan_actual_rate(channel_count: u8, sample_rate_per_channel: f64) -> Result<f64, ErrorCode> {
        let mut actual_sample_rate = 0.0;
        let res = unsafe { bindings::mcc118_a_in_scan_actual_rate(channel_count, sample_rate_per_channel, &mut actual_sample_rate) };
        result_c_to_rs(res).map(|_| actual_sample_rate)
    }

    pub fn a_in_scan_start(&self, channel_mask: u8, samples_per_channel: u32, sample_rate_per_channel: f64, options: ScanOptions) -> Result<(), ErrorCode> {
        let res = unsafe { bindings::mcc118_a_in_scan_start(self.address, channel_mask, samples_per_channel, sample_rate_per_channel, options.bits()) };
        result_c_to_rs(res)
    }

    pub fn a_in_scan_buffer_size(&self) -> Result<u32, ErrorCode> {
        let mut size = 0;
        let res = unsafe { bindings::mcc118_a_in_scan_buffer_size(self.address, &mut size) };
        result_c_to_rs(res).map(|_| size)
    }

    pub fn a_in_scan_status(&self) -> Result<(ScanStatus, u32), ErrorCode> {
        let mut status = 0;
        let mut samples = 0;
        let res = unsafe { bindings::mcc118_a_in_scan_status(self.address, &mut status, &mut samples) };
        result_c_to_rs(res).map(|_| (ScanStatus::from_bits(status).unwrap(), samples))
    }

    pub fn a_in_scan_read(&self, samples_per_channel: i32, timeout_s: f64, buffer: &mut [f64]) -> Result<(ScanStatus, u32), ErrorCode> {
        let mut status: u16 = 0;
        let mut samples_read = 0;
        //  int mcc128_a_in_scan_read(uint8_t address, uint16_t *status, int32_t samples_per_channel, double timeout, double *buffer, uint32_t buffer_size_samples, uint32_t *samples_read_per_channel)
        let res = unsafe {
            bindings::mcc118_a_in_scan_read(
                self.address,
                &mut status,
                samples_per_channel,
                timeout_s,
                buffer.as_mut_ptr(),
                buffer.len() as u32,
                &mut samples_read,
            )
        };

        result_c_to_rs(res).map(|_| (ScanStatus::from_bits(status as u16).unwrap(), samples_read))
    }

    pub fn a_in_scan_channel_count(&self) -> u8 {
        let channel_count = unsafe { bindings::mcc118_a_in_scan_channel_count(self.address) };
        assert!(channel_count >= 0);
        assert!(channel_count <= 8);

        channel_count as u8
    }

    pub fn a_in_scan_stop(&self) -> Result<(), ErrorCode> {
        let res = unsafe { bindings::mcc118_a_in_scan_stop(self.address) };
        result_c_to_rs(res)
    }

    pub fn a_in_scan_cleanup(&self) -> Result<(), ErrorCode> {
        let res = unsafe { bindings::mcc118_a_in_scan_cleanup(self.address) };
        result_c_to_rs(res)
    }
}

impl Drop for MCC118 {
    fn drop(&mut self) {
        unsafe { bindings::mcc118_close(self.address) };
    }
}