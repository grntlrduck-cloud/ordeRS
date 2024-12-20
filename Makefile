configure:
	npm install @openapitools/openapi-generator-cli -g

api_codegen:
	openapi-generator-cli generate \
    -i api/openapi.yaml \
    -g rust-axum \
    -o openapi \
   --additional-properties=generateAliasAsModel=true,modelPackage=models,singleFile=true

build_container:
	docker buildx build --platform linux/arm64 -t orde-rs-service:latest . 
