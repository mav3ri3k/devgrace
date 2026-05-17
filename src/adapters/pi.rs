use super::{AgentAdapter, AgentMessage};
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

pub struct PiAdapter;

impl AgentAdapter for PiAdapter {
    fn name(&self) -> &str {
        "pi"
    }

    fn data_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        if let Some(home) = dirs::home_dir() {
            let pi_dir = home.join(".pi").join("agent").join("sessions");
            if pi_dir.exists() {
                dirs.push(pi_dir);
            }
        }
        dirs
    }

    fn extract_messages(&self) -> Vec<AgentMessage> {
        let mut all_paths = Vec::new();
        for dir in self.data_dirs() {
            let pattern = dir.join("**").join("*.jsonl");
            if let Some(pattern_str) = pattern.to_str() {
                if let Ok(paths) = glob::glob(pattern_str) {
                    all_paths.extend(paths.flatten());
                }
            }
        }

        all_paths
            .par_iter()
            .flat_map_iter(|path| {
                let mut messages = Vec::new();
                if let Ok(content) = fs::read_to_string(path) {
                    for line in content.lines() {
                        if let Ok(event) = serde_json::from_str::<serde_json::Value>(line) {
                            if let Some(role) = event.get("role").and_then(|r| r.as_str()) {
                                if role == "user" {
                                    if let Some(content) = event.get("content") {
                                        let text = match content {
                                            serde_json::Value::String(s) => s.clone(),
                                            _ => String::new(),
                                        };
                                        if !text.is_empty() {
                                            messages.push(AgentMessage {
                                                text,
                                                agent: "pi".to_string(),
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                messages
            })
            .collect()
    }
}
