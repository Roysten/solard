use num_enum::TryFromPrimitiveError;
use num_enum_derive::{IntoPrimitive, TryFromPrimitive};

pub const REQUEST_LEN: usize = 10;
pub const RESPONSE_LEN: usize = 8;
pub const CHECKSUM_LEN: usize = 2;

#[derive(Debug, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum TransmissionState {
    Ok = 0,
    NotImplemented = 51,
    VariableDoesNotExist = 52,
    VariableValueOutOfRange = 53,
    EepromNotAccessible = 54,
    NotToggledServiceMode = 55,
    InternalMicroIOError = 56,
    CommandNotExecuted = 57,
    VariableNotAvailableTryAgain = 58,
}

#[derive(Debug, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum GlobalState {
    SendingParameters = 0,
    WaitSunOrGrid = 1,
    CheckingGrid = 2,
    MeasuringRiso = 3,
    DcDcStart = 4,
    InverterStart = 5,
    Run = 6,
    Recovery = 7,
    Pause = 8,
    GroundFault = 9,
    OTHFault = 10,
    AddressSetting = 11,
    SelfTest = 12,
    SelfTestFail = 13,
    SensorTestAndMeasRiso = 14,
    LeakFault = 15,
    Waitingformanualreset = 16,
    InternalErrorE026 = 17,
    InternalErrorE027 = 18,
    InternalErrorE028 = 19,
    InternalErrorE029 = 20,
    InternalErrorE030 = 21,
    SendingWindTable = 22,
    FailedSendingtable = 23,
    UTHFault = 24,
    RemoteOFF = 25,
    InterlockFail = 26,
    ExecutingAutotest = 27,
    WaitingSun = 30,
    TemperatureFault = 31,
    FanStaucked = 32,
    IntComFault = 33,
    SlaveInsertion = 34,
    DCSwitchOpen = 35,
    TRASSwitchOpen = 36,
    MasterExclusion = 37,
    AutoExclusion = 38,
    ErasingInternalEeprom = 98,
    ErasingExternalEeprom = 99,
    CountingEeprom = 100,
    Freeze = 101,
}

#[derive(Debug, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum DcDcState {
    DcDcOFF = 0,
    RampStart = 1,
    MPPT = 2,
    NotUsed = 3,
    InputOC = 4,
    InputUV = 5,
    InputOV = 6,
    InputLow = 7,
    NoParameters = 8,
    BulkOV = 9,
    CommunicationError = 10,
    RampFail = 11,
    InternalError = 12,
    InputmodeError = 13,
    GroundFault = 14,
    InverterFail = 15,
    DcDcIGBTSat = 16,
    DcDcILEAKFail = 17,
    DcDcGridFail = 18,
    DcDcCommError = 19,
}

#[derive(Debug, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum InverterState {
    StandBy = 0,
    CheckingGrid = 1,
    Run = 2,
    BulkOV = 3,
    OutOC = 4,
    IGBTSat = 5,
    BulkUV = 6,
    DegaussError = 7,
    NoParameters = 8,
    BulkLow = 9,
    GridOV = 10,
    CommunicationError = 11,
    Degaussing = 12,
    Starting = 13,
    BulkCapFail = 14,
    LeakFail = 15,
    DcDcFail = 16,
    IleakSensorFail = 17,
    SelfTestRelayInverter = 18,
    SelfTestWaitForSensorTest = 19,
    SelfTestTestRelayDcDcAndSensor = 20,
    SelfTestRelayInverterFail = 21,
    SelfTestTimeoutFail = 22,
    SelfTestRelayDcDcFail = 23,
    SelfTest1 = 24,
    WaitingSelftestStart = 25,
    DcInjection = 26,
    SelfTest2 = 27,
    SelfTest3 = 28,
    SelfTest4 = 29,
    InternalError = 30,
    InternalErrorAlt = 31,
    ForbiddenState = 40,
    InputUC = 41,
    ZeroPower = 42,
    GridNotPresent = 43,
    WaitingStart = 44,
    MPPT = 45,
    GridFail = 46,
    InputOC = 47,
}

