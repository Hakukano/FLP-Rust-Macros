.PHONY: usage clean audit lint test

clean:
	rm -rf $(OUT_DIRECTORY)
	mkdir -p $(OUT_DIRECTORY)

audit:
	cargo deny check ban

lint:
	cargo clippy -- -D warnings

test:
	cargo test -- --nocapture

