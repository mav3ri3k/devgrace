pub mod amp;
pub mod claude;
pub mod cline;
pub mod codex;
pub mod opencode;
pub mod pi;
pub mod zed;

use std::path::PathBuf;

pub struct AgentMessage {
    pub text: String,
    #[allow(dead_code)]
    pub agent: String,
}

pub trait AgentAdapter: Send + Sync {
    fn name(&self) -> &str;
    fn data_dirs(&self) -> Vec<PathBuf>;
    fn extract_messages(&self) -> Vec<AgentMessage>;
}
