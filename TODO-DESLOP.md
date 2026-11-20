Prefix: DSLP
Next: 7

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

