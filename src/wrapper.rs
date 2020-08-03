use crate::requests::{Pin, Request, ResponseResult};
use futures::{
    future::BoxFuture,
    task::{Context, Poll},
    Future,
};

pub struct Wrapper<T: Request + 'static> {
    fut: BoxFuture<'static, ResponseResult<T::Output>>,
}

impl<T: Request + 'static> Wrapper<T> {
    pub fn new(t: T) -> Wrapper<T> {
        Wrapper { fut: Box::pin(t.send()) }
    }
}

impl<T: Request + 'static> Future for Wrapper<T> {
    type Output = ResponseResult<T::Output>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.fut.as_mut().poll(cx)
    }
}

#[tokio::test]
async fn foo() {
    let res = Wrapper::new(
        SendMessage::new(BotBuilder::new().token("1").build(), 1, "text")
            .disable_notification(true),
    )
    .await;
    println!("{:?}", res);
}
