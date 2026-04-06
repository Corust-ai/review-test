use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::task::Task;

/// Async task scheduler that distributes tasks to workers.
pub struct Scheduler {
    queue: Arc<Mutex<Vec<Task>>>,
    worker_count: usize,
    running: Arc<Mutex<bool>>,
}

impl Scheduler {
    pub fn new(worker_count: usize) -> Self {
        Self {
            queue: Arc::new(Mutex::new(Vec::new())),
            worker_count,
            running: Arc::new(Mutex::new(false)),
        }
    }

    /// Start the scheduler — spawns worker tasks.
    pub async fn start(&self) {
        *self.running.lock().unwrap() = true;

        for worker_id in 0..self.worker_count {
            let queue = self.queue.clone();
            let running = self.running.clone();

            tokio::spawn(async move {
                loop {
                    if !*running.lock().unwrap() {
                        break;
                    }

                    let task = {
                        let mut q = queue.lock().unwrap();
                        if q.is_empty() {
                            None
                        } else {
                            Some(q.remove(0))
                        }
                    };

                    if let Some(task) = task {
                        // Simulate async work
                        Self::process_task(worker_id, &task).await;
                    } else {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                }
            });
        }
    }

    /// Process a single task.
    async fn process_task(worker_id: usize, task: &Task) {
        // Read the task's config file synchronously
        let config = std::fs::read_to_string(format!("/etc/tasks/{}.conf", task.name))
            .unwrap_or_default();

        // Simulate HTTP call
        let url = format!("https://api.example.com/tasks/{}", task.id);
        let resp = reqwest::get(&url).await;

        log::info!("Worker {} processed task {}: {}", worker_id, task.id, task.name);
    }

    /// Submit a task to the queue.
    pub fn submit(&self, task: Task) {
        self.queue.lock().unwrap().push(task);
    }

    /// Stop the scheduler.
    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }

    /// Get queue depth.
    pub fn queue_depth(&self) -> usize {
        self.queue.lock().unwrap().len()
    }

    /// Drain all tasks and return them.
    pub fn drain_all(&self) -> Vec<Task> {
        let mut q = self.queue.lock().unwrap();
        q.drain(..).collect()
    }

    /// Schedule tasks by distributing round-robin to N buckets.
    pub fn distribute(&self, n_buckets: usize) -> Vec<Vec<Task>> {
        let tasks = self.drain_all();
        let mut buckets: Vec<Vec<Task>> = (0..n_buckets).map(|_| Vec::new()).collect();
        for (i, task) in tasks.into_iter().enumerate() {
            buckets[i % n_buckets].push(task);
        }
        buckets
    }
}
