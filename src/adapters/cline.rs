use super::{AgentAdapter, AgentMessage};
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

pub struct ClineAdapter;

impl AgentAdapter for ClineAdapter {
    fn name(&self) -> &str {
        "cline"
    }

    fn data_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        if let Some(home) = dirs::home_dir() {
            let cline_dir = home.join(".cline");
            if cline_dir.exists() {
                dirs.push(cline_dir);
            }
        }
        dirs
    }

    fn extract_messages(&self) -> Vec<AgentMessage> {
        let mut all_paths = Vec::new();
        for dir in self.data_dirs() {
            let pattern = dir.join("**").join("*.json");
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
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(arr) = val.as_array() {
                            for item in arr {
                                if let Some(role) = item.get("role").and_then(|r| r.as_str()) {
                                    if role == "user" {
                                        if let Some(text_val) = item.get("content") {
                                            let text = match text_val {
                                                serde_json::Value::String(s) => s.clone(),
                                                serde_json::Value::Array(arr) => {
                                                    let mut parts = Vec::new();
                                                    for part in arr {
                                                        if let Some(obj) = part.as_object() {
                                                            if let Some(t) = obj
                                                                .get("text")
                                                                .and_then(|t| t.as_str())
                                                            {
                                                                parts.push(t.to_string());
                                                            }
                                                        }
                                                    }
                                                    parts.join(" ")
                                                }
                                                _ => String::new(),
                                            };
                                            if !text.is_empty() {
                                                messages.push(AgentMessage {
                                                    text,
                                                    agent: "cline".to_string(),
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
