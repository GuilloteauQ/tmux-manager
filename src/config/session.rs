use crate::config::traits::ToTmux;
use crate::config::window::Window;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Session {
    windows: Vec<Window>,
}

impl ToTmux for Session {
    fn to_tmux(&self, session_name: &String) {
        // TODO: cleaner
        assert!(self.windows.len() > 0);
        let _ = Command::new("tmux")
            .arg("kill-session")
            .arg("-t")
            .arg(session_name)
            .output()
            .ok();
        let first_window_name = self.windows[0].get_name();
        let first_command = self.windows[0].panes.get_command();
        let root = self.windows[0].get_root();
        if let Some(cmd) = first_command {
            Command::new("tmux")
                .arg("new-session")
                .arg("-c")
                .arg(&root)
                .arg("-d")
                .arg("-s")
                .arg(session_name)
                .arg("-n")
                .arg(&first_window_name)
                .arg(cmd)
                .output()
                .expect("Failed to execute command");
        } else {
            Command::new("tmux")
                .arg("new-session")
                .arg("-c")
                .arg(&root)
                .arg("-d")
                .arg("-s")
                .arg(session_name)
                .arg("-n")
                .arg(&first_window_name)
                .output()
                .expect("Failed to execute command");
        }

        let mut current_id = 0;
        &self.windows[0].panes.create_tmux_panes(
            &format!("{}:{}", session_name, first_window_name),
            &root,
            &mut current_id,
        );

        self.windows
            .iter()
            .skip(1)
            .for_each(|window| window.to_tmux(session_name));
    }
}
