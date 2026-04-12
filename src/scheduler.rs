use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// A task with a name, priority, and execution function.
pub struct Task {
    pub name: String,
    pub priority: u32,
    pub max_retries: u32,
    retries: u32,
    created_at: Instant,
}

/// Simple task scheduler that runs tasks by priority.
pub struct Scheduler {
    tasks: Vec<Task>,
    history: HashMap<String, Vec<Duration>>,
    max_concurrent: usize,
}

impl Scheduler {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            tasks: Vec::new(),
            history: HashMap::new(),
            max_concurrent,
        }
    }

    /// Add a task to the scheduler.
    pub fn add_task(&mut self, name: &str, priority: u32, max_retries: u32) {
        let task = Task {
            name: name.to_string(),
            priority,
            max_retries,
            retries: 0,
            created_at: Instant::now(),
        };
        self.tasks.push(task);
        self.tasks.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Get the next task to run.
    /// BUG 1: panics on empty queue (should return Option)
    pub fn next_task(&mut self) -> &Task {
        &self.tasks[0]
    }

    /// Remove a task by name.
    /// BUG 2: uses swap_remove which changes order, but we sorted by priority
    pub fn remove_task(&mut self, name: &str) -> bool {
        if let Some(pos) = self.tasks.iter().position(|t| t.name == name) {
            self.tasks.swap_remove(pos);
            true
        } else {
            false
        }
    }

    /// Calculate average execution time for a task name.
    /// BUG 3: division by zero when no history exists
    pub fn avg_execution_time(&self, name: &str) -> Duration {
        let times = &self.history[name];
        let total: Duration = times.iter().sum();
        total / times.len() as u32
    }

    /// Record that a task completed.
    pub fn record_completion(&mut self, name: &str, duration: Duration) {
        self.history
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(duration);
    }

    /// Retry a failed task.
    /// BUG 4: modifies task through shared reference (won't compile,
    /// but if it did, retries counter is never checked against max_retries)
    pub fn retry_task(&mut self, name: &str) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.name == name) {
            task.retries += 1;
            true
        } else {
            false
        }
    }

    /// Get tasks older than a given duration.
    pub fn stale_tasks(&self, max_age: Duration) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.created_at.elapsed() > max_age)
            .collect()
    }

    /// Load tasks from a config string (one task per line: "name,priority,retries")
    /// BUG 5: unwrap on parse, panics on malformed input
    pub fn load_from_config(&mut self, config: &str) {
        for line in config.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split(',').collect();
            let name = parts[0].trim();
            let priority: u32 = parts[1].trim().parse().unwrap();
            let max_retries: u32 = parts[2].trim().parse().unwrap();
            self.add_task(name, priority, max_retries);
        }
    }

    /// Export task names to a file
    /// BUG 6: path traversal — user controls the path
    pub fn export_to_file(&self, path: &str) -> std::io::Result<()> {
        let content: String = self.tasks
            .iter()
            .map(|t| format!("{},{},{}", t.name, t.priority, t.max_retries))
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(path, content)
    }
}

/// Shared scheduler for multi-threaded use.
/// BUG 7: the background cleanup holds the lock across sleep,
/// blocking all other threads for 60 seconds
pub fn start_cleanup_thread(scheduler: Arc<Mutex<Scheduler>>, max_age: Duration) {
    std::thread::spawn(move || {
        loop {
            let mut sched = scheduler.lock().unwrap();
            let stale: Vec<String> = sched
                .stale_tasks(max_age)
                .iter()
                .map(|t| t.name.clone())
                .collect();
            for name in &stale {
                sched.remove_task(name);
            }
            // BUG: holds lock during sleep!
            std::thread::sleep(Duration::from_secs(60));
        }
    });
}
