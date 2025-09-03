use std::{future::Future, marker::PhantomData};

pub trait Actor: Send {
    type Req: Send;
    type Reply: Send;

    fn handle(&mut self, msg: Self::Req) -> impl Future<Output = Self::Reply> + Send;
}

// TODO: vvvvvv

pub fn actor_spawn<A: Actor>(_actor: A) -> MailboxRef<A> {
    todo!()
}

pub struct MailboxRef<A: Actor>(PhantomData<A>);

impl<A: Actor> MailboxRef<A> {
    pub async fn ask(&self, _req: A::Req) -> anyhow::Result<A::Reply> {
        todo!()
    }
}