#[derive(Debug, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum AlarmState {
    NoAlarm = 0,
    SunLowW001 = 1,
    InputOCE001 = 2,
    InputUVW002 = 3,
    InputOVE002 = 4,
    SunLowW001Alt = 5,
    NoParametersE003 = 6,
    BulkOVE004 = 7,
    CommErrorE005 = 8,
    OutputOCE006 = 9,
    IGBTSatE007 = 10,
    BulkUVW011 = 11,
    InternalerrorE009 = 12,
    GridFailW003 = 13,
    BulkLowE010 = 14,
    RampFailE011 = 15,
    DcDcFailE012 = 16,
    WrongModeE013 = 17,
    GroundFault = 18,
    OverTempE014 = 19,
    BulkCapFailE015 = 20,
    InverterFailE016 = 21,
    StartTimeoutE017 = 22,
    GroundFaultE018 = 23,
    DegaussError = 24,
    IleaksensfailE019 = 25,
    DcDcFailE012Alt = 26,
    SelfTestError1E020 = 27,
    SelfTestError2E021 = 28,
    SelfTestError3E019 = 29,
    SelfTestError4E022 = 30,
    DCinjerrorE023 = 31,
    GridOVW004 = 32,
    GridUVW005 = 33,
    GridOFW006 = 34,
    GridUFW007 = 35,
    ZgridHiW008 = 36,
    InternalerrorE024 = 37,
    RisoLowE025 = 38,
    VrefErrorE026 = 39,
    ErrorMeasVE027 = 40,
    ErrorMeasFE028 = 41,
    ErrorMeasZE029 = 42,
    ErrorMeasIleakE030 = 43,
    ErrorReadVE031 = 44,
    ErrorReadIE032 = 45,
    TablefailW009 = 46,
    FanFailW010 = 47,
    UTHE033 = 48,
    InterlockfailE034 = 49,
    RemoteOffE035 = 50,
    VoutAvgerrrorE036 = 51,
    BatterylowW012 = 52,
    ClkfailW013 = 53,
    InputUCE037 = 54,
    ZeroPowerW014 = 55,
    FanStuckedE038 = 56,
    DCSwitchOpenE039 = 57,
    TrasSwitchOpenE040 = 58,
    ACSwitchOpenE041 = 59,
    BulkUVE042 = 60,
    AutoexclusionE043 = 61,
    GriddfdtW015 = 62,
    DenswitchOpenW016 = 63,
    JboxfailW017 = 64,
}

#[derive(Debug, Copy, Clone, IntoPrimitive)]
#[repr(u8)]
pub enum MeasurementType {
    GridVoltage = 1,
    GridCurrent = 2,
    GridPower = 3,
    Frequency = 4,
    VBulk = 5,
    ILeakDcDc = 6,
    ILeakInverter = 7,
    Pin1 = 8,
    Pin2 = 9,
    InverterTemperature = 21,
    BoosterTemperature = 22,
    Input1Voltage = 23,
    Input1Current = 25,
    Input2Voltage = 26,
    Input2Current = 27,
    GridVoltageDcDc = 28,
    GridFrequencyDcDc = 29,
    IsolationResistance = 30,
    VBlukDcDc = 31,
    AverageGridVoltage = 32,
    VBulkMid = 33,
    PowerPeak = 34,
    PowerPeakToday = 35,
    GridVoltageNeutral = 36,
    WindGeneratorFrequency = 37,
    GridVoltageNeutralPhase = 38,
    GridCurrentPhaseR = 39,
    GridCurrentPhaseS = 40,
    GridCurrentPhaseT = 41,
    FrequencyPhaseR = 42,
    FrequencyPhaseS = 43,
    FrequencyPhaseT = 44,
    VBulkPositive = 45,
    VBulkNegative = 46,
    SupervisorTemperature = 47,
    AlimTemperature = 48,
    HeatSinkTemperature = 49,
    Temperature1 = 50,
    Temperature2 = 51,
    Temperature3 = 52,
    Fan1Speed = 53,
    Fan2Speed = 54,
    Fan3Speed = 55,
    Fan4Speed = 56,
    Fan5Speed = 57,
    PowerSaturationLimit = 58,
    RiferamentoAnelloBulk = 59,
    VPanelMicro = 60,
    GridVoltagePhaseR = 61,
    GridVoltagePhaseS = 62,
    GridVoltagePhaseT = 63,
}

