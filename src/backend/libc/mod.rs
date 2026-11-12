//! The libc backend.
//!
//! On Unix-family platforms, this uses the `libc` crate to make system calls.

// Every FFI call requires an unsafe block, and there are a lot of FFI
// calls. For now, set this to allow for the libc backend.
#![allow(clippy::undocumented_unsafe_blocks)]
// Lots of libc types vary between platforms, so we often need a `.into()` on
// one platform where it's redundant on another.
#![allow(clippy::useless_conversion)]

mod conv;

pub(crate) mod fd {
    pub use crate::maybe_polyfill::os::fd::*;
    #[allow(unused_imports)]
    pub(crate) use RawFd as LibcFd;
}

pub(crate) mod c;

#[cfg(feature = "event")]
pub(crate) mod event;
#[cfg(feature = "fs")]
pub(crate) mod fs;
pub(crate) mod io;
#[cfg(all(linux_kernel, not(target_os = "android")))]
#[cfg(feature = "io_uring")]
pub(crate) mod io_uring;
#[cfg(not(any(
    target_os = "espidf",
    target_os = "horizon",
    target_os = "vita",
    target_os = "wasi"
)))]
#[cfg(feature = "mm")]
pub(crate) mod mm;
#[cfg(linux_kernel)]
#[cfg(feature = "mount")]
pub(crate) mod mount;
#[cfg(not(target_os = "wasi"))]
#[cfg(feature = "net")]
pub(crate) mod net;
#[cfg(not(any(target_os = "espidf")))]
#[cfg(any(
    feature = "param",
    feature = "runtime",
    feature = "time",
    target_arch = "x86",
))]
pub(crate) mod param;
#[cfg(feature = "pipe")]
pub(crate) mod pipe;
#[cfg(feature = "process")]
pub(crate) mod process;
#[cfg(not(target_os = "wasi"))]
#[cfg(feature = "pty")]
pub(crate) mod pty;
#[cfg(feature = "rand")]
pub(crate) mod rand;
#[cfg(not(target_os = "wasi"))]
#[cfg(feature = "system")]
pub(crate) mod system;
#[cfg(not(any(target_os = "horizon", target_os = "vita")))]
#[cfg(feature = "termios")]
pub(crate) mod termios;
#[cfg(feature = "thread")]
pub(crate) mod thread;
#[cfg(not(any(target_os = "espidf")))]
#[cfg(feature = "time")]
pub(crate) mod time;

/// If the host libc is glibc, return `true` if it is less than version 2.25.
///
/// To restate and clarify, this function returning true does not mean the libc
/// is glibc just that if it is glibc, it is less than version 2.25.
///
/// For now, this function is only available on Linux, but if it ends up being
/// used beyond that, this could be changed to e.g. `#[cfg(unix)]`.
#[cfg(all(unix, target_env = "gnu"))]
pub(crate) fn if_glibc_is_less_than_2_25() -> bool {
    // This is also defined inside `weak_or_syscall!` in
    // backend/libc/rand/syscalls.rs, but it's not convenient to re-export the
    // weak symbol from that macro, so we duplicate it at a small cost here.
    weak! { fn getrandom(*mut c::c_void, c::size_t, c::c_uint) -> c::ssize_t }

    // glibc 2.25 has `getrandom`, which is how we satisfy the API contract of
    // this function. But, there are likely other libc versions which have it.
    getrandom.get().is_none()
}

// Private modules used by multiple public modules.
#[cfg(any(feature = "process", feature = "runtime"))]
#[cfg(not(any(target_os = "wasi")))]
pub(crate) mod pid;
#[cfg(any(feature = "process", feature = "thread"))]
#[cfg(linux_kernel)]
pub(crate) mod prctl;
#[cfg(not(any(
    target_os = "android",
    target_os = "espidf",
    target_os = "horizon",
    target_os = "vita",
    target_os = "wasi"
)))]
#[cfg(feature = "shm")]
pub(crate) mod shm;
#[cfg(any(feature = "fs", feature = "thread", feature = "process"))]
#[cfg(not(any(target_os = "wasi")))]
pub(crate) mod ugid;

#[cfg(bsd)]
const MAX_IOV: usize = c::IOV_MAX as usize;

#[cfg(any(linux_kernel, target_os = "emscripten", target_os = "nto"))]
const MAX_IOV: usize = c::UIO_MAXIOV as usize;

#[cfg(not(any(
    bsd,
    linux_kernel,
    target_os = "emscripten",
    target_os = "espidf",
    target_os = "nto",
    target_os = "horizon",
)))]
const MAX_IOV: usize = 16; // The minimum value required by POSIX.
