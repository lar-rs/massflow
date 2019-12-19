
//! Echoes lines read on stdin to stdout.

#[macro_use]
extern crate lazy_static;
#[macro_use(block)]
extern crate nb;

use ads1x1x::{channel, Ads1x1x, SlaveAddr};
use embedded_hal::adc::OneShot;
use linux_embedded_hal::I2cdev;

// use async_std::io;
// use async_std::task;
// use failure::ResultExt;
// use std::fs;
// use std::fs::File;
// use std::io::LineWriter;
use std::io;
use std::io::prelude::*;
use std::path::{PathBuf,Path};
// use structopt::StructOpt;
// use clap_flags::Log;
// use std::process;
// use std::stream;
// use std::prelude::*;
use atty::Stream;
use crossbeam::channel::{bounded, tick, Receiver, select};

use std::time::Duration;
// use regex::{Regex,RegexSetBuilder};
// use lazy_static::lazy_static;
// use std::time::Duration;
// use std::str::FromStr;
// use std::time::SystemTime;
// use async_std::println;
// use std::collections::HashSet;
 
use massflow::*;





fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

pub fn watch_flow(workdir:&Path,watch:&cli::Watch) -> io::Result<()>{
    let mut mflow = Massflow::create(workdir);
    let ctrl_c_events = ctrl_channel().expect("create ctrl c signal failed");
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = stdout.lock(); // acquire a lock on it
    let ticks = tick(watch.interval());
    let dev = I2cdev::new("/dev/i2c-1").expect(&format!("I2C device /dev/i2c-1 open failed"));
    let address = SlaveAddr::default();
    let mut adc = Ads1x1x::new_ads1115(dev, address);
    let value = block!(adc.read(&mut channel::DifferentialA0A1)).unwrap();
    println!("Measurement: {}", value);
    loop {
        select! {
            recv(ticks) -> _ => {
                println!("working!");
                writeln!(handle, "foo: {}", 42)?; // add `?` if you care about errors here
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Goodbye!");
                break;
            }
        }
    }

    let _= adc.destroy_ads1115();
    Ok(())
}

#[paw::main]
fn main(args: Args) -> io::Result<()> {
    // println!("{}",Paint::blue(can::banner::NAME));
    femme::start(log::LevelFilter::Trace).unwrap();
    match args.cmd() {
        Cmd::Watch(watch) => watch_flow(args.workdir(),watch)?,
        Cmd::Set(setting) => setting.save(args.workdir())?,
    };
    Ok(())
}


