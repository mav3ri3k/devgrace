use super::{AgentAdapter, AgentMessage};
use rayon::prelude::*;
use std::path::PathBuf;

pub struct ZedAdapter;

impl AgentAdapter for ZedAdapter {
    fn name(&self) -> &str {
        "zed"
    }

    fn data_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        if let Some(home) = dirs::home_dir() {
            let mac_dir = home
                .join("Library")
                .join("Application Support")
                .join("Zed")
                .join("sessions");
            if mac_dir.exists() {
                dirs.push(mac_dir);
            }
            let linux_dir = home.join(".local").join("share").join("zed").join("sessions");
            if linux_dir.exists() {
                dirs.push(linux_dir);
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
                if let Ok(content) = std::fs::read_to_string(path) {
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                        messages.extend(extract_zed_messages(&val));
                    }
                }
                messages
            })
            .collect()
    }
}

fn extract_zed_messages(val: &serde_json::Value) -> Vec<AgentMessage> {
    let mut messages = Vec::new();

    if let Some(msgs) = val.get("messages").and_then(|m| m.as_array()) {
        for msg in msgs {
            if let Some(role) = msg.get("role").and_then(|r| r.as_str()) {
                if role == "user" {
                    if let Some(content) = msg.get("content") {
                        let text = match content {
                            serde_json::Value::String(s) => s.clone(),
                            serde_json::Value::Array(arr) => {
                                let mut parts = Vec::new();
                                for part in arr {
                                    if let Some(obj) = part.as_object() {
                                        if let Some(t) = obj.get("text").and_then(|t| t.as_str()) {
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
                                agent: "zed".to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    messages
}
