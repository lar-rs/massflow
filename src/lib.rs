
pub mod banner;
pub mod cli;
pub mod config;
pub mod airflow;
pub mod humidity;
pub mod pressure;


pub use self::airflow::Airflow;
pub use self::humidity::Humidity;
pub use self::pressure::Pressure;
pub use self::cli::{Args,Watch,Set,Cmd};
pub use self::config::Config;

use ads1x1x::{channel, Ads1x1x, SlaveAddr};
use core::marker::PhantomData;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use linux_embedded_hal::I2cdev;
use serde::{Serialize,Deserialize};
use std::path::{Path,PathBuf};
use std::io;
// type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Level{
    Normal,
    Warning,
    Critical,
    Brocket
}

pub struct Massflow {
    path: PathBuf,
    cfg:Config,
    // tic: u64,
    // block: u64,
}




impl Massflow{
    pub fn create(path:&Path) ->  io::Result<Massflow> {
        let path = path.to_path_buf();
        let cfg:Config = Config::read(&path)?;
        Ok(Massflow{path,cfg})
        // let mut input    = Airflow::from_adc16(0);
        // let mut output   = Airflow::from_adc16(0);
        // let mut humidity = Humidity::from_adc16(0);
        // let mut pressure = Humidity::from_adc16(0);

        // for (channel, value) in values.iter().enumerate() {
        //     println!("Channel {}: {}", channel, value);
        //     match channel {
        //         channel::SingleA0 => input     = Airflow::from_adc16(value),
        //         channel::SingleA1 => output    = Airflow::from_adc16(value),
        //         channel::SingleA2 => humidity  = Humidity::from_adc16(value),
        //         channel::SingleA3 => pressure  = Pressure::from_adc16(value),
        //     }
        // }
        
        // Massflow{ adc,cfg,input,output,humidity,pressure }
    }
    /// Returns volume rate on input reported by sensor.
    pub fn airflow_input(&self) -> io::Result<Airflow> {
        airflow::input(&self.path)
    }
    /// Returns volume rate on input reported by sensor.
    pub fn airflow_output(&self) -> io::Result<Airflow> {
        airflow::output(&self.path)
    }

    /// Returns current presure reported by sensor.
    pub fn pressure(&self) -> io::Result<Pressure> {
        pressure::pressure(&self.path)
    }
    /// Returns current presure reported by sensor.
    pub fn humidity(&self) -> io::Result<Humidity> {
        humidity::humidity(&self.path)
    }
    // pub fn warning_pressure(&self) -> Result<u32> {
        // let warn = fs::read_to_string(self.path.join("warning_pressure"))?.parse::<u32>()?;
        // Ok(warn)
    // }
    // pub fn critical_pressure(&self) -> Result<u32> {
        // let critical = fs::read_to_string(self.path.join("critical_pressure"))?.parse::<u32>()?;
        // Ok(critical)
    // }
    // pub fn update_airflow(&mut self, input:Airflow,output:Airflow) -> io::Result<()>{
        // Ok(())
    // }
    // pub fn update_humidity(&mut self, humidity:Humidity) -> io::Result<()> {
        // Ok(())
    // }
    // pub fn update_pressure(&mut self, humidity:Pressure) -> io::Result<()> {
        // Ok(())
    // }
}


