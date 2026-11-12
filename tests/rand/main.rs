//! Tests for [`rustix::rand`].

#![cfg(feature = "rand")]

#[cfg(linux_kernel)]
mod getrandom;
