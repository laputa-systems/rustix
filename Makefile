publish: ## Publish laputa-rustix to the stuff crates registry
	stuff publish cargo -p laputa-rustix

publish-dry-run: ## Check the laputa-rustix publish without uploading or promoting
	stuff publish cargo -p laputa-rustix --dry-run

.PHONY: lint clean

lint:
	cargo clippy --lib --tests --locked

clean:
	rm -rf target
