//! Module for reading from stdin and adding requests to the context
//!

#![allow(unused)]

use crate::prelude::*;

#[derive(Default)]
struct TerminalState {
    raw_mode: bool,
}

impl TerminalState {
    fn leave_raw(&mut self) {
        if self.raw_mode {
            crossterm::terminal::disable_raw_mode().unwrap();
            self.raw_mode = false;
        } else {
            log::warn!("Attempt to leave raw mode when not in it");
        }
    }

    pub fn enter_raw(&mut self) {
        if !self.raw_mode {
            crossterm::terminal::enable_raw_mode().unwrap();
            self.raw_mode = true;
        } else {
            log::warn!("Attempt to enter raw mode when already in it");
        }
    }
}
#[derive(Default)]
pub struct StdinEventHandler {
    term: TerminalState,
    // reader: JoinHandle<()>,
    // rx: std::sync::mpsc::Receiver<E>,
}

impl StdinEventHandler {
    pub fn new() -> Self {
        // let han = std::thread::spawn({});
        let mut s = Self::default();
        s.term.enter_raw();
        s
    }

    pub fn exit(&mut self) {
        self.term.leave_raw();
        println!("Exiting!")
    }
}

impl Drop for StdinEventHandler {
    fn drop(&mut self) {
        self.exit()
    }
}
