publish: ## Publish laputa-rustix to the stuff crates registry
	stuff publish cargo -p laputa-rustix

publish-dry-run: ## Check the laputa-rustix publish without uploading or promoting
	stuff publish cargo -p laputa-rustix --dry-run

.PHONY: test lint clean

test:
	cargo test --workspace --features all-apis

lint:
	cargo clippy --lib --tests

clean:
	find . -path './.git' -prune -o -path './.work' -prune -o -name target -type d -prune -exec rm -rf {} +
