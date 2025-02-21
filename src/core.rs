use crate::bindings;
use bitflags::bitflags;

pub fn result_c_to_rs(code: i32) -> Result<(), ErrorCode> {
    if code == bindings::ResultCode_RESULT_SUCCESS {
        Ok(())
    } else {
        Err(code.into())
    }
}


#[derive(Copy, Clone, Debug)]
pub enum HatId {
    ANY=bindings::HatIDs_HAT_ID_ANY as isize,
    Mcc118=bindings::HatIDs_HAT_ID_MCC_118 as isize,
    Mcc118Bootloader=bindings::HatIDs_HAT_ID_MCC_118_BOOTLOADER as isize,
    Mcc128=bindings::HatIDs_HAT_ID_MCC_128 as isize,
    Mcc134=bindings::HatIDs_HAT_ID_MCC_134 as isize,
    Mcc152=bindings::HatIDs_HAT_ID_MCC_152 as isize,
    Mcc172=bindings::HatIDs_HAT_ID_MCC_172 as isize,
}

impl From<u16> for HatId {
    fn from(id: u16) -> Self {
        match id as u32 {
            bindings::HatIDs_HAT_ID_ANY => HatId::ANY,
            bindings::HatIDs_HAT_ID_MCC_118 => HatId::Mcc118,
            bindings::HatIDs_HAT_ID_MCC_118_BOOTLOADER => HatId::Mcc118Bootloader,
            bindings::HatIDs_HAT_ID_MCC_128 => HatId::Mcc128,
            bindings::HatIDs_HAT_ID_MCC_134 => HatId::Mcc134,
            bindings::HatIDs_HAT_ID_MCC_152 => HatId::Mcc152,
            bindings::HatIDs_HAT_ID_MCC_172 => HatId::Mcc172,
            _ => panic!("Invalid HatId"),
        }
    }
}

pub enum ErrorCode {
    BadParameter=bindings::ResultCode_RESULT_BAD_PARAMETER as isize,
    Busy=bindings::ResultCode_RESULT_BUSY as isize,
    Timeout=bindings::ResultCode_RESULT_TIMEOUT as isize,
    LockTimeout=bindings::ResultCode_RESULT_LOCK_TIMEOUT as isize,
    InvalidDevice=bindings::ResultCode_RESULT_INVALID_DEVICE as isize,
    ResourceUnavail=bindings::ResultCode_RESULT_RESOURCE_UNAVAIL as isize,
    CommsFailure=bindings::ResultCode_RESULT_COMMS_FAILURE as isize,
    Undefined=bindings::ResultCode_RESULT_UNDEFINED as isize,
}

impl From<i32> for ErrorCode {
    fn from(code: i32) -> Self {
        match code {
            bindings::ResultCode_RESULT_BAD_PARAMETER => ErrorCode::BadParameter,
            bindings::ResultCode_RESULT_BUSY => ErrorCode::Busy,
            bindings::ResultCode_RESULT_TIMEOUT => ErrorCode::Timeout,
            bindings::ResultCode_RESULT_LOCK_TIMEOUT => ErrorCode::LockTimeout,
            bindings::ResultCode_RESULT_INVALID_DEVICE => ErrorCode::InvalidDevice,
            bindings::ResultCode_RESULT_RESOURCE_UNAVAIL => ErrorCode::ResourceUnavail,
            bindings::ResultCode_RESULT_COMMS_FAILURE => ErrorCode::CommsFailure,
            bindings::ResultCode_RESULT_UNDEFINED => ErrorCode::Undefined,
            _ => panic!("Invalid ErrorCode"),
        }
    }
}

impl ErrorCode {
    pub fn message(&self) -> &'static str {
        match self {
            ErrorCode::BadParameter => "An incorrect parameter was passed to the function.",
            ErrorCode::Busy => "The device is busy.",
            ErrorCode::Timeout => "There was a timeout accessing a resource.",
            ErrorCode::LockTimeout => "There was a timeout while obtaining a resource lock.",
            ErrorCode::InvalidDevice => "The device at the specified address is not the correct type.",
            ErrorCode::ResourceUnavail => "A needed resource was not available.",
            ErrorCode::CommsFailure => "Could not communicate with the device.",
            ErrorCode::Undefined => "Undefined error.",
        }
    }
}

