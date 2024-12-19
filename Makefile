configure:
	npm install @openapitools/openapi-generator-cli -g

api_codegen:
	openapi-generator-cli generate \
    -i api/openapi.yaml \
    -g rust \
    -o temp_generated \
    --global-property models,modelDocs=false,apis=false
	mkdir -p src/adapters/rest/models && \
  mv temp_generated/src/models/* src/adapters/rest/models/ && \
  rm -rf temp_generated
