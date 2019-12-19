use std::fs;
use std::io;
use serde::{Deserialize, Serialize};
use std::path::{Path};

#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct AirflowSetting {
    pub level:           f32,
    pub deviation:       f32,
    pub cal_level:       f32, 
}



impl Default for AirflowSetting {
    fn default() -> Self{
        Self{
            level: 30.0,
            cal_level: 30.0,
            deviation: 5.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HumiditySetting {
    pub warn_level: f32,
    pub crit_level: f32,
    pub scale: f32,
}

impl Default for HumiditySetting {
    fn default() -> Self{
        Self{
            warn_level: 30.0,
            crit_level:  30.0,
            scale: 1.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PressureSetting {
    pub warn_level: f32,
    pub crit_level: f32,
    pub scale: f32,
}

impl Default for PressureSetting {
    fn default() -> Self{
        Self{
            warn_level: 300.0,
            crit_level:  600.0,
            scale: 1.0,
        }
    }
}


#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct Config {
    pub input:    AirflowSetting,
    pub output:   AirflowSetting,
    pub correlation: f32,
    pub humidity: HumiditySetting,
    pub pressure: PressureSetting,
}

impl Default for Config {
    fn default() -> Self{
        Self{
            input: AirflowSetting::default(),
            output: AirflowSetting::default(),
            correlation: 1.0,
            humidity: HumiditySetting::default(),
            pressure: PressureSetting::default(),
        }
    }
}

impl Config {
    pub fn read(path:&Path)-> io::Result<Config> {
        let toml_str = fs::read_to_string(path.join("config"))?;
        if let Ok(conf) = toml::from_str(&toml_str) {
            Ok(conf)
        }else {
            Ok(Config::default())
        }
    }
}