#[derive(Debug, Copy, Clone, IntoPrimitive)]
#[repr(u8)]
pub enum CumFloatEnergyReadType {
    CurrentDay = 1,
    CurrentWeek = 2,
    CurrentMonth = 3,
    CurrentYear = 4,
    LastNDays = 5,
    Total = 6,
    Partial = 7,
}

#[derive(Debug, Copy, Clone, IntoPrimitive)]
#[repr(u8)]
pub enum CumEnergyReadType {
    Daily = 0,
    Weekly = 1,
    Month = 3,
    Year = 4,
    Total = 5,
    Partial = 6,
}

#[derive(Debug, Copy, Clone, IntoPrimitive)]
#[repr(u8)]
pub enum CommandType {
    State = 50,
    Pn = 52,
    Version = 58,
    MeasureDsp = 59,
    SerialNumber = 63,
    ManufacturingWeekYear = 65,
    FlagsOrSwitch = 67,
    CumFloatEnergy = 68,
    TimeDate = 70,
    FirmwareRelease = 72,
    CumEnergy = 78,
    BaudRateSettings = 85,
    LastFourAlarms = 86,
}

fn is_valid_checksum(data: &[u8]) -> bool {
    let checksum = checksum(&data[0..data.len() - CHECKSUM_LEN]);
    let (checksum_l, checksum_h) = (checksum as u8, (checksum >> 8) as u8);

    (checksum_l, checksum_h)
        == (
            data[data.len() - CHECKSUM_LEN],
            data[data.len() - CHECKSUM_LEN + 1],
        )
}

pub fn checksum(data: &[u8]) -> u16 {
    let (mut bcc_lo, mut bcc_hi) = (0xFFu8, 0xFFu8);

    for &new in data {
        let mut new = new ^ bcc_lo;
        let mut tmp = new << 4;
        new = tmp ^ new;
        tmp = new >> 5;
        bcc_lo = bcc_hi;
        bcc_hi = new ^ tmp;
        tmp = new << 3;
        bcc_lo = bcc_lo ^ tmp;
        tmp = new >> 4;
        bcc_lo = bcc_lo ^ tmp;
    }

    u16::from_le_bytes([!bcc_lo, !bcc_hi])
}

#[derive(Debug)]
pub struct ParseError {}

