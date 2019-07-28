help:	## Display this message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
.PHONY: help
.DEFAULT_GOAL := help

PROTO_DIR      = ../proto
PROTO_FILES    = $(shell find $(PROTO_DIR) -name '*.proto')
PROTO_RS_FILES = $(PROTO_FILES:$(PROTO_DIR)/%.proto=src/%.rs)

src/%.rs: $(PROTO_DIR)/%.proto
	protoc --rust_out $(dir $@) -I $(dir $^) $(notdir $^)

proto: $(PROTO_RS_FILES) ## Generate protobuf files
.PHONY: proto
