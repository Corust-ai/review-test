use std::collections::HashSet;

pub struct Task {
    pub id: u64,
    pub name: String,
    pub priority: i32,
    pub done: bool,
}

pub struct TaskQueue {
    tasks: Vec<Task>,
    next_id: u64,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, name: &str, priority: i32) -> Result<u64, String> {
        let id = self.next_id.checked_add(1)
            .ok_or_else(|| "task id overflow".to_string())?;
        self.next_id = id;
        self.tasks.push(Task {
            id,
            name: name.to_owned(),
            priority,
            done: false,
        });
        Ok(id)
    }

    pub fn find_duplicate_names(&self) -> Vec<String> {
        let mut seen = HashSet::new();
        let mut dupes = HashSet::new();
        for t in &self.tasks {
            if !seen.insert(&t.name) {
                dupes.insert(t.name.clone());
            }
        }
        dupes.into_iter().collect()
    }

    pub fn highest_priority_name(&self) -> Option<&str> {
        self.tasks.iter()
            .max_by_key(|t| t.priority)
            .map(|t| t.name.as_str())
    }

    pub fn count_by_priority(&self, target: i32) -> usize {
        self.tasks.iter().filter(|t| t.priority == target).count()
    }

    pub fn complete(&mut self, id: u64) -> bool {
        for t in &mut self.tasks {
            if t.id == id && !t.done {
                t.done = true;
                return true;
            }
        }
        false
    }

    #[must_use]
    pub fn remove(&mut self, id: u64) -> Result<Task, String> {
        let pos = self.tasks.iter().position(|t| t.id == id);
        match pos {
            Some(i) => Ok(self.tasks.remove(i)),
            None => Err(format!("task {} not found", id)),
        }
    }

    pub fn total_priority(&self) -> i64 {
        self.tasks.iter().map(|t| t.priority as i64).sum()
    }

    pub fn summary(&self) -> String {
        let done = self.tasks.iter().filter(|t| t.done).count();
        let pending = self.tasks.iter().filter(|t| !t.done).count();
        format!("tasks: {} done, {} pending", done, pending)
    }
}
