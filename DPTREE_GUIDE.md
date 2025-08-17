# `dptree` guide

`teloxide` heavily relies on the [`dptree`] crate, but it might take some time to understand how to use it. `dptree` handles where the incoming update will go, and also what values (dependencies) the handlers will receive.

[`dptree`]: https://github.com/teloxide/dptree

This guide assumes you have some basic knowledge of `teloxide` (see `README.md` and the [examples]), but you have some troubles with understanding `dptree`, which this tutorial will guide you through.

[examples]: crates/teloxide/examples/

Let's start from the beginning:

## Basics

You can look at `dptree` like at a _tree_. It all starts with `dptree::entry()`, which is the basic _entry node_.

To actually add functionality to that, you need an _endpoint_:

```rust
let handler = dptree::entry().endpoint(update_handler);
```

Right now, `dptree::entry()` invokes the endpoint, and it does so for every single update.

And let us make this clear: if `update_handler` (the endpoint) executes, the whole tree is done -- it has fulfilled its goal of finding the right handler and will not run any other code. For example:

```rust
let handler = dptree::entry()
    // `branch` explanation will be down below
    .branch(Update::filter_message().endpoint(message_handler))
    .endpoint(/* ... */);  // IF message_handler executed, this endpoint will not fire.
    // Otherwise, if the message_handler did not execute and passed on the controlflow,
    // this endpoint will execute every single time,
    // because nothing prevents it from not executing
```

If `dptree` doesn't find any handler that fits, `teloxide` will execute the [default_handler].

[default handler]: https://docs.rs/teloxide/latest/teloxide/dispatching/struct.DispatcherBuilder.html#method.default_handler

## Filters

To filter the updates that are coming in, there are a few relevant methods. The most convenient ones are the build-in filters like `Update::filter_message()` and `Message::filter_text()`. They work like this:

```rust
let handler = Update::filter_message().endpoint(message_handler);
```

If the incoming update in isn't a message (for example, if it is a callback query), the `Update::filter_message()` will not invoke the endpoint.

## Dependencies

As well as managing the flow of the update, `dptree` also manages the flow of so-called dependencies. Sounds scary, but it is really just a hashmap (`dptree::di::DependencyMap`) with type identifiers mapped to concrete values. When, for example, `Update::filter_message()` puts the value (that being a `Message` value) into `DependencyMap`, it becomes accessible to all subsequent nodes that are invoked by that filter, but only for them.

Let's demonstrate it through a concrete example:

```rust
async fn message_handler(message: Message) -> HandlerResult {
    /// Do stuff with message
    Ok(())
}

let handler = Update::filter_message().endpoint(message_handler);
```

`message_handler` can access the `Message` type only because `Update::filter_message()` ensured that the update is a message, and only then it puts the `Message` type into `DependencyMap`.

Let me reiterate, `DependencyMap` only cares about the types of the stuff that is passed into it: you can name the parameters in your functions however you want, the only thing that matters is `: Message` type definition, `dptree` will know to put the `Message` variable into that parameter.

You can add your own type (or absolutely any type for that matter) into `DependencyMap` with `dptree::map` and `dptree::map_async`:

```rust
async fn map_my_type(
    /* You can put types here the
    same way you would in endpoints */
    update: Update,
) -> MyType {
    return MyType("some values");
}

async fn my_type_handler(my_variable: MyType) -> HandlerResult {
    /// Do stuff with my_variable
    Ok(())
}

let handler = dptree::map_async(map_my_type).endpoint(my_type_handler);
```

If you have two different things that insert the same type, the last one will override everything else:

```rust
let handler = dptree::map_async(map_my_type1)
    .map_async(map_my_type2)  // The `MyType` returned by `map_my_type2`
    // will override `MyType` that `map_my_type1` returns
    .endpoint(my_type_handler);
```

