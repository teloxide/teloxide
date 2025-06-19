# How to release new crate versions

## Prerequisites

1. Run `cargo login` if you aren't logged in to crates.io
2. Make sure that you have push rights both on GitHub repo and crates.io
3. Make sure you're on the updated master branch and switch to new branch from it: `git switch master && git pull && git switch -c release`

## Release

1. `cargo release --package teloxide-core [major|minor|patch]` (if your git remote isn't named `origin`, you need to specify it with `--push-remote <remote_name>`)
2. Analyze the output and if everything is OK, re-run the same command with the `--execute` flag (this will publish release on crates.io and push commit+tag to GitHub repo)
3. Repeat 1 and 2 with the `teloxide-macros` crate if there were any changes
4. Make sure all dependencies in `crates/teloxide/Cargo.toml` point to the released versions from crates.io, including teloxide-core and teloxide-macros. Commit the change if this isn't the case
5. Add the release version in `MIGRATION_GUIDE.md` if it's a breaking release
6. `cargo release --package teloxide [major|minor|patch]` (use `--push-remote <remote_name>` if necessary)
7. If everything is OK, run again with the `--execute` flag
8. Open Pull Request and wait for the merge to complete
9. Publish the teloxide release on GitHub Releases with the specified changelog link
