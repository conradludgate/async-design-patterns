use std::sync::Mutex;

use aimd::{Aimd, AimdConfig};
use tokio::sync::Notify;

mod aimd;

#[derive(Debug)]
pub struct AimdSemaphore {
    aimd: Mutex<State>,
    notify: Notify,
}

#[derive(Debug)]
struct State {
    /// how many permits are currently taken
    acquired: u64,
    /// the aimd state
    state: Aimd,
}

impl AimdSemaphore {
    pub fn new(config: AimdConfig) -> Self {
        let aimd = Aimd::new(config);
        let notify = Notify::new();

        // need to queue an initial notification.
        // Notify will store up to 1 available permit.
        notify.notify_one();

        Self {
            aimd: Mutex::new(State {
                acquired: 0,
                state: aimd,
            }),
            notify,
        }
    }

    pub fn success(&self) {
        let mut lock = self.aimd.lock().unwrap();

        lock.state.success();

        // we might have new permits available.
        if lock.acquired < lock.state.limit() {
            // queue a new notification for the next task
            self.notify.notify_one();
        }
    }

    pub fn failure(&self) {
        self.aimd.lock().unwrap().state.failure();
    }

    pub fn limit(&self) -> u64 {
        self.aimd.lock().unwrap().state.limit()
    }

    pub fn available(&self) -> u64 {
        let lock = self.aimd.lock().unwrap();

        // saturating as acquired can be greater than limit, if we experienced failures.
        lock.state.limit().saturating_sub(lock.acquired)
    }

    pub async fn acquire(&self) -> Permit<'_> {
        // should be waiting while acquired >= limit
        loop {
            // queue up for a notification
            self.notify.notified().await;

            let mut lock = self.aimd.lock().unwrap();
            if lock.acquired < lock.state.limit() {
                // claim a permit
                lock.acquired += 1;

                // queue a new notification for the next task
                self.notify.notify_one();
                break;
            }
        }

        Permit { sem: self }
    }
}

#[derive(Debug)]
pub struct Permit<'a> {
    sem: &'a AimdSemaphore,
}

impl Drop for Permit<'_> {
    fn drop(&mut self) {
        let mut lock = self.sem.aimd.lock().unwrap();
        lock.acquired -= 1;

        if lock.acquired < lock.state.limit() {
            // queue a new notification for the next task
            self.sem.notify.notify_one();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{aimd::AimdConfig, AimdSemaphore};

    #[tokio::test]
    async fn check() {
        let config = AimdConfig {
            min: 1,
            max: 10,
            inc: 1,
            dec: 0.5,
        };
        let sem = AimdSemaphore::new(config);

        assert_eq!(sem.available(), 10);

        let permit1 = sem.acquire().await;
        assert_eq!(sem.available(), 9);

        sem.failure();
        assert_eq!(sem.available(), 4);

        sem.success();
        assert_eq!(sem.available(), 5);

        sem.failure();
        assert_eq!(sem.available(), 2);

        let _permit2 = sem.acquire().await;
        let _permit3 = sem.acquire().await;
        assert_eq!(sem.available(), 0);

        tokio::time::timeout(Duration::from_millis(100), sem.acquire())
            .await
            .expect_err("should timeout while waiting for available permits");

        drop(permit1);
        assert_eq!(sem.available(), 1);
    }
}
