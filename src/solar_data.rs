use crate::protocol::{MeasurementType, Request, Response};

#[derive(Debug, Default)]
pub struct SolarData {
    pub device_id: u8,
    pub tracker_id: u8,
    pub timestamp: u64,
    pub energy_generation: usize,
    pub power_generation: usize,
    pub temperature: f32,
    pub voltage: f32,
}

impl SolarData {
    pub fn new(device_id: u8, tracker_id: u8) -> Self {
        SolarData {
            device_id,
            tracker_id,
            ..Default::default()
        }
    }

    pub fn set_measurement_data(&mut self, response: &Response, request: &Request) {
        match (response, request) {
            (
                Response::MeasureDsp { value, .. },
                Request::MeasureDsp {
                    measurement_type, ..
                },
            ) => {
                match measurement_type {
                    MeasurementType::GridPower => (), // TODO
                    MeasurementType::GridVoltage => self.voltage = *value,
                    MeasurementType::InverterTemperature => self.temperature = *value,
                    MeasurementType::Pin1 | MeasurementType::Pin2 => {
                        self.power_generation = *value as usize
                    }
                    _ => (),
                }
            }
            (Response::CumEnergy { energy_wh, .. }, _) => {
                self.energy_generation = *energy_wh as usize
            }
            _ => (),
        }
    }
}
