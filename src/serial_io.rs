use std::io::{ErrorKind, Read, Write};
use std::time::Duration;

use serialport::SerialPort;

const SERIAL_NOT_CONNECTED_ERR_MSG: &str = "Serial device not connected";

pub struct SerialIO {
    serial_port: Option<serialport::TTYPort>,
}

impl SerialIO {
    pub fn new() -> Self {
        Self { serial_port: None }
    }

    fn get_serialport(&mut self) -> Result<&mut serialport::TTYPort, serialport::Error> {
        if self.serial_port.is_some() {
            return Ok(self.serial_port.as_mut().unwrap());
        }

        for p in Self::enumerate_usb_ttys() {
            let builder = serialport::new(&p, 19200)
                .stop_bits(serialport::StopBits::One)
                .data_bits(serialport::DataBits::Eight)
                .flow_control(serialport::FlowControl::None)
                .timeout(Duration::new(1, 0));
            if let Ok(p) = builder.open_native() {
                // Remove any bytes already in the buffer of serial device
                let _ = p.clear(serialport::ClearBuffer::All);
                self.serial_port = Some(p);
                return Ok(self.serial_port.as_mut().unwrap());
            } else {
                println!("Unable to open serial device `{}`", &p);
            }
        }
        Err(serialport::Error::new(serialport::ErrorKind::NoDevice, ""))
    }

    fn enumerate_usb_ttys() -> Vec<String> {
        let mut result = Vec::new();
        if let Ok(entries) = std::fs::read_dir("/dev") {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_file()
                    && !path.is_dir()
                    && path
                        .as_path()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .starts_with("ttyUSB")
                {
                    result.push(path.display().to_string());
                }
            }
        }
        result
    }

    fn handle_io_error(&mut self, error: &std::io::Error) {
        if error.kind() != std::io::ErrorKind::TimedOut {
            self.serial_port = None;
        }
    }

    pub fn clear_buffers(&mut self) {
        if let Some(p) = &self.serial_port {
            let _ = p.clear(serialport::ClearBuffer::All);
        }
    }
}

impl Write for SerialIO {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.get_serialport() {
            Ok(p) => p.write(buf),
            Err(_) => Err(std::io::Error::new(
                ErrorKind::Other,
                SERIAL_NOT_CONNECTED_ERR_MSG,
            )),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self.get_serialport() {
            Ok(p) => p.flush(),
            Err(_) => Err(std::io::Error::new(
                ErrorKind::Other,
                SERIAL_NOT_CONNECTED_ERR_MSG,
            )),
        }
    }
}

impl Read for SerialIO {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.get_serialport() {
            Ok(p) => {
                let read_result = p.read(buf);
                if let Err(ref e) = read_result {
                    self.handle_io_error(e);
                }
                read_result
            }
            Err(_) => Err(std::io::Error::new(
                ErrorKind::Other,
                SERIAL_NOT_CONNECTED_ERR_MSG,
            )),
        }
    }
}