pub struct HatInfo {
    pub address: u8,
    pub id: HatId,
    pub version: u16,
    pub product_name: String,
}

impl From<bindings::HatInfo> for HatInfo {
    fn from(info: bindings::HatInfo) -> Self {
        let product_name = String::from_utf8_lossy(&info.product_name).into_owned();

        HatInfo {
            address: info.address,
            id: info.id.into(),
            version: info.version,
            product_name,
        }
    }
}

bitflags! {
    pub struct ScanOptions: u32 {
        const DEFAULT = bindings::OPTS_DEFAULT;
        const NOSCALEDATA = bindings::OPTS_NOSCALEDATA;
        const NOCALIBRATEDATA=bindings::OPTS_NOCALIBRATEDATA;
        const EXTCLOCK=bindings::OPTS_EXTCLOCK;
        const EXTTRIGGER=bindings::OPTS_EXTTRIGGER;
        const CONTINUOUS=bindings::OPTS_CONTINUOUS;
    }
}

bitflags! {
    pub struct ScanStatus: u16 {
        const HW_OVERRUN=bindings::STATUS_HW_OVERRUN as u16;
        const BUFFER_OVERRUN=bindings::STATUS_BUFFER_OVERRUN as u16;
        const TRIGGERED=bindings::STATUS_TRIGGERED as u16;
        const RUNNING=bindings::STATUS_RUNNING as u16;
    }
}

pub enum TriggerMode {
    RisingEdge=bindings::TriggerMode_TRIG_RISING_EDGE as isize,
    FallingEdge=bindings::TriggerMode_TRIG_FALLING_EDGE as isize,
    ActiveHigh=bindings::TriggerMode_TRIG_ACTIVE_HIGH as isize,
    ActiveLow=bindings::TriggerMode_TRIG_ACTIVE_LOW as isize,
}

pub fn hat_list(filter_id: HatId) -> Vec<HatInfo> {
    let count: i32 = unsafe { bindings::hat_list(filter_id as u16, std::ptr::null_mut()) };
    assert!(count >= 0);
    assert!(count < bindings::MAX_NUMBER_HATS as i32);

    let count = count as usize;
    let mut raw_hats = unsafe { std::mem::MaybeUninit::<[bindings::HatInfo; 8]>::uninit() };
    unsafe {bindings::hat_list(filter_id as u16, raw_hats.as_mut_ptr() as *mut bindings::HatInfo)};
    let raw_hats = unsafe { raw_hats.assume_init() };

    let mut hats: Vec<HatInfo> = Vec::new();
    for i in 0..count {
        let raw_hat = raw_hats[i];

        hats.push(raw_hat.into());
    }

    hats
}

pub fn hat_wait_for_interrupt(timeout_ms: i32) -> Result<(), ErrorCode> {
    let res = unsafe { bindings::hat_wait_for_interrupt(timeout_ms) };
    result_c_to_rs(res)
}

pub fn hat_interrupt_state() -> bool {
    unsafe { bindings::hat_interrupt_state() == 1 }
}

pub fn hat_interrupt_callback_enable<T>(callback: fn(T), user_data: &T) -> Result<(), ErrorCode> {
    let c_cb = unsafe { std::mem::transmute(callback) };

    let res = unsafe {
        bindings::hat_interrupt_callback_enable(
            Some(c_cb),
            user_data as *const T as *mut std::ffi::c_void
        )
    };

    result_c_to_rs(res)
}

pub fn hat_interrupt_callback_disable() -> Result<(), ErrorCode> {
    let res = unsafe { bindings::hat_interrupt_callback_disable() };
    result_c_to_rs(res)
}


