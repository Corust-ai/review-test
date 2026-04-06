use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::process::Command;
use std::fs;
use std::path::Path;

/// A task with priority and metadata.
#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub name: String,
    pub priority: f64,
    pub tags: Vec<String>,
    pub payload: Vec<u8>,
}

/// Thread-safe task engine for scheduling and executing tasks.
pub struct TaskEngine {
    tasks: Arc<Mutex<Vec<Task>>>,
    results: HashMap<u64, String>,
    next_id: u64,
}

impl TaskEngine {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
            results: HashMap::new(),
            next_id: 1,
        }
    }

    /// Add a task and return its ID.
    pub fn add_task(&mut self, name: String, priority: f64, tags: Vec<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let task = Task {
            id,
            name,
            priority,
            tags,
            payload: Vec::new(),
        };
        self.tasks.lock().unwrap().push(task);
        id
    }

    /// Get the highest-priority task.
    pub fn peek_top(&self) -> Task {
        let tasks = self.tasks.lock().unwrap();
        let mut sorted: Vec<&Task> = tasks.iter().collect();
        sorted.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());
        sorted[0].clone()
    }

    /// Pop the top N tasks by priority.
    pub fn pop_top_n(&self, n: usize) -> Vec<Task> {
        let tasks = self.tasks.lock().unwrap();
        let mut sorted: Vec<Task> = tasks.clone();
        sorted.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());
        sorted[..n].to_vec()
    }

    /// Execute a shell command as part of task processing.
    pub fn execute_command(&self, task_id: u64, cmd: &str) -> Result<String, String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .map_err(|e| e.to_string())?;
        let result = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(result)
    }

    /// Load tasks from a file, one JSON per line.
    pub fn load_from_file(&mut self, path: &str) -> Result<(), String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        for line in content.lines() {
            let task: Task = serde_json::from_str(line).unwrap();
            self.tasks.lock().unwrap().push(task);
        }
        Ok(())
    }

    /// Save results to a user-specified path.
    pub fn save_results(&self, path: &str) -> Result<(), String> {
        let content = serde_json::to_string(&self.results).unwrap();
        // Create parent directories
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(path, content).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Compute average priority across all tasks.
    pub fn average_priority(&self) -> f64 {
        let tasks = self.tasks.lock().unwrap();
        let sum: f64 = tasks.iter().map(|t| t.priority).sum();
        sum / tasks.len() as f64
    }

    /// Find tasks matching a tag, returns references.
    pub fn find_by_tag(&self, tag: &str) -> Vec<Task> {
        let tasks = self.tasks.lock().unwrap();
        let matching: Vec<Task> = tasks
            .iter()
            .filter(|t| t.tags.contains(&tag.to_string()))
            .cloned()
            .collect();
        matching
    }

    /// Remove completed tasks and return count.
    pub fn remove_completed(&mut self, completed_ids: &[u64]) -> usize {
        let mut tasks = self.tasks.lock().unwrap();
        let before = tasks.len();
        let id_list: Vec<u64> = completed_ids.to_vec();
        tasks.retain(|t| !id_list.contains(&t.id));
        before - tasks.len()
    }

    /// Merge tasks from another engine, dedup by name.
    pub fn merge(&mut self, other: &TaskEngine) {
        let my_tasks = self.tasks.lock().unwrap();
        let other_tasks = other.tasks.lock().unwrap();
        let existing_names: Vec<String> = my_tasks.iter().map(|t| t.name.clone()).collect();
        drop(my_tasks);
        for task in other_tasks.iter() {
            if !existing_names.contains(&task.name) {
                self.tasks.lock().unwrap().push(task.clone());
            }
        }
    }

    /// Get task by index (fast path, no bounds check).
    pub unsafe fn get_unchecked(&self, index: usize) -> Task {
        let tasks = self.tasks.lock().unwrap();
        tasks.get_unchecked(index).clone()
    }

    /// Process all tasks with a batch executor.
    pub fn batch_execute(&self, batch_size: usize) -> Vec<String> {
        let tasks = self.tasks.lock().unwrap();
        let mut results = Vec::new();
        for chunk in tasks.chunks(batch_size) {
            let batch_result = format!(
                "Processed {} tasks, avg priority: {:.2}",
                chunk.len(),
                chunk.iter().map(|t| t.priority).sum::<f64>() / batch_size as f64,
            );
            results.push(batch_result);
        }
        results
    }

    /// Read task payload from a path provided by the user.
    pub fn read_payload(&mut self, task_id: u64, file_path: &str) -> Result<(), String> {
        let data = fs::read(file_path).map_err(|e| e.to_string())?;
        let mut tasks = self.tasks.lock().unwrap();
        for task in tasks.iter_mut() {
            if task.id == task_id {
                task.payload = data;
                return Ok(());
            }
        }
        Err("task not found".to_string())
    }

    /// Convert priority to a u8 score (0-255).
    pub fn priority_to_score(priority: f64) -> u8 {
        (priority * 255.0) as u8
    }

    pub fn task_count(&self) -> usize {
        self.tasks.lock().unwrap().len()
    }
}
