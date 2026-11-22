#!/usr/bin/env bash
set -euo pipefail

mode=${1:-}
workload=${2:-}
case "$mode" in
  list)
    test "$#" -eq 1
    printf '%s\n' 'statat: filesystem metadata' 'fstat: descriptor metadata' 'clock: monotonic clock' 'getpid: process identity'
    exit 0
    ;;
  check|edit|full)
    test "$#" -eq 1 || { printf '%s\n' 'unexpected workload' >&2; exit 2; }
    ;;
  change|dry-run)
    test "$#" -eq 2 || { printf '%s\n' 'choose a workload: statat, fstat, clock, getpid' >&2; exit 2; }
    case "$workload" in
      statat) filter='simple statat' ;;
      fstat) filter='simple fstat' ;;
      clock) filter='simple clock_gettime' ;;
      getpid) filter='simple getpid' ;;
      *) printf 'unknown workload: %s\n' "$workload" >&2; exit 2 ;;
    esac
    ;;
  *) printf '%s\n' 'usage: run.sh list|check|edit|full|change WORKLOAD|dry-run WORKLOAD' >&2; exit 2 ;;
esac

samples=10
command=(cargo bench --bench mod --features fs,time,process,stdio --)
case "$mode" in
  edit) command+=(--test) ;;
  change|dry-run) command+=("$filter" --sample-size "$samples" --warm-up-time 0.1 --measurement-time 0.1 --noplot) ;;
esac
if [[ "$mode" == dry-run ]]; then
  printf 'RUSTFLAGS=--cfg=criterion'; printf ' %q' "${command[@]}"; printf '\n'
  exit 0
fi
command -v cargo >/dev/null || { printf '%s\n' 'cargo is missing; install the pinned toolchain before running benchmarks.' >&2; exit 1; }
command -v rustc >/dev/null || { printf '%s\n' 'rustc is missing; install the pinned toolchain before running benchmarks.' >&2; exit 1; }
if [[ "$mode" == check || "$mode" == change ]]; then
  command -v jaq >/dev/null || { printf '%s\n' 'jaq is required to verify Criterion results.' >&2; exit 1; }
fi
cargo --version
rustc --version
df -Pk .
if command -v perf-gate-ready >/dev/null; then
  perf-gate-ready || printf '%s\n' 'Host busy: timings are informational.'
else
  printf '%s\n' 'Host readiness unavailable: timings are informational.'
fi
[[ "$mode" == check ]] && exit 0
mkdir -p .work/bench
run_dir=$(mktemp -d "$PWD/.work/bench/run.XXXXXX")
printf 'Mode: %s; workload: %s; output: %s\n' "$mode" "${workload:-all}" "$run_dir"
printf '%s\n' 'Completion alone is not a performance verdict.'
export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--cfg=criterion"
export CRITERION_HOME="$run_dir/criterion"
started=$(date +%s)
set +e
"${command[@]}" 2>&1 | tee "$run_dir/output.log"
statuses=("${PIPESTATUS[@]}")
result=${statuses[0]}
set -e
if (( statuses[1] != 0 )); then
  printf '%s\n' 'Could not write the benchmark log.' >&2
  result=1
fi
if [[ "$mode" == change && "$result" -eq 0 ]]; then
  expected=("$filter" "$filter libc")
  if [[ "$workload" == statat ]]; then
    expected+=("$filter cstr" "$filter libc cstr")
  fi
  for benchmark in "${expected[@]}"; do
    data="$CRITERION_HOME/$benchmark/new"
    if ! jaq -e --argjson samples "$samples" '
      (.iters | (length == $samples) and all(.[]; type == "number" and . > 0)) and
      (.times | (length == $samples) and all(.[]; type == "number" and . >= 0))
    ' "$data/sample.json" >/dev/null || ! jaq -e '
      .median.point_estimate | type == "number" and . >= 0
    ' "$data/estimates.json" >/dev/null; then
      printf 'Missing or incomplete Criterion measurements: %s\n' "$benchmark" >&2
      result=1
    fi
  done
fi
printf 'exit=%s elapsed_seconds=%s timing=informational\n' "$result" "$(($(date +%s) - started))" | tee "$run_dir/status"
exit "$result"
