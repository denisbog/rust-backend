#!/bin/bash
docker run --rm -u=1000:1000 -v${PWD}:/local openapitools/openapi-generator-cli generate --skip-validate-spec -i /local/definition.yaml -g rust-axum --additional-properties=packageVersion=0.1.0  --global-property=skipFormModel=false -o /local/openapi
