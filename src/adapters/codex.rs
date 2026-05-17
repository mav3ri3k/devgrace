use super::{AgentAdapter, AgentMessage};
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

pub struct CodexAdapter;

impl AgentAdapter for CodexAdapter {
    fn name(&self) -> &str {
        "codex"
    }

    fn data_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        if let Some(home) = dirs::home_dir() {
            let codex_home = home.join(".codex");
            if codex_home.exists() {
                dirs.push(codex_home);
            }
        }
        dirs
    }

    fn extract_messages(&self) -> Vec<AgentMessage> {
        let mut all_paths = Vec::new();
        for dir in self.data_dirs() {
            let history = dir.join("history.jsonl");
            if history.exists() {
                all_paths.push(history);
            }

            let pattern = dir.join("sessions").join("**").join("*.jsonl");
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
                            if let Some(text) = event.get("text").and_then(|t| t.as_str()) {
                                if !text.is_empty() {
                                    messages.push(AgentMessage {
                                        text: text.to_string(),
                                        agent: "codex".to_string(),
                                    });
                                }
                                continue;
                            }

                            if let Some(role) = event.get("role").and_then(|r| r.as_str()) {
                                if role == "user" {
                                    if let Some(content) = event.get("content") {
                                        let text = extract_codex_text(content);
                                        if !text.is_empty() {
                                            messages.push(AgentMessage {
                                                text,
                                                agent: "codex".to_string(),
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

fn extract_codex_text(content: &serde_json::Value) -> String {
    match content {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(arr) => {
            let mut parts = Vec::new();
            for item in arr {
                if let Some(obj) = item.as_object() {
                    if obj.get("type").and_then(|t| t.as_str()) == Some("input_text") {
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
