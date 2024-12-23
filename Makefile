configure:
	npm install @openapitools/openapi-generator-cli -g
	cargo install cargo-audit
	rustup component add clippy
	@echo "Installing pre-commit hook ..."
	cp pre-commit.sh .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit


api_codegen:
	openapi-generator-cli generate \
    -i api/openapi.yaml \
    -g rust-axum \
    -o openapi \
   --additional-properties=generateAliasAsModel=true,modelPackage=models,singleFile=true
	find openapi -type f -name "*.rs" -exec sed -i '' 's/pub fn new<I, A>(api_impl: I) -> Router/pub fn new<I, A, C>(api_impl: I) -> Router/g' {} +
	cargo fmt

build_container:
	docker buildx build --platform linux/arm64 -t orde-rs-service:latest .

vuln_scan:
	cargo audit

fmt:
	cargo fmt

lint:
	cargo fmt --check --verbose
	cargo clippy
