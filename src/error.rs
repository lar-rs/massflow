#![allow(unused_variables)]

use failure::{Fail};
use std::io;
// use std::io::{Error, ErrorKind};

// use std::string::FromUtf8Error;

// use jsonrpc_core::Error as RpcError;
// use mut_guard::*;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "io error - {}",err)]
    IOError {err: io::Error },

    // #[fail(display = "rpc error - {}",err)]
    // RpcError {err: RpcError },
    #[fail(display = "read config - {}",err)]
    ReadConfig {err: io::Error},

    #[fail(display = "socket can error - {}", msg)]
    Convert { msg: String },

    #[fail(display = "data converd error - {}", msg)]
    DataConvert{ msg: String },

}


pub fn wrong_data(msg:String) -> Error {
    Error::DataErr{ msg }
}

impl From<io::Error> for Error {
    fn from(kind:io::Error) -> CanError {
        CanError::IOError{err: kind}
    }
}

impl std::convert::From<std::num::ParseIntError> for CanError {
    fn from(e: std::num::ParseIntError) -> Self {
        CanError::DataConvert{msg : format!("{}",e)}
    }
}