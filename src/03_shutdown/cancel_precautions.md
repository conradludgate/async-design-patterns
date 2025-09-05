# Cancellation Precautions

Sometimes APIs will assume cancel safety. This isn't always a bad thing, but it can cause unexpected bugs or
missed optimisations.

An example I had at Neon:
We operate a HTTP-based interface for postgres. We have a defensive implementation of a postgres connection pool
that needs to make sure the connection is in a stable state before returning to the pool (no in-flight transactions).

If the client cancels a HTTP request, then we will want to rollback any transactions and check that the connection
is steady, before retuning it to the pool. If we cannot complete these checks, we will discard the connection.

Most web frameworks in Rust will cancel handlers via drop if the HTTP request is cancelled, which can cause
us to discard a lot of postgres connections that would otherwise be easy to clean up. While there could
be many other ways to address this issue, the one we went with was utilising `CancellationToken` and drop guards.

```rust
let handler = service_fn(|req| async {
    let token = CancellationToken::new();
    // gracefully cancel the handler on drop
    guard = token.clone().drop_guard();

    // try and wait for the task to complete
    // spawned to allow it to continue making progress
    // in the background
    tokio::spawn(req_handler(req, token)).await;

    // if the task completed, there's nothing to cancel
    guard.disarm();
});
```

