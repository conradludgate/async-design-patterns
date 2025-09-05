# Signals

When running applications in a managed environment like docker or kubernetes, a shutdown request
might be received as a `SIGTERM`. This isn't cross-platform, so we're not using it here today,
but keep in mind that in a real webserver you might want to use

```rust
use tokio::signal::unix::{signal, SignalKind};

let mut sigterm = signal(SignalKind::terminate())
    .expect("signal handler should be registered");

sigterm.recv()
    .await
    .expect("signal handler should not be disconnected");
```

---

Windows also has an alternative to SIGTERM, `tokio::signal::windows::ctrl_shutdown()`
