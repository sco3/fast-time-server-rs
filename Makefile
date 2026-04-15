# Get all arguments passed to make
RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
# Treat the arguments as do-nothing targets so make doesn't complain
$(eval $(RUN_ARGS):;@:)

.PHONY: %

# The "catch-all" target
%:
	@if command -v just > /dev/null; then \
		just $@ $(RUN_ARGS); \
	else \
		echo "Error: 'just' is not installed. Please install it from https://github.com/casey/just"; \
		exit 1; \
	fi

# Default goal when running just 'make'
.DEFAULT_GOAL := default_help
default_help:
	@just --list
