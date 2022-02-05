This document describes breaking changes of `teloxide` crate, as well as the ways to update code.
Note that the list of required changes is not fully exhaustive and it may lack something in rare cases.

## 0.5 -> 0.6

v0.6 of teloxide introduces a new dispatching model based on the [chain of responsibility pattern]. To use it, you need to replace `prelude` with `prelude2` and `dispatching` with `dispatching2`. Instead of using old REPLs, you should now use `teloxide::repls2`.

The whole design is different than the previous one based on Tokio streams. In this section, we are only to address the most common usage scenarios.

First of all, now there are no streams. Instead of using streams, you use [`dptree`], which is a more suitable alternative for our purposes. Thus, if you previously used `Dispatcher::messages_handler`, now you should use `Update::filter_message()`, and so on.

Secondly, `Dispatcher` has been split into two separate abstractions: `DispatcherBuilder` and `Dispatcher`. The calling sequence is simple: you call `Dispatcher::builder(bot, handler)`, set up your stuff, and then call `.build()` to obtain `Dispatcher`. Later, you can `.setup_ctrlc_handler()` on it and finally `.dispatch()` (or `.dispatch_with_listener()`).

Lastly, the dialogue management system has been greatly simplified. Just compare the [new `examples/dialogue.rs`](https://github.com/teloxide/teloxide/blob/25f863402d4f377f573ce2ba394f5b768ee8052e/examples/dialogue.rs) with the [old one](https://github.com/teloxide/teloxide/tree/2a6067fe94773a0015627a6aaa1930b8f88b6da0/examples/dialogue_bot/src) to see the difference. Now you don't need `TransitionIn`, `TransitionOut`, `#[teloxide(subtransition)]`, etc. All you need is to derive `DialogueState` for your FSM enumeration, call `.enter_dialogue()` and write handlers for each of a dialogue's states. Instead of supplying dependencies in the `aux` parameter of `Transition::react`, you can just call `.dependencies()` while setting up the dispatcher and all the dependencies will be passed to your handler functions as parameters.

For more information, please look at the appropriate documentation pages and the [updated examples](https://github.com/teloxide/teloxide/tree/master/examples). Note that, in one of the upcoming releases, the old dispatching model will be removed, so we highly encourage you to migrate your bots to the new one.

Thanks for using teloxide!

[chain of responsibility pattern]: https://en.wikipedia.org/wiki/Chain-of-responsibility_pattern
[`dptree`]: https://github.com/p0lunin/dptree
