use super::{AgentAdapter, AgentMessage};
use rayon::prelude::*;
use std::path::PathBuf;

pub struct OpenCodeAdapter;

impl AgentAdapter for OpenCodeAdapter {
    fn name(&self) -> &str {
        "opencode"
    }

    fn data_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        if let Some(home) = dirs::home_dir() {
            let mac_dir = home
                .join("Library")
                .join("Application Support")
                .join("opencode");
            if mac_dir.exists() {
                dirs.push(mac_dir);
            }
            let linux_dir = home.join(".local").join("share").join("opencode");
            if linux_dir.exists() {
                dirs.push(linux_dir);
            }
        }
        dirs
    }

    fn extract_messages(&self) -> Vec<AgentMessage> {
        let mut messages = Vec::new();
        for dir in self.data_dirs() {
            let db_path = dir.join("opencode.db");
            if db_path.exists() {
                messages.extend(extract_from_sqlite(&db_path));
            }
            let pattern = dir.join("**").join("*.json");
            if let Some(pattern_str) = pattern.to_str() {
                if let Ok(paths) = glob::glob(pattern_str) {
                    let json_paths: Vec<_> = paths.flatten().collect();
                    let json_msgs: Vec<AgentMessage> = json_paths
                        .par_iter()
                        .flat_map_iter(|path| {
                            let mut msgs = Vec::new();
                            if let Ok(content) = std::fs::read_to_string(path) {
                                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content)
                                {
                                    msgs.extend(extract_json_messages(&val));
                                }
                            }
                            msgs
                        })
                        .collect();
                    messages.extend(json_msgs);
                }
            }
        }
        messages
    }
}

fn extract_from_sqlite(db_path: &std::path::Path) -> Vec<AgentMessage> {
    use rusqlite::Connection;

    let conn = match Connection::open(db_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let mut messages = Vec::new();

    let query = "SELECT m.data, p.data FROM message m JOIN part p ON p.message_id = m.id";
    if let Ok(mut stmt) = conn.prepare(query) {
        if let Ok(rows) = stmt.query_map([], |row| {
            let message_data: String = row.get(0)?;
            let part_data: String = row.get(1)?;
            Ok((message_data, part_data))
        }) {
            for (message_data, part_data) in rows.flatten() {
                if let Some(text) = extract_current_db_user_text(&message_data, &part_data) {
                    messages.push(AgentMessage {
                        text,
                        agent: "opencode".to_string(),
                    });
                }
            }
        }
    }

    let query = "SELECT text FROM messages WHERE role = 'user' OR kind = 'user'";
    if let Ok(mut stmt) = conn.prepare(query) {
        if let Ok(rows) = stmt.query_map([], |row| {
            let text: String = row.get(0)?;
            Ok(text)
        }) {
            for text in rows.flatten() {
                if !text.is_empty() {
                    messages.push(AgentMessage {
                        text,
                        agent: "opencode".to_string(),
                    });
                }
            }
        }
    }

    let query2 = "SELECT content FROM parts WHERE role = 'user' OR kind = 'user'";
    if let Ok(mut stmt) = conn.prepare(query2) {
        if let Ok(rows) = stmt.query_map([], |row| {
            let text: String = row.get(0)?;
            Ok(text)
        }) {
            for text in rows.flatten() {
                if !text.is_empty() {
                    messages.push(AgentMessage {
                        text,
                        agent: "opencode".to_string(),
                    });
                }
            }
        }
    }

    messages
}

fn extract_current_db_user_text(message_data: &str, part_data: &str) -> Option<String> {
    let message = serde_json::from_str::<serde_json::Value>(message_data).ok()?;
    if message.get("role").and_then(|r| r.as_str()) != Some("user") {
        return None;
    }

    let part = serde_json::from_str::<serde_json::Value>(part_data).ok()?;
    if part.get("synthetic").and_then(|s| s.as_bool()) == Some(true) {
        return None;
    }
    if part.get("type").and_then(|t| t.as_str()) != Some("text") {
        return None;
    }

    let text = part.get("text").and_then(|t| t.as_str())?.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

fn extract_json_messages(val: &serde_json::Value) -> Vec<AgentMessage> {
    let mut messages = Vec::new();
    if let Some(arr) = val.as_array() {
        for item in arr {
            if let Some(role) = item.get("role").and_then(|r| r.as_str()) {
                if role == "user" {
                    if let Some(content) = item.get("content") {
                        let text = match content {
                            serde_json::Value::String(s) => s.clone(),
                            serde_json::Value::Array(arr) => {
                                let mut parts = Vec::new();
                                for part in arr {
                                    if let Some(s) = part.as_str() {
                                        parts.push(s.to_string());
                                    } else if let Some(obj) = part.as_object() {
                                        if let Some(text) = obj.get("text").and_then(|t| t.as_str())
                                        {
                                            parts.push(text.to_string());
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
                                agent: "opencode".to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
    messages
}
