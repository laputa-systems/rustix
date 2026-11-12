//! Tests for [`rustix::io`].

mod close;
mod dup;
mod error;
mod from_into;
#[cfg(not(target_os = "redox"))]
mod ioctl;
#[cfg(not(target_os = "redox"))] // redox doesn't have cwd/openat
mod read_write;
