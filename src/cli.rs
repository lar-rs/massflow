// use failure::ResultExt;
use std::io;
use std::fs;
// use std::io;
use std::path::{PathBuf,Path};
use structopt::StructOpt;
use std::time::Duration;

/// select driver 
#[derive(Debug, StructOpt)]
pub enum Driver{
    #[structopt(name = "simulate", about = "📢 start simulate airflow ")]
    Simulate,
    #[structopt(name = "dbus", about = "📢 start dbus airflow ")]
    DBus{
        ///🔌 hardware connection address
        #[structopt(short = "a", long = "address",  default_value = "tcp:host=192.168.66.59,port=6666")]
        address: String,
    }
}

/// ✇ watch airflow
#[derive(Debug, StructOpt)]
pub struct Watch {
    /// ⏱  interval in seconds
    #[structopt(short = "i", long = "interval",  default_value = "2")]
    interval: u64,
}

/// watch implementation
impl Watch {
     /// Interval in milli seconds
     #[inline]
     pub fn interval(&self) -> Duration {
         Duration::from_millis(self.interval)
     }
}
/// ✇ average signal
#[derive(Debug, StructOpt)]
pub struct Set {
   
    ///🔧 average counter
    #[structopt(short = "c", long = "counter",  default_value = "20")]
    counter: usize,
   
}

impl Set {
    pub fn save(&self, path: &Path) -> io::Result<()> {
        if ! path.is_dir() {
            fs::create_dir_all(path)?;
        }
        // fs::write(path.join("counter"),format!("{}",self.counter).as_bytes())?;
        // fs::write(path.join("interval"),format!("{}",self.interval).as_bytes())?;
        Ok(())
    }
}

/// 📢 subcommands 
#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(name = "watch", about = "📢 start watch a airflow ")]
    Watch(Watch),
    #[structopt(name = "setup", about = "📢 subcommand to setup sensor parameter")]
    Set(Set)
}



/// massflow sensor command arguments
#[derive(Debug, StructOpt)]
#[structopt(name = "massflow", about = "🧰massflow sensor command interface")]
pub struct Args {
    ///🔧 working directory 
    #[structopt(long = "workdir",  default_value = ".")]
    workdir: PathBuf,
    ///🔌 hardware interface
    #[structopt(long = "interface",  default_value = "/dev/i2c-1")]
    interface: String,
    ///📢 subcommand 
    #[structopt(subcommand, about = "📢 subcommand controller ")]
    cmd:Cmd,
}

/// 🔧 Activate debug mode 
impl Args {
    /// Workdir
    #[inline]
    pub fn workdir(&self) -> &Path {
        &self.workdir
    }
    /// interface
    #[inline]
    pub fn interface(&self) -> &str {
        &self.interface
    }
    /// Access the directory name.
    #[inline]
    pub fn cmd(&self) -> &Cmd {
        &self.cmd
    }

}

