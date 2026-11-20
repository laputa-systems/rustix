Prefix: DSLP
Next: 10

Deslop campaign backlog ([plan](https://github.com/laputa-systems/stuff/blob/main/docs/campaigns/deslop.md), tier E). One agent works this file with the `deslop` skill: take the lowest-numbered open ticket whose area does not start with `Needs owner ·`. After DSLP-5, work the open P1 and P2 findings, then fill in `## Report` below.

## Report

- Base: campaign seed `8fdf95b`; admission `1fb51827a51cd7bc081b3f802881cc482b45cdf8`; upstream merge base `ffa6ee556f54ef646c5cfc67afc72703ae3a7382` (`upstream/main`). Scope is fork changes only. CI minutes: 0 locally; parent runs hosted CI.
- Scorecard: Rust source 71,218 and test 14,677 lines before and after (`wc -l` over `src/**/*.rs` and `tests/**/*.rs`); escape-hatch occurrences in `src` 2,049 before and after. No source lines or dependencies removed in this pass. Warm `make test` before 2.51s and later 2.63s through the first failing binary, on a busy host; this is not a suite performance verdict. Line and branch coverage: Needs environment, cargo-llvm-cov unavailable. Automated mutants caught/missed/timeout/unviable: unmeasured, cargo-mutants unavailable. Hand faults: 2 caught, 0 missed in the two-case socket-timeout scope; broader mutation and the Windows-target boundary need environment. No escaped regressions established. Perf gate deferred: busy host; no runtime performance claim is made.
- Ground: added all-API `make test` and pull-request invocation; lint retained. Four-core macOS test 2.51s, 94 passed before the fs binary stopped with 2 named xattr failures and 3 ignored; lint 24.59s exit 0. Initial cold test took 125.17s and also hit the intermittent sparse-hole case. Needs environment: APFS provenance attributes cause `tests/fs/xattr.rs` xattr_basic and set to fail; APFS can report EOF for `tests/fs/seek.rs` test_seek_holes. Both tests and implementations match upstream. Parent will verify the complete suite on hosted Ubuntu/ext4. Local CI workflow validator unavailable.
- Diagnose: reviewed fork changes and filed P3 DSLP-6; no P1/P2 finding established. Test 3.06s with the same xattr failures; lint 24.79s exit 0. Cross-platform and hosted consumer checks need environment.
- Types: adopted 0 settings. Rejected unsafe_code (254 diagnostics), cast_possible_truncation (201), cast_sign_loss (61), unwrap_used (23), expect_used (3), needless_pass_by_value (3), wildcard_enum_match_arm (0), match_wildcard_for_single_variants (0), redundant_clone (0): full-library macOS all-API warning counts, not fork-only defects. Unsafe syscall bindings and inherited conversions account for warnings; zero-warning settings lacked a defect or escape-hatch witness. Profile command 42.72s exit 0, 549 library warnings including other defaults; no contract changed. Test 26.66s with the same xattr failures; lint 24.40s exit 0.
- Tests: supplemental all-API run excluding only the three host-sensitive cases passed 243 tests, ignored 3 and filtered 3, in 104.71s; it does not replace hosted full-suite proof. In the fork-touched libc timeout return path, inverted zero detection and discarded nonzero seconds each made `sockopt::test_sockopts_ipv4` fail at the corresponding assertion; restored controls passed. Fault commands took 43.85s and 43.25s, restored controls 18.75s and 18.22s. No new tests; Windows compile-error boundary needs an installed Windows target. Final step test 73.36s with the same xattr failures; lint 24.98s exit 0.
- Dead code: `cargo rustc --lib --features all-apis -- -W dead_code` exit 0 in 62.27s with no dead-code diagnostic on macOS; no further safe fork-only removal established. cargo-machete, udeps and non-host platform scans need environment. Test 2.63s with the same xattr failures; lint 25.03s exit 0.
- Bench: the existing harness covers statat, fstat, clock and getpid microbenchmarks, not representative consumer throughput, startup, allocations, memory or artifact size; attribution and checked results are gaps DSLP-8 and DSLP-9. Added list, preflight, dry-run, Edit, Change and manual Full entry points. Edit passed ten cases (cold 321.10s; final warm 24.78s); Change getpid passed in 0.96s. Empty selection and missing cargo fail preflight; isolated results prevent stale Criterion comparisons. Full not run. All timings informational on a busy host. Test 10.45s with the same xattr failures; lint 24.37s exit 0.
- Report: cumulative review of the admission-to-candidate diff found the stale Criterion comparison described below; it was fixed in the Bench step. Report check: `make test` 26.89s with the same xattr failures and `make lint` 24.39s exit 0; final-commit checks follow.
- Findings: 0 P1/P2 fixed or open; P3 DSLP-6, DSLP-8 and DSLP-9 remain. No owner walk-back proposed. Consumer tests and workload performance gates unaffected by documentary, test-entry and benchmark-runner changes.
- Method feedback: running beyond the first failing binary with explicit host-case exclusions exposed the rest of the suite without weakening `make test`. The first Criterion wrapper reused old samples and emitted a misleading regression; per-run output isolation corrected it. Needs environment: mirrored coverage and mutation tools, macOS filesystem-independent upstream tests, hosted ext4 CI, Windows target, and dependency scanners; these require no shared-file edits in this worktree.

## Tickets

### DSLP-6 · P3 · Point fork badges at the fork package and CI

- **Area:** documentation
- **Where:** README.md:8-14
- **Problem:** The fork README identifies the Windows-free fork, but its CI, crate and API badges still link to bytecodealliance/rustix and the upstream rustix package. Readers following those badges see upstream status and documentation rather than this fork.
- **Recommendation:** Choose fork-specific status and package links and update the badge destinations when authorized to edit the inherited lines.
- **Fix with:** technical-writing
- **Effort:** S

### DSLP-8 · P3 · Measure representative consumer workloads

- **Area:** performance
- **Where:** benches/mod.rs:50
- **Problem:** The benchmarks time individual stat, fstat, clock and getpid calls; they do not measure a consumer journey, allocations, binary size or end-to-end throughput. A wrapper regression in a real workload can escape this microbenchmark set.
- **Recommendation:** Choose a representative consumer workload with a correctness oracle and resource measurements, then add a bounded comparison and attribution path.
- **Fix with:** refactor-benchmarks
- **Effort:** M

### DSLP-9 · P3 · Keep timed syscall results observable

- **Area:** performance
- **Where:** benches/mod.rs:135-175
- **Problem:** The clock and getpid benchmark loops discard their results without black-boxing or a correctness oracle, so optimizer elimination or hoisting can make the timing unrepresentative. The busy-host getpid run reported sub-nanosecond estimates; that run is not a performance verdict.
- **Recommendation:** Use observable inputs and results, verify completed work, and inspect the compiled workload before using it for comparisons.
- **Fix with:** refactor-benchmarks
- **Effort:** S

