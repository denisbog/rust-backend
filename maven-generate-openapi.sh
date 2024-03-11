#!/bin/bash
java -jar openapi-generator/modules/openapi-generator-cli/target/openapi-generator-cli.jar generate --skip-validate-spec -i definition.yaml -g rust-axum --additional-properties=packageVersion=0.1.0  --global-property=skipFormModel=false -o openapi
