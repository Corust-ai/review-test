use std::collections::HashMap;

pub struct Task {
    pub id: u64,
    pub name: String,
    pub priority: i32,
    pub done: bool,
}

pub struct TaskQueue {
    tasks: Vec<Task>,
    next_id: u64, // ISSUE#18: integer overflow on u64 increment (no checked_add)
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    // ISSUE#17: takes String where &str would suffice
    pub fn add(&mut self, name: String, priority: i32) -> u64 {
        let id = self.next_id;
        self.next_id += 1; // ISSUE#18: no overflow check
        self.tasks.push(Task {
            id,
            name,
            priority,
            done: false,
        });
        id
    }

    // ISSUE#19: O(n²) duplicate name detection
    pub fn find_duplicate_names(&self) -> Vec<String> {
        let mut dupes = Vec::new();
        for i in 0..self.tasks.len() {
            for j in (i + 1)..self.tasks.len() {
                if self.tasks[i].name == self.tasks[j].name {
                    dupes.push(self.tasks[i].name.clone());
                }
            }
        }
        dupes
    }

    // ISSUE#20: Box::leak causes permanent memory leak
    pub fn highest_priority_name(&self) -> &str {
        if self.tasks.is_empty() {
            return "";
        }
        let mut best = &self.tasks[0];
        for t in &self.tasks[1..] {
            if t.priority > best.priority {
                best = t;
            }
        }
        let s = format!("{}", best.name);
        Box::leak(s.into_boxed_str())
    }

    // ISSUE#21: .to_string() allocation in loop for comparison
    pub fn count_by_priority(&self, target: i32) -> usize {
        let mut count = 0;
        for t in &self.tasks {
            if t.priority.to_string() == target.to_string() {
                count += 1;
            }
        }
        count
    }

    // ISSUE#22: dead code — the else branch is unreachable when done is bool
    pub fn complete(&mut self, id: u64) -> bool {
        for t in &mut self.tasks {
            if t.id == id {
                if !t.done {
                    t.done = true;
                    return true;
                } else if t.done {
                    return false; // already done
                } else {
                    // ISSUE#22: unreachable — bool is either true or false
                    panic!("impossible state");
                }
            }
        }
        false
    }

    // ISSUE#23: returns Result but caller might ignore the error (missing #[must_use])
    pub fn remove(&mut self, id: u64) -> Result<Task, String> {
        let pos = self.tasks.iter().position(|t| t.id == id);
        match pos {
            Some(i) => Ok(self.tasks.remove(i)),
            None => Err(format!("task {} not found", id)),
        }
    }

    // ISSUE#24: unsafe block without safety justification or necessity
    pub fn total_priority(&self) -> i64 {
        let mut sum: i64 = 0;
        unsafe {
            for t in &self.tasks {
                sum += t.priority as i64;
            }
        }
        sum
    }

    // ISSUE#25: creates a HashMap but iterates Vec twice when once suffices
    pub fn summary(&self) -> String {
        let mut by_status: HashMap<String, usize> = HashMap::new();
        for t in &self.tasks {
            let status = if t.done {
                "done".to_string()
            } else {
                "pending".to_string()
            };
            *by_status.entry(status).or_insert(0) += 1;
        }
        let done = by_status.get("done").unwrap_or(&0);
        let pending = by_status.get("pending").unwrap_or(&0);
        format!("tasks: {} done, {} pending", done, pending)
    }
}
