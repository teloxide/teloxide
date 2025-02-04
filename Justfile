# Show available recipes
help:
    just --list

# Format code
fmt:
    cargo fmt --all

# Check formatting and run clippy
lint:
    cargo fmt --all --check || (echo "Run 'just fmt' to fix formatting!" && exit 1)
    cargo clippy --all-targets --features "full nightly"

# Run tests
test:
    cargo test --features "full nightly"

# Test documentation
docs:
    cargo docs

# Run all checks we do in CI (lint + test + docs)
ci:
    just lint
    just test
    just docs

# Same as 'just ci' but in clean environment
clean_ci:
    cargo clean
    just ci
