# Synchronisation

Let's make our own Semaphore!

A semaphore allows setting an upper limit on concurrency, which can be useful to reduce load on downstream servers.
However, downstream APIs are quite dynamic on how much load they can tolerate. We can use research on congestion control
to support dynamic and automatic limit detection. We will use [AIMD](https://en.wikipedia.org/wiki/Additive_increase/multiplicative_decrease) for simplicity.

We would like a semaphore that can add permits and remove permits at any time.
Tokio's semaphore allows adding permits at any time, and it does have an API to "forget" permits, but it's
unfortunately not sufficient as we might end up needing to forget more permits than are available

> For example, if the current limit is 100 and we want to set the new limit to 70, and there are 90 permits already taken,
> then tokio only lets us forget 10 permits.

How a semaphore works:
1. We need some state to tell us how many permits are available. In tokio this is always positive, but for our usecase this might be negative!
2. We need some way for tasks to wait, forming a queue, in the case that there are no permits available.
3. We need some way to notify one task in the queue that a permit is available after returning it.

In `lib.rs` we have our `AimdSemaphore` struct. 
* `aimd.state.limit() - aimd.acquired` is how we can determine how many permits are available.
* `notify: Notify` will be how we create the queue.
    * `notify.notified().await` allows us to wait for a notification.
    * `notify.notify_one()` allows us to notify one task.

Hint:
`Notify` will store up to 1 notification if no tasks are currently waiting.
