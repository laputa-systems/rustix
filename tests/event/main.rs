//! Tests for [`rustix::event`].

#![cfg(feature = "event")]

#[cfg(not(feature = "rustc-dep-of-std"))] // TODO
#[cfg(feature = "net")]
#[cfg(any(linux_kernel, target_os = "illumos", target_os = "redox"))]
mod epoll;
#[cfg(not(feature = "rustc-dep-of-std"))] // TODO
#[cfg(feature = "net")]
#[cfg(any(linux_kernel, target_os = "illumos", target_os = "redox"))]
mod epoll_timeout;
#[cfg(not(target_os = "wasi"))]
mod eventfd;
mod poll;
#[cfg(solarish)]
mod port;
#[cfg(any(bsd, linux_kernel, target_os = "wasi"))]
mod select;
