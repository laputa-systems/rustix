publish: ## Publish laputa-rustix to the stuff crates registry
	stuff publish cargo -p laputa-rustix

publish-dry-run: ## Check the laputa-rustix publish without uploading or promoting
	stuff publish cargo -p laputa-rustix --dry-run

.PHONY: test lint clean bench-check bench-list bench-edit bench-change bench-dry-run bench bench-full

test:
	cargo test --workspace --features all-apis

lint:
	cargo clippy --lib --tests

clean:
	@if test -L ./target || { test -e ./target && ! test -d ./target; }; then \
	  printf '%s\n' 'make clean: ./target must be a local build directory' >&2; exit 1; \
	fi
	rm -rf -- ./target

BENCH_CASE ?= getpid

bench-check:
	bash benches/run.sh check

bench-list:
	bash benches/run.sh list

bench-edit:
	bash benches/run.sh edit

bench-change:
	bash benches/run.sh change "$(BENCH_CASE)"

bench-dry-run:
	bash benches/run.sh dry-run "$(BENCH_CASE)"

bench: bench-change

bench-full:
	bash benches/run.sh full
