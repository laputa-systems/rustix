## Verification

- `make clean` removes the checkout’s Cargo target directory and refuses a symlink.
- `make test`: all-API Cargo tests; macOS warm run 2.51s through the first failing test binary (host busy; not a performance gate).
- `make lint`: Cargo Clippy for library and tests; macOS warm run 24.59s (host busy).
- Needs environment: on this APFS host `tests/fs/xattr.rs` xattr_basic and set fail because files carry `com.apple.provenance`; `tests/fs/seek.rs` test_seek_holes can return EOF instead of the expected sparse hole. These tests and their implementations match upstream. GitHub Ubuntu/ext4 is authoritative for the full suite.
- Rust public API and errno behavior: feature-specific integration suites in `tests/`; select: `tests/event/select.rs`; sockets: `tests/net/`; filesystem: `tests/fs/`; time: `tests/time/`. Other feature-specific modules use their corresponding suites.
- Cargo feature and platform surface: `tests/backends.rs`, `test-crates/`, and CI's target matrix; no CLI, owned file format, or network service.
- Consumers: e and tinyland; run their tests against this checkout when changing public API behavior.
- Performance: `benches/mod.rs` measures filesystem, clock and process syscall wrappers; no representative consumer workload is established.
- Components: `src/backend`, `src/net`, `src/fs`, `src/process`, `src/thread`, `src/io_uring`, other `src` modules, and `tests`.
- Test tiers: Edit `cargo test --features all-apis --test net sockopt::test_sockopts_ipv4 -- --exact` (cost unmeasured); Change `make test` (full all-API suite, locally stops at the host filesystem failures); Full is the CI platform and feature matrix, manual/hosted only.
- Supplemental macOS check: `cargo test --workspace --features all-apis -- --skip xattr::xattr_basic --skip xattr::set --skip seek::test_seek_holes` passed in 104.71s on a busy host; the excluded contracts still require the full suite on ext4.
- Needs environment: `cargo-llvm-cov` and `cargo-mutants` are unavailable; hand-injected faults replace mutation tooling for the scoped critical boundary.
- Benchmarks: `make bench-list`, `make bench-check`, `make bench-dry-run BENCH_CASE=getpid`, `make bench-edit`, and `make bench BENCH_CASE=getpid`; choices are statat, fstat, clock, getpid. Full suite: `make bench-full`, manual only. Cargo, the pinned Rust toolchain, and Criterion's existing Cargo dependency are required. Change also needs `jaq` and checks the sample count and estimates for every expected result. Logs and isolated Criterion results go under `.work/bench/`. Costs and representative-workload performance verdicts remain unmeasured on a quiet host.
