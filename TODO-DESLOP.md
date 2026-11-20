Prefix: DSLP
Next: 10

Deslop campaign backlog ([plan](https://github.com/laputa-systems/stuff/blob/main/docs/campaigns/deslop.md), tier E). One agent works this file with the `deslop` skill: take the lowest-numbered open ticket whose area does not start with `Needs owner ·`. After DSLP-5, work the open P1 and P2 findings, then fill in `## Report` below.

## Report

Not started.

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

