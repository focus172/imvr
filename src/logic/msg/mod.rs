use crate::prelude::*;

// mod event;
mod key;
mod source;
mod terminal;
mod window;

use serde::Deserialize;
use tokio::sync::oneshot;
use std::os::fd::RawFd;
use std::path::PathBuf;

pub use self::source::{EventHandler, EventSendError};
pub use self::{terminal::TerminalMsg, window::WindowMsg};
use super::SurfaceId;

#[derive(Deserialize)]
pub enum Msg {
    ShowImage { path: PathBuf, id: SurfaceId },
    OpenWindow { resp: Option<ReturnAddress> },
}

impl Msg {
    #[inline]
    pub fn open(sender: oneshot::Sender<u64>) -> Self {
        Self::OpenWindow {
            resp: Some(ReturnAddress::Memory(sender)),
        }
    }
}

#[derive(Deserialize)]
#[serde(from = "RawFd")]
pub enum ReturnAddress {
    Memory(oneshot::Sender<u64>),
    File(RawFd),
}

impl fmt::Debug for ReturnAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Memory(_) => f
                .debug_tuple("Memory")
                .field(&"oneshot::Sender<u64>")
                .finish(),
            Self::File(fd) => f.debug_tuple("File").field(fd).finish(),
        }
    }
}

impl ReturnAddress {
    pub fn send(self, value: u64) -> Result<(), ReturnerError> {
        match self {
            ReturnAddress::Memory(s) => s
                .send(value)
                .map_err(|_| Report::new(ReturnerError::SenderError)),

            // FIXME: unimplemented
            ReturnAddress::File(f) => Err(Report::new(ReturnerError::FileError(f))),
        }
    }
}

impl From<RawFd> for ReturnAddress {
    fn from(value: RawFd) -> Self {
        Self::File(value)
    }
}

#[derive(Debug)]
pub enum ReturnerError {
    SenderError,
    FileError(RawFd),
}

impl fmt::Display for ReturnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SenderError => {
                f.write_str("Data was either already sent on this channel or consumer hung up")
            }
            Self::FileError(fd) => {
                write!(f, "Failed to write data to specified fd: {fd:?}")
            }
        }
    }
}

impl Context for ReturnerError {}
