use std::future::Future;

use anyhow::Context;
use tokio::sync::{mpsc, oneshot};

pub trait Actor: Send + 'static {
    type Req: Send + 'static;
    type Reply: Send + 'static;

    fn handle(&mut self, msg: Self::Req) -> impl Future<Output = Self::Reply> + Send;
}

// TODO: vvvvvv

pub fn actor_spawn<A: Actor>(mut actor: A) -> MailboxRef<A> {
    let (tx, mut rx) = mpsc::channel::<(A::Req, oneshot::Sender<A::Reply>)>(10);
    tokio::spawn(async move {
        loop {
            let Some((msg, tx)) = rx.recv().await else {
                break;
            };

            let resp = actor.handle(msg).await;
            let _ = tx.send(resp);
        }
    });

    MailboxRef { tx }
}

pub struct MailboxRef<A: Actor> {
    tx: mpsc::Sender<(A::Req, oneshot::Sender<A::Reply>)>,
}

impl<A: Actor> MailboxRef<A> {
    pub async fn ask(&self, req: A::Req) -> anyhow::Result<A::Reply> {
        let (tx, rx) = oneshot::channel();
        self.tx.send((req, tx)).await.unwrap();
        let resp = rx.await.context("actor failed to respond")?;
        Ok(resp)
    }
}
