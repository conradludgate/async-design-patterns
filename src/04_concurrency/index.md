# Chapter 4 - Concurrency

Spawning multiple unrelated tasks is one thing,
but sometimes you want to process multiple related but independant tasks concurrently to reduce latencies.
Fork-Join, Map-Reduce, whatever you call it, how would be best represent this in async Rust?

It's important to establish what we want to process concurrently, and what even we mean by concurrent.

## Concurrency vs Parallelism

If you needed to transcode 8 different video streams because you're building a competitor to youtube,
then async rust won't really help you. Instead, you'd prefer something like rayon, since you need pure parallelism.

Async Rust really shines when you mostly need to wait, and when most of the time is spent waiting,
you can use that time waiting to queue up other tasks. This is where the concurrency shines. In our
case, we might use that to issue 10 HTTP requests at the same time, as most of that time is spent waiting.

## Channel

One approach we might introduce is to use mutli-produce single-consumer channels,
and spawn a tokio task per item we want to dispatch.

Channels are self-closing, which allows us to easily detect when all tasks are complete, so
we don't need anything else like a TaskTracker.

```rust
let (tx, mut rx) = tokio::sync::mpsc::channel(1);

for item in items {
    let tx = tx.clone();
    tokio::spawn(async move {
        tx.send(handle(item).await)
    });
}

drop(tx);

let mut responses = vec![];
while let Some(resp) = rx.recv().await {
    responses.push(resp);
}
```

## JoinSet

This is again so common that tokio provides a dedicated utility for this, [`JoinSet`](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html)

```rust
let mut joinset = JoinSet::new();

for item in items {
    joinset.spawn(async move {
        handle(item).await
    });
}

let mut responses = vec![];
while let Some(resp) = joinset.join_next().await.unwrap() {
    responses.push(resp);
}
```

## FuturesUnordered

There's one issue with using spawn for concurrency that can sometimes be an issue, `'static`.
All the tasks might need to share some state that is owned by the parent task.
There's currently no sound way (ask me later/tomorrow) to spawn a tokio task such that
it can borrow from it's parent. Fortunately, you don't need to spawn a task to use concurrency.

We saw earlier how `select!` can run tasks concurrently, and that works without spawn. Unfortunately,
that doesn't scale for dynamic tasks, so we will need something else but something similar.
From the `futures` crate, you can find [`futures::FuturesUnordered`](https://docs.rs/futures/latest/futures/prelude/stream/struct.FuturesUnordered.html). It works very similarly to the `JoinSet`

```rust
let mut futures = FuturesUnordered::new();

for item in items {
    futures.push(async move { handle(item).await });
}

let mut responses = vec![];
while let Some(resp) = futures.next().await {
    responses.push(resp);
}
```

The futures crate has some extra goodies that makes this pattern simpler, since it is again so common:

```rust
futures::stream::iter(items)
    .map(|item| async move { handle(item).await })
    .buffer_unordered(10)
    .collect::<Vec<_>>()
    .await
```

There's a risk here, however. <https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/barbara_battles_buffered_streams.html>
