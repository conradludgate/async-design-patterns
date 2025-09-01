use p01_actor::{Actor, actor_spawn};

struct Counter {
    count: i64,
}

enum Message {
    Inc { amount: i64 },
}

#[derive(Debug, PartialEq, Eq)]
enum Response {
    Value { amount: i64 },
}

impl Actor for Counter {
    type Req = Message;
    type Reply = Response;

    async fn handle(&mut self, msg: Message) -> Self::Reply {
        match msg {
            Message::Inc { amount } => {
                self.count += amount;
                Response::Value { amount: self.count }
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Spawn the actor and obtain its reference
    let actor_ref = actor_spawn(Counter { count: 0 });

    // Send messages to the actor
    let count = actor_ref.ask(Message::Inc { amount: 42 }).await?;
    assert_eq!(count, Response::Value { amount: 42 });

    Ok(())
}
