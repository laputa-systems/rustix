//! Network-related operations.

pub mod addr;
mod send_recv;
mod socket;
mod socket_addr_any;
#[cfg(not(any(target_os = "wasi")))]
mod socketpair;
mod types;

#[cfg(linux_kernel)]
pub mod netdevice;
pub mod sockopt;

pub use crate::maybe_polyfill::net::{
    IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6,
};
pub use send_recv::*;
pub use socket::*;
pub use socket_addr_any::SocketAddrAny;
pub(crate) use socket_addr_any::SocketAddrBuf;
#[cfg(not(any(target_os = "wasi")))]
pub use socketpair::socketpair;
pub use types::*;
