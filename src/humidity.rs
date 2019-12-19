/// Humidity sensor
/// Channel adc:IN03
///
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io;
use std::fs;
use std::path::{Path};

pub const HUMIDITY:  &'static str      = "humidity";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Level{
    Normal,
    Warning,
    Critical,
    Brocket
}

use std::convert::TryFrom;
/// interface transfer

pub const VALUE:  &'static str     = "humidity";
pub const LIMIT:  &'static str     = "humidity_limit";


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Humidity {
    pub value:    f32,
}

impl Humidity {
    pub fn from_adc16(value: i16) -> Humidity {
        let signal =  (value as f32 / i16::max_value() as f32) * 5.0;
        Humidity::from_voltage(signal)
    }
    // 0..5v
    pub fn from_voltage(voltage:f32) -> Humidity {
        let broken = voltage < 0.8 * 4.0 / 5.0;
        let mut humidity = ((voltage - 0.8)  / (3.6 - 0.8))*100.0;
        if broken {
            humidity = -1.0;
        }
        Humidity {
            value:   humidity,
        }
    }
}

impl fmt::Display for Humidity {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f,"humidity :{} {} ",self.value,if self.value<0.0 {"BROKEN"} else {"OK"})
    }
}

pub fn humidity(path:&Path) -> io::Result<Humidity> {
    let value = fs::read_to_string(path.join(HUMIDITY)).unwrap_or(String::from("-1.0")).parse::<f32>().expect("humidity value convert error");
   Ok(Humidity{value})
}