impl<Enum: num_enum::TryFromPrimitive> From<TryFromPrimitiveError<Enum>> for ParseError {
    fn from(_e: TryFromPrimitiveError<Enum>) -> Self {
        Self {}
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Request {
    State {
        address: u8,
    },
    Pn {
        address: u8,
    },
    Version {
        address: u8,
    },
    MeasureDsp {
        address: u8,
        measurement_type: MeasurementType,
        global: u8,
    },
    SerialNumber {
        address: u8,
    },
    ManufacturingWeekYear {
        address: u8,
    },
    FlagsOrSwitch {
        address: u8,
    },
    CumFloatEnergy {
        address: u8,
        read_type: CumFloatEnergyReadType,
        day_count: u16,
        global: u8,
    },
    TimeDate {
        address: u8,
    },
    FirmwareRelease {
        address: u8,
        release_type: u8,
    },
    CumEnergy {
        address: u8,
        read_type: CumEnergyReadType,
    },
    BaudRateSettings {
        address: u8,
        baud_code: u8,
        serial_line: u8,
    },
    LastFourAlarms {
        address: u8,
    },
}

impl Into<CommandType> for Request {
    fn into(self) -> CommandType {
        match self {
            Self::State { .. } => CommandType::State,
            Self::Pn { .. } => CommandType::Pn,
            Self::Version { .. } => CommandType::Version,
            Self::MeasureDsp { .. } => CommandType::MeasureDsp,
            Self::SerialNumber { .. } => CommandType::SerialNumber,
            Self::ManufacturingWeekYear { .. } => CommandType::ManufacturingWeekYear,
            Self::FlagsOrSwitch { .. } => CommandType::FlagsOrSwitch,
            Self::CumFloatEnergy { .. } => CommandType::CumFloatEnergy,
            Self::TimeDate { .. } => CommandType::TimeDate,
            Self::FirmwareRelease { .. } => CommandType::FirmwareRelease,
            Self::CumEnergy { .. } => CommandType::CumEnergy,
            Self::BaudRateSettings { .. } => CommandType::BaudRateSettings,
            Self::LastFourAlarms { .. } => CommandType::LastFourAlarms,
        }
    }
}

impl From<Request> for [u8; REQUEST_LEN] {
    fn from(request: Request) -> Self {
        let mut bytes = [0; REQUEST_LEN];
        let command_type: CommandType = request.into();
        match request {
            Request::State { address }
            | Request::Pn { address }
            | Request::Version { address }
            | Request::TimeDate { address }
            | Request::LastFourAlarms { address }
            | Request::SerialNumber { address }
            | Request::ManufacturingWeekYear { address }
            | Request::FlagsOrSwitch { address } => {
                bytes[0] = address;
                bytes[1] = command_type.into();
            }
            Request::MeasureDsp {
                address,
                measurement_type,
                global,
            } => {
                bytes[0] = address;
                bytes[1] = command_type.into();
                bytes[2] = measurement_type.into();
                bytes[3] = global;
            }
            Request::CumFloatEnergy {
                address,
                read_type,
                day_count,
                global,
            } => {
                bytes[0] = address;
                bytes[1] = command_type.into();
                bytes[2] = read_type.into();
                bytes[3] = (day_count >> 8) as u8;
                bytes[4] = day_count as u8;
                bytes[5] = global;
            }
            Request::FirmwareRelease {
                address,
                release_type,
            } => {
                bytes[0] = address;
                bytes[1] = command_type.into();
                bytes[2] = release_type;
            }
            Request::CumEnergy { address, read_type } => {
                bytes[0] = address;
                bytes[1] = command_type.into();
                bytes[2] = read_type.into();
            }
            Request::BaudRateSettings {
                address,
                baud_code,
                serial_line,
            } => {
                bytes[0] = address;
                bytes[1] = baud_code;
                bytes[2] = serial_line;
            }
        }
        let checksum = checksum(&bytes[..REQUEST_LEN - CHECKSUM_LEN]);
        bytes[REQUEST_LEN - CHECKSUM_LEN] = checksum as u8;
        bytes[REQUEST_LEN - CHECKSUM_LEN + 1] = (checksum >> 8) as u8;
        bytes
    }
}

#[derive(Debug)]
pub enum Response {
    State {
        transmission_state: TransmissionState,
        global_state: GlobalState,
        inverter_state: InverterState,
        ch1_state: DcDcState,
        ch2_state: DcDcState,
        alarm_state: AlarmState,
    },
    Pn {
        pn: String,
    },
    Version {
        transmission_state: TransmissionState,
        global_state: GlobalState,
        par: String,
    },
    MeasureDsp {
        transmission_state: TransmissionState,
        global_state: GlobalState,
        value: f32,
    },
    SerialNumber {
        serial_number: String,
    },
    ManufacturingWeekYear {
        transmission_state: TransmissionState,
        global_state: GlobalState,
        wwyy: String,
    },
    FlagsOrSwitch {
        transmission_state: TransmissionState,
        global_state: GlobalState,
        flag1: u8,
        flag2: u8,
        switch1: u8,
        switch2: u8,
    },
    CumFloatEnergy {
        transmission_state: TransmissionState,
        global_state: GlobalState,
        value: f32,
    },
    TimeDate {
        transmission_state: TransmissionState,
        global_state: GlobalState,
        time: u32,
    },
    FirmwareRelease {
        transmission_state: TransmissionState,
        global_state: GlobalState,
        version: String,
    },
    CumEnergy {
        transmission_state: TransmissionState,
        global_state: GlobalState,
        energy_wh: u32,
    },
    LastFourAlarms {
        transmission_state: TransmissionState,
        global_state: GlobalState,
        alarm_fifo_queue: [AlarmState; 4],
    },
}

impl Response {
    /** We need a request because the response does not contain data to identify it by */
    pub fn from_bytes(request: &Request, data: &[u8]) -> Result<Self, ParseError> {
        if data.len() < RESPONSE_LEN || !is_valid_checksum(data) {
            return Err(ParseError {});
        }

        match request {
            Request::State { .. } => Ok(Response::State {
                transmission_state: data[0].try_into()?,
                global_state: data[1].try_into()?,
                inverter_state: data[2].try_into()?,
                ch1_state: data[3].try_into()?,
                ch2_state: data[4].try_into()?,
                alarm_state: data[5].try_into()?,
            }),
            Request::Pn { .. } => Ok(Response::Pn {
                pn: String::from_utf8_lossy(&data[..6]).to_string(),
            }),
            Request::Version { .. } => Ok(Response::Version {
                transmission_state: data[0].try_into()?,
                global_state: data[1].try_into()?,
                par: String::from_utf8_lossy(&data[2..6]).to_string(),
            }),
            Request::MeasureDsp { .. } => Ok(Response::MeasureDsp {
                transmission_state: data[0].try_into()?,
                global_state: data[1].try_into()?,
                value: f32::from_be_bytes([data[2], data[3], data[4], data[5]]),
            }),
            Request::SerialNumber { .. } => Ok(Response::SerialNumber {
                serial_number: String::from_utf8_lossy(&data[..6]).to_string(),
            }),
            Request::ManufacturingWeekYear { .. } => Ok(Response::ManufacturingWeekYear {
                transmission_state: data[0].try_into()?,
                global_state: data[1].try_into()?,
                wwyy: String::from_utf8_lossy(&data[2..6]).to_string(),
            }),
            Request::FlagsOrSwitch { .. } => Ok(Response::FlagsOrSwitch {
                transmission_state: data[0].try_into()?,
                global_state: data[1].try_into()?,
                flag1: data[2],
                flag2: data[3],
                switch1: data[4],
                switch2: data[5],
            }),
            Request::CumFloatEnergy { .. } => Ok(Response::CumFloatEnergy {
                transmission_state: data[0].try_into()?,
                global_state: data[1].try_into()?,
                value: f32::from_be_bytes([data[2], data[3], data[4], data[5]]),
            }),
            Request::TimeDate { .. } => Ok(Response::TimeDate {
                transmission_state: data[0].try_into()?,
                global_state: data[1].try_into()?,
                time: u32::from_be_bytes([data[2], data[3], data[4], data[5]]),
            }),
            Request::FirmwareRelease { .. } => Ok(Response::FirmwareRelease {
                transmission_state: data[0].try_into()?,
                global_state: data[1].try_into()?,
                version: String::from_utf8_lossy(&data[2..6]).to_string(),
            }),
            Request::CumEnergy { .. } => Ok(Response::CumEnergy {
                transmission_state: data[0].try_into()?,
                global_state: data[1].try_into()?,
                energy_wh: u32::from_be_bytes([data[2], data[3], data[4], data[5]]),
            }),
            /* BaudRateSettings request has no response */
            Request::BaudRateSettings { .. } => Err(ParseError {}),
            Request::LastFourAlarms { .. } => Ok(Response::LastFourAlarms {
                transmission_state: data[0].try_into()?,
                global_state: data[1].try_into()?,
                alarm_fifo_queue: [
                    data[2].try_into()?,
                    data[3].try_into()?,
                    data[4].try_into()?,
                    data[5].try_into()?,
                ],
            }),
        }
    }
}

#[cfg(test)]
mod request_tests {
    use super::*;

    #[test]
    fn test_parse_alarms_response() {
        let checksumv = 23597u16;
        let data = vec![
            0,
            0,
            50,
            51,
            52,
            53,
            checksumv as u8,
            (checksumv >> 8) as u8,
        ];
        let response = Response::from_bytes(&Request::LastFourAlarms { address: 2 }, &data[..]);
        assert_eq!(true, response.is_ok());
    }
}
