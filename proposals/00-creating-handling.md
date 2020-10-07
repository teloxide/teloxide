# General Info
- Name: Creating and handling (work name)
- Start date: 07-10-2020 (October 7, 2020)

# Contents

- [Abstract](#Abstract)
- [Motivation](#Motivation)
- [Specification](#Specification)
    - [`View` trait](#View-trait)
        - [Impls](#View-Impls)
    - [`ViewFactory` trait](#ViewFactory-trait)
    - [`Parser` trait](#Parser-trait)
    - [`Handler` trait](#Handler-trait)
    - [`Callback` trait](#Callback-trait)
        - [Additional interfaces for `Callback`](#Additional-interfaces-for-Callback)
    - [`Schema` trait](#Schema-trait)
- [Usability](#Usability)
    - [Simple text messages](#Simple-text-messages)
    - [Reply buttons](#Reply-buttons)
    - [Inline buttons](#Inline-buttons)
    - [Commands](#Commands)
- [Generate](#Generate)
    - [For `Callback`](#For-Callback)
    - [Schema](#Schema)
        - [Generate `ViewFactory`](#Generate-ViewFactory)
        - [Generate `Schema`](#Generate-Schema)
- [Open questions](#Open-questions)

# Abstract
This proposal propose new interfaces for working with api telegram using the callback model, where are one type produce an message and wait for an answer from user. These interfaces can be used with following update types:
1. Text messages (simple text and reply buttons). 
2. Callback queries (inline buttons).
3. Commands.
4. Inline queries.

# Motivation

Common send-update processing follows the following scenario:
1. Send possible callbacks.
2. Receive chosen callback.
3. Choose handler to handle the update.
4. Handle the update and send answer.

Since telegram bots are arranged according to the client-server arhitecture, when the client chooses an callback, we cannot directly call the handler function. We must receive a tag or array of tags and then choose handler. If we were to write both the callback constructor and the handler ourselves, it would be easy to notice that this code is repeated from application to application. This proposal propose new interfaces to work with telegram api using callback model. Further it will be shown how you can automate the process of writing code based on schemas and traits.

# Specification
Three core abstractions from this proposal is `Schema`, `ViewFactory` and `Callback`. 

`Callback` must can do 2 things:
1. Create a view with data by which it can then recognize its view.
2. Handle the answer.

`ViewFactory` create an views for `Schema` and `Callback`s.

`Schema` is an object collecting more `View`s into one view and then send it to telegram. After receiving answer it must dispatch the answer among their `Callback`s.

There are several laws that must be obeyed by `Schema`s.
1. `Schema` must produce an view with unique data, so that there are no ambiguous situations when one object was engaged in creation and another object was processed the update.
2. `Schema` must handle all queries that produces using view which was constructed using this `Schema`.

The programmer must write an schema. It must contain the possible callbacks that should be send to user. Schema implementation using derive macros will look like this:
```rust
#[derive(Schema)]
struct MainMenu {
    order_pizza : OrderPizzaCallback,
    show_my_orders : ShowMyOrdersCallback,
    log_out : LogOutCallback,
}
```
where all fields must implement `Callback` and `ViewFactory`.

## `View` trait
View trait represent an view that can send request to the server which will be gived by `Parser` and then by `Handler`. It may be for example `InlineButton`, `ReplyMarkup`. This trait may be look like this:
```rust
trait View {
    type ProducableUpdate;
}
```
where `ProducableUpdate` is a object that receives by server when user choose a `View`.

This trait is needed so that we know what update will come from this view.

Implementation for `InlineButton`:
```rust
impl View for InlineButton {
    type ProducableUpdate = CallbackData;
}
```

### `View` Impls
`View` trait must be implement for:
1. `InlineButton`.
2. `InlineKeyboard`.
3. `ReplyButton`.
4. `ReplyKeyboard`.

## `ViewFactory`
Factory create a View which will be send to the user. This trait may be look like this:
```rust
trait ViewFactory {
    type Ctx;
    type View: View;
    fn construct(&self, ctx: Self::Ctx) -> Self::View;
}
```
where `Ctx` is the context needed to create an `View`.

Implementation for `ShowMyOrdersCallback`:
```rust
impl ViewFactory for ShowMyOrdersCallback {
    type Ctx = User;
    type View = InlineButton;
    fn construct(&self, user: User) -> InlineButton {
        InlineButton {
            text: "My Orders",
            callback_data : "show_orders " + user.id
        }
    }
}
```

## `Parser` trait
Parser is a trait that parses one type to another and if it fails returns input object. For example, it can parse `Update` to `Text` when receive the text message. This trait may be look like this:
```rust
trait Parser {
    type Input;
    type Output;
    fn parse(&self, data: Self::Input) -> Result<Self::Output, Self::Input>;
}
```
It is needed to separate the process of dispatching an update from process of handle an update.

Implementation for `ShowMyOrdersCallback`:
```rust
impl Parser for ShowMyOrdersCallback {
    type Input = CallbackData;
    type Output = UserId;
    fn parse(&self, data: CallbackData) -> Result<UserId, CallbackData> {
        let data = upd.split(' ');
        if data[0] == "show_orders" {
            return Ok(data[1].to_i64());
        }
        else {
            return Err(upd);
        }
    }
}
```

## `Handler` trait
Handler is a trait that represents a handler function that handle update which send from user by `View`. This request for example may be `User`, `AnswerDTO`, etc. This trait may be look like this:

```rust
#[async_trait]
trait Handler {
    type Data;
    async fn handle(&self, data: Self::Data);
}
```
where `Data` represent an update to handle.

Implementation for `ShowMyOrdersCallback`:
```rust
#[async_trait]
impl Handler for ShowMyOrdersCallback {
    type Data = UserId;
    async fn handle(&self, user_id: Self::Data) {
        let orders = self.db.getOrders(user_id).await;
        self.bot.send_message(user_id, "Your orders:")
            .reply_markup(OrdersCallback::construct(orders))
            .send()
            .await;
    }
}
```

**Open question**: error handling.

## `Callback` trait
Callback is a trait that represents one callback. It must can:
1. Parse `Update` for further processing by `Handler`.
2. Handle parsed data.
It may be look like this:
```rust
#[async_trait]
trait Callback : 
    Parser +
    Handler<Data = <Self as Parser>::Output>
{ 
    async fn try_handle(&self, input: Self::Input) -> Result<(), Self::Input> {
        /* ommited */
    }
}

impl<A> Callback for A where
    A: Parser +
       Handler<Data = <A as Parser>::Output> +
{ }
```
And implementation may look like this:
```rust
type UserId = i64;

struct ShowMyOrdersCallback {
    bot: Bot,
    db: Arc<DbConnection>,
}
impl Parser for ShowMyOrdersCallback {
    type Input = CallbackData;
    type Output = UserId;
    fn parse(&self, upd: CallbackData) -> Result<UserId, CallbackData> {
        let data = upd.split(' ');
        return if data[0] == "show_orders" {
            Ok(data[1].to_i64())
        }
        else {
            Err(upd)
        }
    }
}
#[async_trait]
impl Handler for ShowMyOrdersCallback {
    type Data = UserId;
    async fn handle(&self, user_id: Self::Data) {
        let orders = self.db.getOrders(user_id).await;
        self.bot.send_message(user_id, "Your orders:")
            .reply_markup(OrdersCallback::construct(orders)).await;
    }
}
```

### Additional interfaces for `Callback`
`Callback` can implement some interfaces for convinient working with `Callbacks`s. First, `Callback` can work as conveyor (or, in terms of FP, `Alternative`), that try to handle using first `Callback`, and if it fails, send the update to next `Callback`. In other words we can implement function:
```rust
pub struct Alternative<A1, A2> where
    A1: Callback, 
    A2: Callback<
        Input=A1::Input,
        Output=A1::Output,
        Data=A1::Data
    >
{
    left: A1,
    right: A2,
}

impl Parser for Alternative { /* ommited */ }
impl Handler for Alternative { /* ommited */ }

pub fn or<A1, A2>(left: F1, right: F2) -> Alternative<A1, A2> where 
    A1: Callback,
    A2: Callback<Input=A1::Input, Output=A1::Output, Data=A1::Data>,
{
    Alternative {
        left,
        right,
    }
}
```
and use like this:
```rust
let res = or(callback1, or(callback2, callback3)).try_handle(data);
```

## `Schema` trait
`Schema` is a trait used to create an view collection like `InlineKeyboard`, and handle producable by this view update. It can be fully generated, that will be demonstrated further. Trait may be look like this:
```rust
#[async_trait]
trait Schema {
    async fn handle(&self, update: Self::View::ProducableUpdate) -> Result<(), Self::View::ProducableUpdate>;
}
```
`Schema` cab be `ViewFactory` because it can produces the collection of callbacks that will be sent to the user. In handle it will be used `or` function from `Callback`, such as:
```rust
async fn handle(&self, update: Upd) -> Result<(), Upd> {
    or(
        self.callback1, 
        or(
            self.callback2,
            self.callback3
        )
    ).handle(update).await
}
```
An implementation of this trait may be look like this:

```rust
impl ViewFactory for MainMenu {
    type Ctx = (OrderPizzaCallback::Ctx, ShowMyOrdersCallback::Ctx, LogOutCallback::Ctx);
    type View = InlineKeyboard;
    fn construct(&self, ctx: Self::Ctx) -> Self::View {
        let (ctx1, ctx2, ctx3) = ctx;
        InlineKeyboard::new(vec![
            self.order_pizza.construct(ctx1),
            self.show_my_orders.construct(ctx2),
            self.log_out.construct(ctx3),
        ])
    }
}
#[async_trait]
impl Schema for MainMenu {
    async fn handle(&self, update: Self::View::ProducableUpdate) -> Result<(), Self::View::ProducableUpdate> {
        or(
            self.order_pizza, 
            or(
                self.show_my_orders,
                self.log_out
            )
        ).handle(update).await
    }
}
```
As we can see these methods can be fully generated by macros.

# Generate
We can generate `ViewFactory` and `Parser` traits for `Callback`, `Viewfactory` for `Schema` and `Schema` traits.

## For `Callback`
We can auto-generate `ViewFactory` and `Parser` using additional traits and macros. This section dedicated to this.

The following deals with derive cases for structs.

Can be seen that `ViewFactory` and `Parser` will be repeated often between `Callback`s because it used the same `Update` and `View` type. So we can create objects that create the view and parse the incoming answer. For example, see `CallbackDataController`:
```rust
struct StaticTextCallbackDataController<T> {
    text: String,
    prefix: String,
    phantom: PhantomData<T>,
}
impl<T: ToString> ViewFactory for StaticTextCallbackDataController<T> {
    type Ctx = T;
    type View = InlineButton;
    fn construct(&self, data: T) -> Self::View {
        InlineButton::new(self.text, format!("{}|{}", self.prefix, data.to_string()))
    }
}
/* implemetation of Parser was ommited */
```
And then we can implement `ViewFactory` and `Handler` more simple:

```rust
type UserId = i64;

struct ShowMyOrdersCallback {
    controller: StaticTextCallbackDataController
}
impl ViewFactory for ShowMyOrdersCallback {
    type Ctx = User;
    type View = InlineButton;
    fn construct(&self, user: User) -> InlineButton {
        self.controller.construct(user.id)
    }
}
impl Parser for ShowMyOrdersCallback {
    type Input = CallbackData;
    type Output = UserId;
    fn parse(&self, data: CallbackData) -> Result<UserId, CallbackData> {
        self.controller.parse(data)
    }
}
```
Now we can create a derive macro with `factory` and `parser` attributes. How it will work:
1. Derive macros search `factory` attribute, and use it to generate a `ViewFactory` trait.
2. Derive macros search `parser` attribute, and use it to generate a `Parser` trait.
3. If there are 0 or 2+ attributes, it will fail.
For defining associated types will be used from type of field which marked as `parser` or `view_factory`.

Example:
```rust
#[derive(ViewFactory, Parser)]
struct ShowMyOrdersCallback {
    #[view_factory]
    #[parser]
    controller: StaticTextCallbackDataController<UserId>
}
```
There are will be used an `StaticTextCallbackDataController<UserId>::View` for defining `ShowMyOrdersCallback::View` type.

## `Schema`
Schema need generate 2 traits: `ViewFactory` and `Schema`.

### Generate `ViewFactory`
Generation of `ViewFactory` can be doing for non-empty `struct`. We need non-empty because we need at least 1 view to create a response for Telegram.

For generate `ViewFactory` for `struct` we need that all types of fields of deriving struct implements `ViewFactory` with same `View` type. Also we need a type of resulting `View`, that will be passed from attribute `view`.

Example:
```rust
#[derive(SchemaViewFactory)]
#[view = InlineKeyboard]
struct MainMenu {
    order_pizza : OrderPizzaCallback,
    show_my_orders : ShowMyOrdersCallback,
    log_out : LogOutCallback,
}
```
It will generate code from the [Schema](Schema-trait) section.

Note that we need that `View` from `view` attribute will have a `new` function that gives an `Vec` with array of `View` from `Callback`s.

### Generate `Schema`

Before generating `Schema` trait we need to generate `ViewFactory`, because it is for `Schema` trait. Also we need that all of fields of struct will implement `Callback` trait. Than we generate a `Schema` trait where we generate `Alternative` struct for all fields and then call `handle` method with `update`.

Example:
```rust
#[derive(SchemaViewFactory, Schema)]
#[view = InlineKeyboard]
struct MainMenu {
    order_pizza : OrderPizzaCallback,
    show_my_orders : ShowMyOrdersCallback,
    log_out : LogOutCallback,
}
```
It will generate code from the [Schema](#`Schema`_trait) section.



# Usability

## Simple text messages
For simple text messages must not use `ViewFactory`, because there are no view with possible actions. It need only `Parser` and `Handler` traits. For this we need to implement function that parse text and handle it. For using we can use `try_handle` function from `Callback` trait.

Example:
```rust
type UserId = i64;
type ChatId = i64;
struct BadWordsFilterCallback {
    bot: Bot,
    bad_words: Vec<String>,
}
impl Parser for BadWordsFilterCallback {
    type Input = Message;
    type Output = (UserId, ChatId);
    fn parse(&self, data: Self::Input) -> Result<Self::Output, Self::Input> {
        let text = data.text().ok_or_else(|| data)?;
        match self.bad_words.iter().any(|bad_word| text.contains(&bad_word)) {
            true => Ok(data.from.id),
            false => Err(data),
        }
    }
}
#[async_trait]
impl Handler for BadWordsFilterCallback {
    type Data = (UserId, UserId);
    async fn handle(&self, data: Self::Data) {
        let (user_id, chat_id) = data;
        self.bot.send_message(chat_id, "You use bad word!").await;
        self.bot.restrict_chat_member(chat_id, user_id).await;
    }
}
```

## Reply buttons
For Reply buttons we must create a `ViewFactory` that will construct the reply buttons and the `ReplyKeyboard` with an array of views. Then we must create a `Callback`s for every button type and `Schema` for keyboard. 

Example:
```rust
// from teloxide
struct ReplyButtonFactory;
impl ViewFactory for ReplyButtonFactory {
    type Ctx = String;
    type View = ReplyButton;
    fn construct(&self, data: Self::Ctx) -> Self::View {
        ReplyButton::new(data)
    }
}

// user code
struct WeatherData {
    city: String,
    temperature: f32,
}
impl Display for WeatherData { /* ommited */ }

type CityName = String;
type ChatId = i64;

struct ShowWeatherSchema<T> {
    client: WeatherClient,
    bot: Bot,
    factory: T,
    city_names: Vec<CityName>,
}
impl<T> ViewFactory for ShowWeatherSchema<T> where
    T: ViewFactory<Ctx = CityName, View = ReplyButton>
{
    type Ctx = ();
    type View = ReplyKeyboard;
    fn construct(&self, _: Self::Ctx) -> Self::View {
        ReplyKeyboard::new(
            self.city_names().iter().map(|city| self.factory.construct(city)).collect()
        )
    }
}
impl<T> Parser for ShowWeatherSchema<T> {
    type Input = Message;
    type Output = CityName;
    fn parse(&self, data: Self::Input) -> Result<Self::Output, Self::Input> {
        let text = data.text().ok_or_else(|| data)?;
        let mut city_name = None;
        for city in self.city_names.iter() {
            if text == city {
                city_name = Some(city);
                break;
            }
        }
        city_name.map(|name| (name, data.chat.id)).ok_or_else(|| data)
    }
}
impl<T> Handler for ShowWeatherSchema<T> {
    type Data = (CityName, ChatId);
    async fn handle(&self, data: Self::Data) {
        let (city_name, chat_id) = data;
        let weather: WeatherData = self.client.get_weather(city_name).await;
        self.bot.send_message(chat_id, weather.to_string()).send().await;
    }
}

```

## Inline buttons
Suppose that we create an `Schema` with main menu for pizza delivery, which can construct an `InlineKeyboard` and handle the `CallbackData` query. We must join it to:
1. `Handler` that handles the command to show main menu.
2. `Handler` that handles the `CallbackData`.

For the first item we can simply call the `construct` method that creates us a view, that will be sended to the user after.
For the second item, we can use `handle` that try to parse an update and handle it if parses was succesfull.

Examples you can see along this proposal.

## Commands
For commands we can define a `CommandParser` struct in library that will provide same functionality that provides `teloxide_macros` for `Command` trait now.

# Open questions
- Derive macros for enums for `ViewFactory`, `Parser`.
- Error handling in `Handler`.
- Support for inline queries.
