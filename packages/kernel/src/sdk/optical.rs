//! V5 Optical Sensor

use core::ffi::c_double;

use vex_sdk::*;

pub extern "C" fn vexDeviceOpticalHueGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceOpticalSatGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceOpticalBrightnessGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceOpticalProximityGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceOpticalRgbGet(device: V5_DeviceT, data: *mut V5_DeviceOpticalRgb) {
    log::debug!("vexDeviceOpticalRgbGet called with data: {:?}", data);
}
pub extern "C" fn vexDeviceOpticalLedPwmSet(device: V5_DeviceT, value: i32) {
    log::debug!("vexDeviceOpticalLedPwmSet called with value: {:?}", value);
}
pub extern "C" fn vexDeviceOpticalLedPwmGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceOpticalStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceOpticalRawGet(device: V5_DeviceT, data: *mut V5_DeviceOpticalRaw) {
    log::debug!("vexDeviceOpticalRawGet called with data: {:?}", data);
}
pub extern "C" fn vexDeviceOpticalModeSet(device: V5_DeviceT, mode: u32) {
    log::debug!("vexDeviceOpticalModeSet called with mode: {:?}", mode);
}
pub extern "C" fn vexDeviceOpticalModeGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceOpticalGestureGet(
    device: V5_DeviceT,
    pData: *mut V5_DeviceOpticalGesture,
) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceOpticalGestureEnable(device: V5_DeviceT) {
    log::debug!("vexDeviceOpticalGestureEnable called");
}
pub extern "C" fn vexDeviceOpticalGestureDisable(device: V5_DeviceT) {
    log::debug!("vexDeviceOpticalGestureDisable called");
}
pub extern "C" fn vexDeviceOpticalProximityThreshold(device: V5_DeviceT, value: i32) {
    log::debug!(
        "vexDeviceOpticalProximityThreshold called with value: {:?}",
        value
    );
}
pub extern "C" fn vexDeviceOpticalIntegrationTimeSet(device: V5_DeviceT, timeMs: c_double) {
    log::debug!(
        "vexDeviceOpticalIntegrationTimeSet called with timeMs: {:?}",
        timeMs
    );
}
pub extern "C" fn vexDeviceOpticalIntegrationTimeGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
