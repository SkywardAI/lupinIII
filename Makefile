###########################################################
# cargo family

.PHONY: check
check:
	@cargo check

.PHONY: clippy
clippy:
	@cargo clippy

.PHONY: build
build:
	@cargo build