By default, every single `teloxide` tree has two types in it: the `Update` type because, well, something has to kickstart the tree, and the `Me` type, which contains the info about the bot. There also is a third type, usually a `Bot` type, but the type can change with [bot adaptors](https://docs.rs/teloxide/latest/teloxide/adaptors/index.html). To add types that exist in every tree, you need to use `deps![]` with `.dependencies(deps![/* your variables here */])` in `DispatcherBuilder`.

## Branches

Branches are what allows `dptree` to take on different paths to try and find the one that fits. Let's start off with an example:

```rust
let handler = dptree::entry()
    .branch(Update::filter_message().endpoint(message_handler))
    .branch(Update::filter_callback_query().endpoint(callback_handler))
    .branch(Update::filter_inline_query().endpoint(inline_query_handler));
```

Here the `dptree` starts off at `dptree::entry()`, as it is the first thing that is in the tree. Then it invokes the first branch, and that branch invokes `Update::filter_message()`.

If update is a new message, that filter places `Message` in `DependencyMap` and invokes the endpoint `message_handler`.

But if the update is, for example, a callback query, then `Update::filter_message()` will stop its whole branch (which is `.endpoint(message_handler)`), and the first branch will invoke the second one, which will execute as usual.

If one branch adds types to `DependencyMap`, but then gets stopped by a filter, all of the types that got added to `DependencyMap` will not carry over to other branches. For example:

```rust
let handler = dptree::entry()
    .branch(
        // .chain just says "go on", it allows to start a new branch,
        // like `dptree::entry()` or, in this case, `Update::filter_message()`
        dptree::map_async(map_my_type)
            .chain(Update::filter_message())
            .endpoint(my_type_handler),
    )
    .branch(
        // This will cause a startup error, because `MyType` is not
        // in the `DependencyMap` in this branch, it has to be added again
        Update::filter_callback_query().endpoint(my_type_handler),
    );
```

## The `case!` macro

The `case![]` macro just checks if some enumeration value in `DependencyMap` is of a particular variant, and if it is, it adds its contents to `DependencyMap` in the form of a tuple (if there is more than one type):

```rust
// Any types will work! Custom ones, built-in ones, from another crate, any!
#[derive(Clone)]
struct A;

#[derive(Clone)]
struct B;

#[derive(Clone)]
struct C;

#[derive(Clone)]
enum MyEnum {
    Variant1 { field1: A, field2: B },
    Variant2 { field: A },
    Variant3 { field1: A, field2: B, field3: C },
}

fn map_enum_variant1() -> MyEnum {
    return MyEnum::Variant1 {
        field1: A,
        field2: B,
    };
}

async fn enum_variant1_handler(variant_content: (A, B)) -> HandlerResult {
    Ok(())
}

fn map_enum_variant2() -> MyEnum {
    return MyEnum::Variant2 { field: A };
}

async fn enum_variant2_handler(
    // Not in a tuple!
    variant_content: A,
) -> HandlerResult {
    Ok(())
}

fn map_enum_variant3() -> MyEnum {
    return MyEnum::Variant3 {
        field1: A,
        field2: B,
        field3: C,
    };
}

async fn enum_variant3_handler((field1, field2, field3): (A, B, C)) -> HandlerResult {
    Ok(())
}

let handler = dptree::map(map_enum_variant1)
    .branch(case![MyEnum::Variant1 { field1, field2 }].endpoint(enum_variant1_handler));

let handler = dptree::map(map_enum_variant2)
    .branch(case![MyEnum::Variant2 { field }].endpoint(enum_variant2_handler));

let handler = dptree::map(map_enum_variant3).branch(
    case![MyEnum::Variant3 {
        field1,
        field2,
        field3
    }]
    .endpoint(enum_variant3_handler),
);
```

This is particularly useful with [dialogue feature] and states.

[dialogue feature]: https://github.com/teloxide/teloxide?tab=readme-ov-file#dialogues-management

## Type checking and dead code detection

Now `dptree` can detect if the types of your handlers are wrong, or if some of your code will never execute.
Moslty the error messages will tell you everything you need to do.

Let's look at an example of dead code detection:

```rust
let handler = dptree::entry()
    .endpoint(update_handler)
    .endpoint(/* ... */); // This endpoint will trigger a startup panic, because the first
                          // endpoint always break execution

let handler = dptree::entry()
    .filter(/* ... */)
    .endpoint(update_handler)
    .map(/* ... */); // This will also trigger a panic, because we already specified the
                     // endpoint earlier
```

The error message will look like this:
![image](https://github.com/user-attachments/assets/aeb6a6df-bb11-4033-98b8-9ee547c452c0)

And an example of type checking:

```rust
let handler = dptree::entry().endpoint(message_handler); // Will result in a startup error,
// There is no `Message` in the `DependencyMap`

let handler = Update::filter_callback_query().endpoint(message_handler); // Will also result in a startup error,
// There is only `CallbackQuery` in the `DependencyMap`, no `Message`
```

The error message will look like this:
![image](https://github.com/user-attachments/assets/b5e46af2-1a7d-4b88-8f9f-a721cb136ea2)

To look at more examples of dead code and type checking you can look at [dptree/src/handler/core.rs](https://github.com/teloxide/dptree/blob/master/src/handler/core.rs#L983)

## Going deeper into `dptree`

This section just showcases what is considered "common knowledge", but may be something you don't notice:

1. You can call `.filter()`, `.map()` and any other `dptree` function with any part of the `dptree` (aside from the `.endpoint()`, because it will do nothing):

```rust
let handler = dptree::entry().filter(/* ... */);
let handler = Update::filter_message().map_async(/* ... */);
let handler = dptree::map_async(/* ... */).map_async(/* ... */).filter(/* ... */);
```

2. The generic return type of the complete tree is `UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>>`. To have the type of the whole tree figured out by Rust compiler without spelling out the types, you need to have an `.endpoint()` somewhere as well as actively using it in `teloxide`'s `DispatcherBuilder`, which will tell the compiler the type of the error, and also that it is an `UpdateHandler` tree, and not something else.
3. To use `Message::filter_...()` and stuff like that, you need to have `Message` in `DependencyMap` (usually by the means of `Update::filter_message()`)
4. There exist the `filter_map` and `filter_map_async` methods. They have to return `Option<ReturnType>`. If the option is `None`, the method will act as a filter and close up the branch. If the option is `Some(ReturnType)`, the method will act as a map and insert `ReturnType` into `DependencyMap`
5. There also exist `inspect` and `inspect_async`, which will allow you to just, well, inspect the fields without altering anything. They work just as `filter` or `map` -- they just don't do anything to the control flow, only inspecting. This is very useful for debugging and seeing where the update goes!
6. If you get an error like `the trait bound [closure@examples/state_machine.rs:150:20: 150:92]: Injectable<_, bool, _> is not satisfied`, your handler does not implement the `Injectable` trait. Ensure that your types implement `Clone` and can be shared between threads (`Send + Sync`). If they are too expensive to clone, you can wrap your types into `Arc`.
7. There are a lot of premade filters in `teloxide`, like `.filter_command()`, `.filter_mention()`, `Message::filter_poll()` and many others -- you can look them up in the documentation!

If you find something you got stuck on with `dptree`, please make a PR that adds an explanation to this guide -- this will help others a lot!
