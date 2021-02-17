use crate::config::pane::Pane;
use crate::config::traits::ToTmux;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Window {
    name: String,
    root: String,
    pub panes: Pane,
}

impl Window {
    pub fn get_name(&self) -> String {
        // berk
        self.name.clone()
    }
    pub fn get_root(&self) -> String {
        // berk
        self.root.clone()
    }
}

impl ToTmux for Window {
    fn to_tmux(&self, session_name: &String) {
        if let Some(cmd) = self.panes.get_command() {
            Command::new("tmux")
                .arg("new-window")
                .arg("-c")
                .arg(&self.root)
                .arg("-t")
                .arg(session_name)
                .arg("-n")
                .arg(&self.name)
                .arg(cmd)
                .output()
                .expect("Failed to execute command");
        } else {
            Command::new("tmux")
                .arg("new-window")
                .arg("-c")
                .arg(&self.root)
                .arg("-t")
                .arg(session_name)
                .arg("-n")
                .arg(&self.name)
                .output()
                .expect("Failed to execute command");
        }
        let mut current_id = 0;
        self.panes
            .create_tmux_panes(session_name, &self.root, &mut current_id);
    }
}
