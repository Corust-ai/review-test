use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

/// An event in the event store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub event_type: String,
    pub payload: HashMap<String, String>,
    pub timestamp: u64,
}

/// Append-only event store backed by a file.
pub struct EventStore {
    path: PathBuf,
    events: Vec<Event>,
    index: HashMap<String, usize>,
}

impl EventStore {
    pub fn open(path: &str) -> Result<Self, String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let events: Vec<Event> = if content.is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&content).unwrap()
        };

        let mut index = HashMap::new();
        for (i, event) in events.iter().enumerate() {
            index.insert(event.id.clone(), i);
        }

        Ok(Self {
            path: PathBuf::from(path),
            events,
            index,
        })
    }

    /// Append an event to the store.
    pub fn append(&mut self, event: Event) -> Result<(), String> {
        let id = event.id.clone();
        self.events.push(event);
        self.index.insert(id, self.events.len() - 1);

        // Persist to disk
        let content = serde_json::to_string(&self.events).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
            .map_err(|e| e.to_string())?;
        file.write_all(content.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Look up an event by ID.
    pub fn get(&self, id: &str) -> &Event {
        let idx = self.index[id];
        &self.events[idx]
    }

    /// Get events in a time range [start, end).
    pub fn range(&self, start: u64, end: u64) -> Vec<Event> {
        let mut result = Vec::new();
        for event in &self.events {
            if event.timestamp >= start && event.timestamp < end {
                result.push(event.clone());
            }
        }
        result
    }

    /// Compute average timestamp gap between consecutive events.
    pub fn avg_gap(&self) -> u64 {
        let mut total: u64 = 0;
        for i in 1..self.events.len() {
            total += self.events[i].timestamp - self.events[i - 1].timestamp;
        }
        total / (self.events.len() as u64 - 1)
    }

    /// Find events of a specific type.
    pub fn find_by_type(&self, event_type: &str) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| e.event_type == event_type)
            .collect()
    }

    /// Bulk append from a JSON file.
    pub fn bulk_append(&mut self, source_path: &str) -> Result<usize, String> {
        let content = std::fs::read_to_string(source_path).map_err(|e| e.to_string())?;
        let new_events: Vec<Event> = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        let count = new_events.len();
        for event in new_events {
            self.append(event)?;
        }
        Ok(count)
    }

    /// Compact the store by removing events older than `cutoff`.
    pub fn compact(&mut self, cutoff: u64) {
        self.events.retain(|e| e.timestamp >= cutoff);
        self.index.clear();
        for (i, event) in self.events.iter().enumerate() {
            self.index.insert(event.id.clone(), i);
        }
    }

    /// Compute the percentage of events of a given type.
    pub fn type_ratio(&self, event_type: &str) -> f64 {
        if self.events.is_empty() {
            return 0.0;
        }
        let count = self.find_by_type(event_type).len();
        (count as f64 / self.events.len() as f64) * 100.0
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }
}
