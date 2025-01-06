use future01::future::Future;
use future01::task::{self, Task};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tracing::info;

pub struct TenPolls {
    name: String,
    count: Arc<Mutex<u32>>,
    task: Arc<Mutex<Option<Task>>>,
}

impl TenPolls {
    pub fn new(name: String) -> Self {
        TenPolls {
            name,
            count: Arc::new(Mutex::new(0)),
            task: Arc::new(Mutex::new(None)),
        }
    }

    // Helper method to spawn background task for notifications
    fn setup_notifier(&self) {
        let task = self.task.clone();
        let count = self.count.clone();

        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(100));

            let current_count = *count.lock().unwrap();
            if current_count >= 10 {
                break;
            }

            if let Some(ref task) = *task.lock().unwrap() {
                task.notify();
            }
        });
    }
}

impl Future for TenPolls {
    type Item = String;
    type Error = ();

    fn poll(&mut self) -> future01::Poll<Self::Item, Self::Error> {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        info!("Polled {} times", *count);

        if *count >= 10 {
            Ok(future01::Async::Ready(format!("hello {}", self.name)))
        } else {
            // Store the current task for later wakeup
            *self.task.lock().unwrap() = Some(task::current());

            // If this is the first poll, setup the notifier
            if *count == 1 {
                self.setup_notifier();
            }

            Ok(future01::Async::NotReady)
        }
    }
}
