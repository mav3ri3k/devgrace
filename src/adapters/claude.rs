use super::{AgentAdapter, AgentMessage};
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

pub struct ClaudeAdapter;

impl AgentAdapter for ClaudeAdapter {
    fn name(&self) -> &str {
        "claude"
    }

    fn data_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        if let Some(home) = dirs::home_dir() {
            let claude_dir = home.join(".claude").join("projects");
            if claude_dir.exists() {
                dirs.push(claude_dir);
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
                            if let Some(msg_type) = event.get("type").and_then(|t| t.as_str()) {
                                if msg_type == "human" || msg_type == "user" {
                                    if let Some(message) = event.get("message") {
                                        if let Some(content) = message.get("content") {
                                            let text = extract_text(content);
                                            if !text.is_empty() {
                                                messages.push(AgentMessage {
                                                    text,
                                                    agent: "claude".to_string(),
                                                });
                                            }
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

fn extract_text(content: &serde_json::Value) -> String {
    match content {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(arr) => {
            let mut parts = Vec::new();
            for item in arr {
                if let Some(obj) = item.as_object() {
                    if obj.get("type").and_then(|t| t.as_str()) == Some("text") {
                        if let Some(text) = obj.get("text").and_then(|t| t.as_str()) {
                            parts.push(text.to_string());
                        }
                    }
                }
            }
            parts.join(" ")
        }
        _ => String::new(),
    }
}
