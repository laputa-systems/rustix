## Verification

- `make test`: all-API Cargo tests; macOS warm run 2.51s through the first failing test binary (host busy; not a performance gate).
- `make lint`: Cargo Clippy for library and tests; macOS warm run 24.59s (host busy).
- Needs environment: on this APFS host `tests/fs/xattr.rs` xattr_basic and set fail because files carry `com.apple.provenance`; `tests/fs/seek.rs` test_seek_holes can return EOF instead of the expected sparse hole. These tests and their implementations match upstream. GitHub Ubuntu/ext4 is authoritative for the full suite.
- Rust public API and errno behavior: feature-specific integration suites in `tests/`; select: `tests/event/select.rs`; sockets: `tests/net/`; filesystem: `tests/fs/`; time: `tests/time/`. Other feature-specific modules use their corresponding suites.
- Cargo feature and platform surface: `tests/backends.rs`, `test-crates/`, and CI's target matrix; no CLI, owned file format, or network service.
- Consumers: e and tinyland; run their tests against this checkout when changing public API behavior.
- Performance: `benches/mod.rs` measures filesystem, clock and process syscall wrappers; no representative consumer workload is established.
- Components: `src/backend`, `src/net`, `src/fs`, `src/process`, `src/thread`, `src/io_uring`, other `src` modules, and `tests`.
