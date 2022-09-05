use std::io::{Read, Write};
use std::iter::zip;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};

use crate::db_io::DbIO;
use crate::protocol::{
    self, CumEnergyReadType, MeasurementType, Request, Response, REQUEST_LEN, RESPONSE_LEN,
};
use crate::serial_io::SerialIO;
use crate::solar_data::SolarData;

const MEASUREMENT_INTERVAL: Duration = Duration::new(5 * 60, 0);

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Parse(protocol::ParseError),
    Db(rusqlite::Error),
    Serial(serialport::Error),
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> AppError {
        AppError::Db(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> AppError {
        AppError::Io(e)
    }
}

impl From<protocol::ParseError> for AppError {
    fn from(e: protocol::ParseError) -> AppError {
        AppError::Parse(e)
    }
}

struct Tracker {
    device_id: u8,
    array_id: u8,
}

pub struct App {
    serial_io: SerialIO,
    db_io: DbIO,
    trackers: Vec<Tracker>,
}

impl App {
    pub fn new(db_path: &str) -> Result<App, AppError> {
        let serial_io = SerialIO::new();
        let db_io = DbIO::new(db_path)?;
        Ok(App {
            serial_io,
            db_io,
            trackers: vec![
                Tracker {
                    device_id: 2,
                    array_id: 1,
                },
                Tracker {
                    device_id: 2,
                    array_id: 2,
                },
                Tracker {
                    device_id: 3,
                    array_id: 1,
                },
            ],
        })
    }

    fn build_request_sequences(trackers: &[Tracker]) -> Vec<Vec<Request>> {
        let mut requests = vec![];
        for tracker in trackers.iter() {
            let mut sub_requests = vec![
                /*Request::State {
                    address: tracker.device_id,
                },*/
                Request::CumEnergy {
                    address: tracker.device_id,
                    read_type: CumEnergyReadType::Total,
                },
                Request::MeasureDsp {
                    address: tracker.device_id,
                    measurement_type: MeasurementType::GridVoltage,
                    global: 1,
                },
                Request::MeasureDsp {
                    address: tracker.device_id,
                    measurement_type: MeasurementType::InverterTemperature,
                    global: 0,
                },
            ];

            match tracker.array_id {
                1 => sub_requests.append(&mut vec![Request::MeasureDsp {
                    address: tracker.device_id,
                    measurement_type: MeasurementType::Pin1,
                    global: 0,
                }]),
                2 => sub_requests.append(&mut vec![Request::MeasureDsp {
                    address: tracker.device_id,
                    measurement_type: MeasurementType::Pin2,
                    global: 0,
                }]),
                _ => (),
            }
            requests.push(sub_requests);
        }
        requests
    }

    pub fn run(&mut self) {
        loop {
            let t0 = Instant::now();
            let request_sequences = Self::build_request_sequences(&self.trackers);
            'tracker_loop: for (tracker, request_sequence) in
                zip(&self.trackers, &request_sequences)
            {
                let mut data = SolarData::new(tracker.device_id, tracker.array_id);
                for &request in request_sequence.iter() {
                    let max_attempts = 3;
                    for attempt in 1..=max_attempts {
                        self.serial_io.clear_buffers();
                        let read_result =
                            Self::read_into(&mut self.serial_io, &mut data, request);
                        match read_result {
                            Ok(_) => break,
                            Err(e) => {
                                println!("E: {:?}", e);
                                if attempt == max_attempts {
                                    continue 'tracker_loop;
                                }
                                sleep(Duration::new(1, 0));
                            }
                        }
                    }
                }

                data.timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                println!("I: inserting {:?}", data);
                let _ = self.db_io.insert_solar_data(data);
            }

            let sleep_duration = MEASUREMENT_INTERVAL - (Instant::now() - t0);
            sleep(sleep_duration);
        }
    }

    pub fn read_into(
        serial_io: &mut SerialIO,
        solar_data: &mut SolarData,
        request: Request,
    ) -> Result<(), AppError> {
        let mut response_buf = [0; RESPONSE_LEN];
        serial_io
            .write_all(&<[u8; REQUEST_LEN]>::from(request))
            .and(serial_io.flush())
            .and(serial_io.read_exact(&mut response_buf))?;

        let response = Response::from_bytes(&request, &response_buf)?;
        solar_data.set_measurement_data(&response, &request);
        Ok(())
    }
}
