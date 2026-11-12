//! Tests for [`rustix::pipe`].

#![cfg(feature = "pipe")]

mod basic;
mod fcntl;
mod splice;
mod tee;
