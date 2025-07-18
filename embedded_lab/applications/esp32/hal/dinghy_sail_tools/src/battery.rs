use crate::domain::DataBrooker;
use micromath::F32Ext;

pub const VOLTAGE_DIVIDER_RATIO: f32 = 2.0;
pub const VREF_MV: u32 = 3100;
pub const ADC_MAX_VALUE: u32 = 4095;
pub const CORRECTION_FACTOR: f32 = 1.1045;

pub fn caclutate_battery_voltage(raw_value: u16, gps_data: &mut DataBrooker) -> &DataBrooker {
    let voltage_adc_mv = (raw_value as u32 * VREF_MV) / ADC_MAX_VALUE;
    let battery_voltage_v = (voltage_adc_mv as f32 / 1000.0) * VOLTAGE_DIVIDER_RATIO;
    let calibrated_voltage = ((battery_voltage_v * CORRECTION_FACTOR) * 1000.0);

    gps_data.voltage = Some(calibrated_voltage.round() / 1000.0 );
    gps_data
}
