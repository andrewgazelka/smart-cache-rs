# Contributing to smart-cache

Hey there! 👋 Thanks for considering contributing to smart-cache. We're pretty casual here and welcome contributions of all sizes.

## Getting Started

1. **Check out the issues**: The [issues page](https://github.com/andrewgazelka/smart-cache/issues) is the best place to start. Look for issues tagged with `good first issue` if you're new!

2. **No formal process**: Feel free to dive right in! You can:
   - Pick up any open issue
   - Fix a bug you found
   - Add a feature you think would be cool
   - Improve documentation
   - Add more tests

3. **Making changes**:
   - Fork the repo
   - Create a branch
   - Make your changes
   - Submit a PR

## Development

```bash
# Run tests
cargo nextest run

# Format code
cargo fmt

# Run lints
cargo clippy
```

## Need Help?

- Just open an issue with your question
- No such thing as a dumb question here!

## Code Style

- We use `rustfmt` and `clippy` defaults
- Use `tracing` for logging (`debug!`, `trace!`, `info!`, `error!`, `warn!`)
- Add tests for new functionality
- Use `eyre` for error handling

That's pretty much it! Looking forward to your contributions! 🚀 