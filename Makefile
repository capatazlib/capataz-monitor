help:	## Display this message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
.PHONY: help
.DEFAULT_GOAL := help

PROTO_FILES    = $(shell find proto -name '*.proto')
PROTO_RS_FILES = $(PROTO_FILES:proto/%.proto=src/%.rs)

src/%.rs: proto/%.proto
	protoc --rust_out $(dir $@) $^

proto: $(PROTO_RS_FILES) ## Generate protobuf files

.PHONY: proto
