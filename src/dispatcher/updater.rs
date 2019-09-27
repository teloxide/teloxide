use std::{
    pin::Pin,
    task::{Context, Poll},
};

use pin_project::pin_project;
use futures::{Stream, StreamExt, stream};

use crate::{
    bot::Bot,
    requests::Request,
    types::Update,
    RequestError,
};

// Currently just a placeholder, but I'll  add here some methods
pub trait Updater<E>: Stream<Item=Result<Update, E>> {}

#[pin_project]
pub struct StreamUpdater<S> {
    #[pin]
    stream: S
}

impl<S> StreamUpdater<S> {
    pub fn new(stream: S) -> Self {
        Self { stream }
    }
}

impl<S, E> Stream for StreamUpdater<S> where S: Stream<Item=Result<Update, E>> {
    type Item = Result<Update, E>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().stream.poll_next(cx)
    }
}

impl<S, E> Updater<E> for StreamUpdater<S> where S: Stream<Item=Result<Update, E>> {}

pub fn polling<'a>(bot: &'a Bot) -> impl Updater<RequestError> + 'a/*StreamUpdater<impl Stream<Item=ResponseResult<Update>> + 'a>*/ {
    let stream = stream::unfold((bot, 0), |(bot, mut offset)| async move {
        // this match converts Result<Vec<_>, _> -> Vec<Result<_, _>>
        let updates = match bot.get_updates().offset(offset).send().await {
            Ok(updates) => {
                if let Some(upd) = updates.last() {
                    offset = upd.id + 1;
                }
                updates.into_iter().map(|u| Ok(u)).collect::<Vec<_>>()
            },
            Err(err) => vec![Err(err)]
        };
        Some((stream::iter(updates), (bot, offset)))
    })
        .flatten();

    StreamUpdater { stream }
}

// TODO implement webhook (this actually require webserver and probably we
//   should add cargo feature that adds webhook)
//pub fn webhook<'a>(bot: &'a Bot, cfg: WebhookConfig) -> Updater<impl Stream<Item=Result<Update, ???>> + 'a> {}
