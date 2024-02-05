#!/bin/bash
docker run --rm -u=1000:1000 -v${PWD}:/local openapitools/openapi-generator-cli generate --skip-validate-spec -i /local/definition.yaml -g rust-axum -o /local/openapi
