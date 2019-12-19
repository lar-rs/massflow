/// Pressure sensor
/// CHannel:  `Analog:CH04`
/// Model:     `presurei877`
///
use std::fmt;
use std::path::{Path,PathBuf};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

pub const PRESSURE:  &'static str      = "pressure";
/// Presure value model
///
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Pressure {
    pub value: f32,
}

impl Pressure {
    pub fn from_adc16(value: i16) -> Pressure {
        let mut voltage =  value as f32 / i16::max_value() as f32 * 5.0;
        Pressure::from_voltage(voltage)
    }
    pub fn from_voltage(voltage: f32) -> Pressure {
        let broken = voltage< 1.0 * 4.0 / 5.0;
        let mut value = ((voltage - 1.0)  / (5.0 - 1.0))*1000.0;
        if broken {
            value = -1.0;
        }
        Pressure{value}
    }
}


impl fmt::Display for Pressure {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f,"pressure :{} {} ",self.value,if self.value<0.0 {"BROKEN"} else {"OK"})
    }
}


pub fn pressure(path:&Path) -> io::Result<Pressure> {
    let value = fs::read_to_string(path.join(PRESSURE)).unwrap_or(String::from("-1.0")).parse::<f32>().expect("pressure  value convert to f32 error");;
   Ok(Pressure{value})
}

