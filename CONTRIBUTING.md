# Contributing

> [!NOTE]
>
> These contributing instructions might not be fully up-to-date or complete.
> However, they should be a good starting point.
>
> If you find inaccuracies/missing things, please expand this or contact us.

## Reporting bugs, questions, feature requests and adding new features

To report a bug or suggest new functionality, go to the [issues](https://github.com/teloxide/teloxide/issues). Try to make MRE (**M**inimal **R**eproducible **E**xample) and specify your `teloxide` version to let others help you.

If you want to ask a question, you can either
- open a new [GitHub discussion](https://github.com/teloxide/teloxide/discussions), or
- write to our Telegram group ([ENG](https://t.me/teloxide), [RU](https://t.me/teloxide_ru)).

If you want to contribute a new feature or a TBA update, you should contact other developers in the [teloxide_dev](https://t.me/teloxide_dev) chat, or in the github issues. What you want to do may already be in the works, or we may not want to support your feature in the future. 

## Code

### Style guide

Before writing code, please read [our code style](./CODE_STYLE.md).

### Git

To change the source code, you need a local copy of it. Fork the `master` branch of this repository via GitHub and clone your fork locally.

When working on a new thing, create a new branch with `git switch -c my-branch-name` (or other commands that work with branches). This way, it will be easier to manage when you want to do other things.

When your changes are ready, you can open a new GitHub pull request.

### Pull Requests

If your pull request fixes/resolves an existing [GitHub issue], please specify so in the PR description. For example:

> Fixes #991.

You can learn more about [using keywords in issues and pull requests] in the GitHub documentation.

If your pull request suggests new functionality or new changes, please explain your point of view and all the necessary details (pros, cons, why you chose the design you chose, your use cases, etc.)

In general, try to make PR title/description as clear as possible, as they are the primary ways of communicating your intent to the reviewer.

If your PR introduces breaking changes, mark them with `[**BC**]` in the changelog and update [MIGRATION_GUIDE.md](MIGRATION_GUIDE.md).

[GitHub issue]: https://github.com/teloxide/teloxide/issues
[using keywords in issues and pull requests]: https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/using-keywords-in-issues-and-pull-requests

### Merge conflicts

If your branch has conflicts with master, please resolve them by doing something like this:

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

### Developing with DevPod

If you want, you can develop `teloxide` inside a DevPod container. You can read more about DevPod [here](https://devpod.sh/docs/what-is-devpod).

Option 1:

```shell
git clone <your-fork> teloxide
cd teloxide
devpod up .
```

Option 2:

```shell
devpod up https://github.com/teloxide/teloxide
```

### Testing

When you open a PR, it will be tested in the CI. We recommend you test the PR before opening it:

```shell
just ci
```

Or manually, if you don't have `just` tool:

```shell
# Formatting (use `-- --check` if you only want to check)
cargo fmt --all

# Run linter (compiles code too)
cargo clippy --all-targets --features "full nightly"

# Running tests
cargo test --features "full nightly"

# Documentation (use --open if you want to open it in a browser)
# (note the -s, `docs` is an alias to pass some additional flags to `rustdoc`)
cargo docs
```

### Making a TBA update

Once again, if you want to make a major update, like a TBA update, you need to contact other developers to not waste your and other developers time!

#### Step 1:

You start at the [official TBA release notes](https://core.telegram.org/bots/api-changelog), BUT you should use [web archive](https://web.archive.org/) if `teloxide` is lagging behind a few updates. If you use the latest docs you might add something that is not yet supported in the rest of the codebase!

There are a few most common lines:

1. A new type (e.g: "Added the class `UniqueGift` describing a gift that was upgraded to a unique one.")
2. A new method (e.g: "Added the method `giftPremiumSubscription`, allowing bots to gift a user a Telegram Premium subscription paid in Telegram Stars.")
3. A new parameter to a method (e.g: "Added the parameter `pay_for_upgrade` to the method `sendGift`.")
4. A new field to a type (e.g: "Added the field `upgrade_star_count` to the class `Gift`.")
5. A documentation change (e.g: "Documented that .MP3 and .M4A files can be used as voice messages.". But it can be more convoluted, and it does NOT say the classes where the docs were changed!)

There are more obscure things, like renaming or replacing something, but these five are the most common.

#### Step 2:

You should start with adding a new type. New methods and fields may depend on the new types, and adding them is pretty easy. The hardest part is figuring out `serde`.

1. You look at the web archive for the type documentation and just... Write it into a struct! For example, lets implement [`PollOption`](https://core.telegram.org/bots/api#polloption) (you should be using web archive)

The docs say this:
![image](https://github.com/user-attachments/assets/c907e173-afe3-4c68-9c31-5990b2875462)

So, writing that out into a struct:

```rust
/// This object contains information about one answer option in a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#polloption).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PollOption {
    /// Option text, 1-100 characters.
    pub text: String,

    /// Special entities that appear in the option text. Currently, only custom
    /// emoji entities are allowed in poll option texts
    pub text_entities: Option<Vec<MessageEntity>>,

    /// Number of users that voted for this option.
    pub voter_count: u32,
}
```

All types should implement `Clone`, `Debug`, `PartialEq`, `Serialize` and `Deserialize`. If the fields allow it, you should also implement `Eq` and `Hash` (if some field contains `f32` there traits are not possible to implement)

And all new types should be added to `teloxide-core/src/types/` as a new file, if needed.

2. Teloxide has more types than the official TBA docs. For example, all ids should be their own type:

```rust
/// Identifier of a story.
#[derive(Clone, Copy)]
#[derive(Debug, derive_more::Display)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct StoryId(pub u64);
```

for numbered ids, and

```rust
/// A unique identifier of the Telegram transaction.
#[derive(Clone, Debug, derive_more::Display)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize, From)]
#[serde(transparent)]
#[from(&'static str, String)]
pub struct TelegramTransactionId(pub String);
```

for `String` ids

3. If a type has more than one variant, you need to make that type an enum:

```rust
/// This object describes the source of a transaction, or its recipient for
/// outgoing transactions.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum TransactionPartner {
    Fragment(TransactionPartnerFragment),
    User(Box<TransactionPartnerUser>),
    TelegramAds,
    TelegramApi(TransactionPartnerTelegramApi),
    Other,
}
```

Or, if a type has variants, but all variants share some common variables, you should make a `TypeKind`. For example:

```rust
/// This object represents a sticker.
///
/// [The official docs](https://core.telegram.org/bots/api#sticker).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct Sticker {
    /// Metadata of the sticker file.
    #[serde(flatten)]
    pub file: FileMeta,

    /// Sticker width, in pixels.
    ///
    /// You can assume that `max(width, height) = 512`, `min(width, height) <=
    /// 512`. In other words one dimension is exactly 512 pixels and the other
    /// is at most 512 pixels.
    pub width: u16,

    /// Sticker height, in pixels.
    ///
    /// You can assume that `max(width, height) = 512`, `min(width, height) <=
    /// 512`. In other words one dimension is exactly 512 pixels and the other
    /// is at most 512 pixels.
    pub height: u16,

    /// Kind of this sticker - regular, mask or custom emoji.
    ///
    /// In other words this represent how the sticker is presented, as a big
    /// picture/video, as a mask while editing pictures or as a custom emoji in
    /// messages.
    #[serde(flatten)]
    pub kind: StickerKind,

    /* Other common fields... */
}

/// Kind of a [`Sticker`] - regular, mask or custom emoji.
///
/// Dataful version of [`StickerType`].
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum StickerKind {
    /// "Normal", raster, animated or video sticker.
    Regular {
        /// Premium animation for the sticker, if the sticker is premium.
        premium_animation: Option<FileMeta>,
    },
    /// Mask sticker.
    Mask {
        /// For mask stickers, the position where the mask should be placed.
        mask_position: MaskPosition,
    },
    /// Custom emoji sticker.
    CustomEmoji {
        /// A unique identifier of the custom emoji.
        custom_emoji_id: CustomEmojiId,
    },
}
```

4. You should write out all of the `serde` stuff. For `TypeKind` you need to add `#[serde(flatten)]`. For almost all structs you need to add `#[serde_with::skip_serializing_none]`. For fields that don't have a default (like `bool`) you need to add `#[serde(default)]`. There are more `serde` stuff, mostly try to look at the existing types and copy them.

#### Step 3:

Add new fields to types and new parameters to methods. For how to add a new parameter to a method, look at the [Adding a new TBA method](#adding-a-new-tba-method) section.

#### Step 4:

Add new methods. For that also look at [Adding a new TBA method](#adding-a-new-tba-method).

#### Step 5:

If some new things broke too much, or are complex in general, they should be tested! Add tests as neeeded, the code needs to be robust!

#### Step 6:

Finally, update all of the docs! Beware: TBA can be a little secretive about what it updated in the docs.

#### Step 7:

Bump the supported TBA version where needed. For how to do that look at [Bumping supported TBA version](#bumping-supported-tba-version) section.

If there were breaking changes, you should also write them in `MIGRATION_GUIDE.md`!

#### Step 8:

If there was a release lately, you should probably change the `teloxide/Cargo.toml` to use the path for `teloxide-core` and `teloxide-macros`, so they reference the latest code

#### Step 9:

If there was something that wasn't documented in `CONTRIBUTING.md`, you should update it with your experience!

#### Step 10:

Make a PR and wait for review, you should be all set.

### Bumping supported TBA version

When you introduce changes that bump suppported Telegram Bot API version (e.g. 6.9 → 7.0), you must:

- Specify your changes in [crates/teloxide-core/CHANGELOG.md](crates/teloxide-core/CHANGELOG.md) file
- Mark all breaking changes with `[**BC**]` in the CHANGELOG
- Specify your PR without detailed list of changes in [CHANGELOG.md](CHANGELOG.md) file and mention breaking changes with `[**BC**]` mark if there're any
- Change TBA version and it's announce date in `api_version: ApiVersion(ver: "7.0", date: "December 29, 2023"),` line in head of [crates/teloxide-core/schema.ron](crates/teloxide-core/schema.ron) file
- Change TBA version in `(Currently, version … is supported)` line in head of [crates/teloxide-core/src/lib.rs](crates/teloxide-core/src/lib.rs) file
- Change TBA version in `Currently, version … of` line in head of [crates/teloxide/src/lib.rs](crates/teloxide/src/lib.rs) file
- Change TBA version in `…https://img.shields.io/badge/API%20coverage…` line in [crates/teloxide-core/README.md](crates/teloxide-core/README.md) file
- Change TBA version in `…https://img.shields.io/badge/API%20coverage…` line in [README.md](README.md) file

### Adding a new TBA method

#### Step 1:

Add the method and its info to `crates/teloxide-core/src/schema.ron` file.

For example, lets add a `createChatInviteLink` from TBA 5.1. Look at the [TBA documentation](https://web.archive.org/web/20210331031440/https://core.telegram.org/bots/api#createchatinvitelink) in the web archive (`teloxide` can lag behind a few versions of TBA, you should look at the past versions to not implement something that isn't in the version you are doing).
After that, you need to start adding the method. First of all, look at where that method is relative to other methods. 
`createChatInviteLink` is between `exportChatInviteLink` and `editChatInviteLink`. Find that spot in the `schema.ron` file and add the method as such:

```ron
Method(
    names: ("createChatInviteLink", "CreateChatInviteLink", "create_chat_invite_link"), // one camelCase, one PascalCase and one snake_case
    return_ty: RawTy("ChatInviteLink"), // The return type. TBA docs usually specify it. 
    // If the type is not one of the basic ones (e.g. String, u32, True, bool), you need to add RawTy("...")
    doc: Doc(
        md: "Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. The link can be revoked using the method [revokeChatInviteLink]. Returns the new invite link as [ChatInviteLink] object.",
        // Copied from the TBA. If the docs contain links, they should be added in the md_links
        md_links: {
            "revokeChatInviteLink": "https://core.telegram.org/bots/api#revokechatinvitelink",
            "ChatInviteLink": "https://core.telegram.org/bots/api#chatinvitelink",
        }
    ),
    tg_doc: "https://core.telegram.org/bots/api#createchatinvitelink",
    tg_category: "Available methods",
    // Copy parameters to here. 
    params: [
        Param(
            name: "chat_id",
            // Types work the same way as the return_ty
            ty: RawTy("Recipient"),
            descr: Doc(md: "Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)")
        ),
        Param(
            name: "name",
            ty: Option(String),
            descr: Doc(md: "Invite link name; 0-32 characters")
        ),
        Param(
            name: "expire_date",
            ty: Option(i64),
            descr: Doc(md: "Point in time (Unix timestamp) when the link will expire")
        ),
        Param(
            name: "member_limit",
            ty: Option(u32),
            descr: Doc(md: "Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999")
        ),
        Param(
            name: "creates_join_request",
            ty: Option(bool),
            descr: Doc(md: "True, if users joining the chat via the link need to be approved by chat administrators. If True, member_limit can't be specified")
        ),
    ],
),
```

That's the basics, for more info about .ron file look at the other methods or ask others!

#### Step 2:

Run `cargo test --features "full nightly"` or `just test`. This will trigger the codegen scripts, and .ron will be converted to code!

#### Step 3:

Rerun the tests, and look at the compiler errors. Most likely there will be errors from `crates/teloxide-core/src/adaptors`, `crates/teloxide-core/src/requests/requester.rs` and `crates/teloxide-core/src/bot/api.rs`.

1. To fix some of the errors in the adaptors, just add the snake_case of the new method to the `requester_forward!` macro in `throttle/requester_impl.rs`, `cache_me.rs`, `erased.rs` and `trace.rs`. You also need to add it to the end of `requests/requester.rs` to `forward_all!` macro:
```diff
requester_forward! {
    ...
    export_chat_invite_link,
+   create_chat_invite_link,
    edit_chat_invite_link,
    ...
}
```

2. To fix `parse_mode.rs` adaptor, most of the time you will need to add the snake_case of the new method to the **second** `requester_forward!` macro:
```diff
requester_forward! {
    ...
    => f, fty
}

requester_forward! {
    ...
    export_chat_invite_link,
+   create_chat_invite_link,
    edit_chat_invite_link,
    ...
}
```

But if new method contains `ParseMode`, `InlineQueryResult`, `InputMedia` or maybe some other type containing `ParseMode` inside, you will need to add `Clone` where clause to the `DefaultParseMode` impl and the snake_case of the new method to the **first** `requester_forward!` macro:
```diff
impl<B> Requester for DefaultParseMode<B>
where
...
    B::AnswerInlineQuery: Clone,
    B::AnswerWebAppQuery: Clone,
+   B::SavePreparedInlineMessage: Clone,
...
requester_forward! {
    ...
    answer_inline_query,
    answer_web_app_query,
+   save_prepared_inline_message,
    ...
    => f, fty
}

requester_forward! {
    ...
```

Also if your method contains `ParseMode` in any form, you will need to implement `VisitParseModes` trait, see `parse_mode.rs` for examples.

3. Then you have to add the function definition to `bot/api.rs` like that:
```rust
type CreateChatInviteLink = JsonRequest<payloads::CreateChatInviteLink>;

fn create_chat_invite_link<C>(&self, chat_id: C) -> Self::CreateChatInviteLink
where
    C: Into<Recipient>,
{
    Self::CreateChatInviteLink::new(self.clone(), payloads::CreateChatInviteLink::new(chat_id))
}
```

4. To fix `erased.rs` adaptor you need to add a new function to `ErasedRequester`. First, add the definition:

```rust
fn create_chat_invite_link(
    &self,
    chat_id: Recipient,
) -> ErasedRequest<'a, CreateChatInviteLink, Self::Err>;
```

Then add the function, similarly to `bot/api.rs`:
```rust
fn create_chat_invite_link(
    &self,
    chat_id: Recipient,
) -> ErasedRequest<'a, CreateChatInviteLink, Self::Err> {
    Requester::create_chat_invite_link(self, chat_id).erase()
}
```

After that run the tests again, it should be all done!

#### Special cases

1. Telegram has `editMessageText`, `editMessageCaption` and other edit methods. Usually they work two ways, one for regular messages and one for inline ones. But that is very confusing and complicates a lot of interactions, so `teloxide` works differently. All edit methods are split in two, one for regular messages and one for inline ones. To look at an example, look at `editMessageText` and `editMessageTextInline` in `schema.ron`. The process is similar to adding a regular method, but you will have to add two, as well as adding a `sibling` field to both of the new methods. Also, return types differ (refer to telegram documentation for what these types are).

2. Same as the 1st point, if the method requires "this or that", you probably should make two methods. The example is `sendGift` and `sendGiftChat`.

3. Sometimes you will encounter situation when it's impossible to write something in `schema.ron`. It may be some doc comment that renders incorrectly or some type (like `Option<DateTime<Utc>>`) impossible to write. In such cases you can workaround it with `crates/teloxide-core/src/codegen/patch.rs` file.

4. In very rare cases, autogenerated files in `crates/teloxide-core/src/payloads/*` may have errors related to deriving impls for the `PartialEq`, `Eq`, or `Hash` traits. In such cases, remove them manually and add the payload as an exception to `codegen_payloads()` in `crates/teloxide-core/src/payloads/codegen.rs`.

#### Other notes

1. If you mess up the .ron and run the codegen, it is better to reset the files, rather than to try and fix it all by hand:
`git restore crates/teloxide-core/src/local_macros.rs crates/teloxide-core/src/payloads.rs crates/teloxide-core/src/payloads/ crates/teloxide-core/src/requests/requester.rs && git clean -fd crates/teloxide-core/src/payloads/`

This command will restore the listed files to your current git branch and delete all the new files in the `payloads/` dict.

2. Some methods require special attention, if you find that, please make sure to add it to this guide as an exception!

## @teloxidebot

`teloxide` uses @teloxidebot as a helper to manage PRs and issues. It is based on triagebot used by rustc developers, which docs can be found [here](https://forge.rust-lang.org/triagebot/index.html).

We will describe here a few most used @teloxidebot's features, but we still recommend you to read the docs.

### PR status tracking

`teloxide` uses `S-*` labels (mainly https://github.com/teloxide/teloxide/labels/S-waiting-on-author and https://github.com/teloxide/teloxide/labels/S-waiting-on-review) to track the status of pull requests.

You can change the status with `@teloxidebot review` and `@teloxidebot ready` (sets the status to https://github.com/teloxide/teloxide/labels/S-waiting-on-review) or `@teloxidebot author` (sets the status to https://github.com/teloxide/teloxide/labels/S-waiting-on-author).

Requesting a review from PR's assignee via GitHub UI will also change the status of the PR to waiting on review. Similarly, submitting a review that requests changes will change the status of the PR to waiting on author.

There is also https://github.com/teloxide/teloxide/labels/S-blocked, which can be set with `@teloxidebot blocked`.

Please note that your PR won't be reviewed unless it's waiting for review :)

### Labels

Normally, GitHub only allows privileged users to change labels. @teloxidebot allows anyone to add or remove certain [labels](https://github.com/teloxide/teloxide/labels/) with `@teloxidebot label +additional_label -removed_label`. See more in the [documentation](https://forge.rust-lang.org/triagebot/index.html).

### PR assignment

When a PR is created, @teloxidebot will automatically assign one of the maintainers to it. If you want to override this assignment, use `r? @ReviewerUsername`.
