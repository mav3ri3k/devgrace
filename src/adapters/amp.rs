use super::{AgentAdapter, AgentMessage};
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

pub struct AmpAdapter;

impl AgentAdapter for AmpAdapter {
    fn name(&self) -> &str {
        "amp"
    }

    fn data_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        if let Some(home) = dirs::home_dir() {
            let amp_dir = home.join(".local").join("share").join("amp");
            if amp_dir.exists() {
                dirs.push(amp_dir);
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

            let pattern = dir.join("threads").join("*.json");
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
                    if path.extension().and_then(|ext| ext.to_str()) == Some("jsonl") {
                        for line in content.lines() {
                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(line) {
                                if let Some(text) = val.get("text").and_then(|t| t.as_str()) {
                                    if !text.is_empty() {
                                        messages.push(AgentMessage {
                                            text: text.to_string(),
                                            agent: "amp".to_string(),
                                        });
                                    }
                                }
                            }
                        }
                    } else if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                        messages.extend(extract_amp_messages(&val));
                    }
                }
                messages
            })
            .collect()
    }
}

fn extract_amp_messages(val: &serde_json::Value) -> Vec<AgentMessage> {
    let mut messages = Vec::new();

    if let Some(threads) = val.get("threads").and_then(|t| t.as_array()) {
        for thread in threads {
            messages.extend(extract_amp_messages_from_array(thread.get("messages")));
        }
    }

    messages.extend(extract_amp_messages_from_array(val.get("messages")));
    messages.extend(extract_amp_messages_from_array(Some(val)));

    messages
}

fn extract_amp_messages_from_array(val: Option<&serde_json::Value>) -> Vec<AgentMessage> {
    let mut messages = Vec::new();

    let Some(msgs) = val.and_then(|v| v.as_array()) else {
        return messages;
    };

    for msg in msgs {
        if msg.get("role").and_then(|r| r.as_str()) == Some("user") {
            if let Some(content) = msg.get("content") {
                let text = extract_amp_text(content);
                if !text.is_empty() {
                    messages.push(AgentMessage {
                        text,
                        agent: "amp".to_string(),
                    });
                }
            }
        }
    }

    messages
}

fn extract_amp_text(content: &serde_json::Value) -> String {
    match content {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(arr) => arr
            .iter()
            .filter_map(|part| part.get("text").and_then(|t| t.as_str()))
            .collect::<Vec<_>>()
            .join(" "),
        _ => String::new(),
    }
}
