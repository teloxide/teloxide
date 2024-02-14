# Contributing

Before contributing, please read [the license](./LICENSE).

> [!NOTE]
>
> These contributing instructions might not be fully up-to-date or complete.
> However, they should be a good starting point.
>
> If you find inaccuracies / missing things, please expand this or contact us.

## Reporting bugs, questions, feature requests

To report a bug or suggest new functionality go to [issues](https://github.com/teloxide/teloxide/issues).
Try to make MRE (**M**inimal **R**eproducible **E**xample) and specify your teloxide version to let others help you.

If you want to ask a question, you can either
- Open a new [github discussion](https://github.com/teloxide/teloxide/discussions), or
- Write to our telegram group ([eng](https://t.me/teloxide), [ru](https://t.me/teloxide_ru))

## Code

### Style guide

Before writing code, please read [our code style](./CODE_STYLE.md).

### Git

To change the source code, you need a local copy of it.
Fork the `master` branch of this repository via github and clone your fork locally.

When working on a new thing, create a new branch with `git switch -c my-branch-name` (or other commands that work with branches).
This way it will be easier to manage when you want to do other things.

When your changes are ready, you can open a github pull request.

### Pull Requests

If your pull request fixes/resolves an existing [github issue] please specify so in the PR description.
For example:

> Fixes #991

You can learn more about [Using keywords in issues and pull requests] in github documentation.

If your pull request suggests new functionality or new changes,
please explain your point of view and all the necessary details.
(pros, cons, why you chose design you chose, your use cases, etc)

In general try to make PR title/description clear, as they are the primary ways of communicating your intent to the reviewer 

[github issue]: https://github.com/teloxide/teloxide/issues
[Using keywords in issues and pull requests]: https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/using-keywords-in-issues-and-pull-requests

### Merge conflicts

If your branch has conflicts with master please resolve them by doing something like this:

```shell
# Temporary switch to master branch
git switch master

# Pull changes from the upstream.
# You may need to use something different from "origin",
# depending on how you setup your remotes.
git pull origin master

# Switch back to your feature branch
git switch -

# Move your changes on top of changes in master branch.
git rebase master

# Here you'll need to resolve the conflicts,
# git commands will print some guidance.

# Once conflicts are resolved,
# forcefully push the changes to your fork
git push --force-with-lease
```

### Testing

When you open a PR it will be tested in CI.
We recommend you test the PR before opening it:

```shell
# Formatting (use `-- --check` if you only want to check)
cargo fmt --all

# Build
cargo build --features "full nightly"

# Run linter
cargo clippy --all-targets --features "full nightly"

# Running tests
cargo test --features "full nightly"

# Documentation (use --open if you want to open it in a browser)
# (note the -s, `docs` is an alias to pass some additional flags to `rustdoc`)
cargo docs
```

## Teloxide bot

Teloxide uses @teloxidebot as a helper to manage PRs and issues.
It's based on triagebot developed by rustc developers which docs can be found [here](https://forge.rust-lang.org/triagebot/index.html).

We'll describe here a few most used @teloxidebot's features, but we still recommend you read the docs.

### PR status tracking

Teloxide uses `S-*` labels (mainly https://github.com/teloxide/teloxide/labels/S-waiting-on-author and https://github.com/teloxide/teloxide/labels/S-waiting-on-review) to track the status of pull requests.

You can change the status with `@teloxidebot review` and `@teloxidebot ready` (set the status to https://github.com/teloxide/teloxide/labels/S-waiting-on-review) or `@teloxidebot author` (sets the status to https://github.com/teloxide/teloxide/labels/S-waiting-on-author).

Requesting a review from PR's assignee via github ui will also change the status of the PR to waiting on review.
Similarly, submitting a review that requests changes will change the status of the PR to waiting on author.

There is also https://github.com/teloxide/teloxide/labels/S-blocked which can be set with `@teloxidebot blocked`.

Please note that your PR won't be reviewed unless it's waiting for review :)

### Labels

Normally github only allows privileged users to change labels.
@teloxidebot allows anyone to add or remove certain [labels](https://github.com/teloxide/teloxide/labels/) with `@teloxidebot label +additional_label -removed_label`.
See more in the [documentation](https://forge.rust-lang.org/triagebot/index.html).

### PR assignment

When a PR is created @teloxidebot will automatically assign one of the maintainers to it.
If you want to override this assignment, use `r? @ReviewerUsername`.
