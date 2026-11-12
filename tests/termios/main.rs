//! Tests for [`rustix::termios`].

#![cfg(feature = "termios")]

mod isatty;
#[cfg(not(any(target_os = "wasi")))]
mod pgrp;
#[cfg(not(any(target_os = "wasi")))]
mod sid;
#[cfg(feature = "pty")]
mod termios;
#[cfg(feature = "fs")]
#[cfg(not(any(target_os = "fuchsia")))]
mod ttyname;
