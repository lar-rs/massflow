/// Airflow sensor
/// Input adc:IN01
/// Input adc:IN02
///

use serde::{Deserialize, Serialize};
use std::fmt;
use std::i16;
use std::io;
use std::fs;
use std::path::{Path};
/// für 0..60:   0.230197;
const A6:f32 = 0.003836617;
/// für 0..60:  -3.616438;
const A5:f32 = -0.06027397;
/// für 0..60:  22.36370;
const A4:f32 = 0.3727283;
/// für 0..60: -68.58285;
const A3:f32 = -1.1430475;
/// für 0..60: 110.3052;
const A2:f32 = 1.83842;
/// für 0..60: -84.19201;
const A1:f32 = -1.4032;
/// für 0..60:  23.49542;
const A0:f32 = 0.39159;

// pub type FlowRange = Range<f32>;
pub const INPUT:  &'static str     = "input";
pub const OUTPUT:  &'static str     = "output";
pub const CORELATION: &'static str = "corelation";
pub const REFERENCE: &'static str  = "reference";
pub const DEVIATION: &'static str  = "deviation";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Airflow {
    pub value:   f32,
}

impl Airflow {
    pub fn from_voltage(voltage:f32) -> Airflow{
        let broken = voltage > 1.0;
        let mut value = 60.0 * ((((((A6*voltage +A5)*voltage +A4) *voltage +A3) *voltage + A2) * voltage + A1) *voltage + A0);
        if broken {
            value = -1.0;
        }
        Airflow { value }
    }
    pub fn from_adc16(value:i16) -> Airflow{
        let voltage = (value as f32 / i16::max_value() as f32) * 5.0;
        Airflow::from_voltage(voltage)
    }
}



impl fmt::Display for Airflow {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f,"aiflow :{} {} ",self.value,if self.value<0.0 {"BROKEN"} else {"OK"})
    }
}

pub fn input(path:&Path) -> io::Result<Airflow> {
    let value = fs::read_to_string(path.join(INPUT)).unwrap_or(String::from("-1.0")).parse::<f32>().expect("airflow input value convert error");
   Ok(Airflow{value})
}

pub fn output(path:&Path) -> io::Result<Airflow> {
    let value = fs::read_to_string(path.join(OUTPUT)).unwrap_or(String::from("-1.0")).parse::<f32>().expect("airflow output value convert error");
   Ok(Airflow{value})
}