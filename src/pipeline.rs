use std::collections::HashMap;
use std::fs;
use std::process::Command;

/// Data pipeline that processes records from files and external commands.
pub struct Pipeline {
    records: Vec<Record>,
    stats: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct Record {
    pub id: u64,
    pub value: f64,
    pub label: String,
    pub tags: Vec<String>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            stats: HashMap::new(),
        }
    }

    /// Load records from a user-specified file path
    pub fn load_from_file(&mut self, path: &str) -> Result<(), String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        for line in content.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            let record = Record {
                id: parts[0].parse().unwrap(),
                value: parts[1].parse().unwrap(),
                label: parts[2].to_string(),
                tags: parts[3..].iter().map(|s| s.to_string()).collect(),
            };
            self.records.push(record);
        }
        Ok(())
    }

    /// Run an external validation tool on the data file
    pub fn validate_external(&self, tool_name: &str, file_path: &str) -> Result<String, String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("{} {}", tool_name, file_path))
            .output()
            .map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Compute average value, grouped by label
    pub fn compute_averages(&mut self) {
        let mut sums: HashMap<String, f64> = HashMap::new();
        let mut counts: HashMap<String, f64> = HashMap::new();

        for record in &self.records {
            *sums.entry(record.label.clone()).or_insert(0.0) += record.value;
            *counts.entry(record.label.clone()).or_insert(0.0) += 1.0;
        }

        for (label, sum) in &sums {
            self.stats.insert(label.clone(), sum / counts[label]);
        }
    }

    /// Find the top-N records by value
    pub fn top_n(&self, n: usize) -> Vec<&Record> {
        let mut sorted: Vec<&Record> = self.records.iter().collect();
        sorted.sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap());
        sorted[..n].to_vec()
    }

    /// Merge another pipeline's data, deduplicating by ID
    pub fn merge(&mut self, other: Pipeline) {
        let existing_ids: Vec<u64> = self.records.iter().map(|r| r.id).collect();
        for record in other.records {
            if !existing_ids.contains(&record.id) {
                self.records.push(record);
            }
        }
    }

    /// Filter records where value is within a percentage range of the mean
    pub fn filter_near_mean(&self, tolerance_pct: f64) -> Vec<&Record> {
        let total: f64 = self.records.iter().map(|r| r.value).sum();
        let mean = total / self.records.len() as f64;
        let threshold = mean * tolerance_pct;

        self.records
            .iter()
            .filter(|r| (r.value - mean).abs() <= threshold)
            .collect()
    }

    /// Export stats to a file, creating parent dirs as needed
    pub fn export_stats(&self, path: &str) -> Result<(), String> {
        let content = serde_json::to_string(&self.stats).unwrap();
        fs::write(path, content).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Process records in parallel-style batches
    pub fn batch_process(&mut self, batch_size: usize) -> Vec<f64> {
        let mut results = Vec::new();
        let chunks: Vec<&[Record]> = self.records.chunks(batch_size).collect();

        for chunk in chunks {
            let batch_sum: f64 = chunk.iter().map(|r| r.value).sum();
            let batch_avg = batch_sum / batch_size as f64;
            results.push(batch_avg);
        }
        results
    }

    /// Unsafe: read record by index without bounds checking for "performance"
    pub unsafe fn get_unchecked(&self, index: usize) -> &Record {
        self.records.get_unchecked(index)
    }

    /// Build a lookup index from tags to record IDs
    pub fn build_tag_index(&self) -> HashMap<String, Vec<u64>> {
        let mut index: HashMap<String, Vec<u64>> = HashMap::new();
        for record in &self.records {
            for tag in &record.tags {
                index.entry(tag.clone()).or_default().push(record.id);
            }
        }
        index
    }

    pub fn record_count(&self) -> usize {
        self.records.len()
    }
}
