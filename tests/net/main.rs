//! Tests for [`rustix::net`].

#![cfg(feature = "net")]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg(not(target_os = "wasi"))]
#![cfg_attr(core_c_str, feature(core_c_str))]

mod addr;
#[cfg(all(unix, not(target_os = "redox")))]
mod cmsg;
mod connect_bind_send;
mod dgram;
#[cfg(linux_kernel)]
mod netlink;
#[cfg(feature = "event")]
mod poll;
#[cfg(unix)]
mod recv_trunc;
mod sockopt;
#[cfg(unix)]
mod unix;
#[cfg(unix)]
mod unix_alloc;
mod v4;
mod v6;
