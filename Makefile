configure:
	npm install @openapitools/openapi-generator-cli -g
	cargo install cargo-audit
	cargo install cargo-binutils
	cargo install grcov
	rustup toolchain add stable
	rustup component add llvm-tools-preview
	rustup default stable
	@echo "Installing pre-commit hook ..."
	cp pre-commit.sh .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit

api_codegen:
	openapi-generator-cli generate \
    -i api/openapi.yaml \
    -g rust-axum \
    -o openapi \
   --additional-properties=generateAliasAsModel=true,modelPackage=models,singleFile=true
	@find openapi -type f -name "*.rs" -exec sed -i '' 's/pub fn new<I, A, E>(api_impl: I) -> Router/pub fn new<I, A, E, C>(api_impl: I) -> Router/g' {} +
	@cargo fmt

build_container:
	docker buildx build --platform linux/arm64 -t orde-rs-service:latest .

vuln_scan:
	cargo audit

fmt:
	cargo fmt

lint:
	cargo fmt --check --verbose
	cargo clippy
	@rm -f ./*.profraw ./**/*.profraw

lint_fix:
	cargo fmt
	cargo clippy --fix --allow-dirty
	@rm -f ./*.profraw ./**/*.profraw

clean:
	cargo clean

cov:
		grcov . \
		--binary-path ./target/debug/deps/ \
		-s . \
		-t $(COVERAGE_FORMAT) \
		--branch \
		--ignore-not-existing \
		--ignore '../*' \
		--ignore "/*" \
		--ignore "openapi/*" \
		--ignore "probe/*" \
		--llvm-path "$(TOOLCHAIN_PATH)/lib/rustlib/$(RUST_TARGET)/bin" \
		-o $(COVERAGE_OUTPUT)

test:
	@command -v rustup >/dev/null 2>&1 || { echo "rustup is required but not installed" >&2; exit 1; }
	@rustup component list | grep -q "llvm-tools-preview (installed)" || rustup component add llvm-tools-preview
	@command -v grcov >/dev/null 2>&1 || { echo "grcov is required but not installed" >&2; exit 1; }
	$(eval TOOLCHAIN_PATH := $(shell rustup which rustc | sed 's/\/bin\/rustc//'))
	$(eval RUST_TARGET := $(shell rustup show active-toolchain | cut -d' ' -f1 | cut -d'-' -f2-))
	CARGO_INCREMENTAL=0 \
	RUSTFLAGS='-Cinstrument-coverage' \
	LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' \
	cargo test
	mkdir -p coverage
	@$(MAKE) cov TOOLCHAIN_PATH=$(TOOLCHAIN_PATH) RUST_TARGET=$(RUST_TARGET) COVERAGE_FORMAT=cobertura COVERAGE_OUTPUT=coverage/coverage.xml
	@$(MAKE) cov TOOLCHAIN_PATH=$(TOOLCHAIN_PATH) RUST_TARGET=$(RUST_TARGET) COVERAGE_FORMAT=html COVERAGE_OUTPUT=coverage/
	@rm -f ./*.profraw ./**/*.profraw

clean_test: clean test
