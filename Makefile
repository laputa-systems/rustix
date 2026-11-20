publish: ## Publish laputa-rustix to the stuff crates registry
	stuff publish cargo -p laputa-rustix

publish-dry-run: ## Check the laputa-rustix publish without uploading or promoting
	stuff publish cargo -p laputa-rustix --dry-run

.PHONY: test lint clean bench-check bench-list bench-edit bench-change bench-dry-run bench

test:
	cargo test --workspace --features all-apis

lint:
	cargo clippy --lib --tests

clean:
	find . -path './.git' -prune -o -path './.work' -prune -o -name target -type d -prune -exec rm -rf {} +

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

bench:
	bash benches/run.sh full
