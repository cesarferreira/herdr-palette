.PHONY: check test release

LEVEL ?= minor

check:
	bun run typecheck
	bun test

test:
	bun test

# Validate, bump both package and plugin versions, commit, tag, and push.
# Usage: make release or make release LEVEL=patch
release:
	bun pm version $(LEVEL)
