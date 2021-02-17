use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Pane {
    #[serde(default)]
    command: Option<String>,
    // Position
    #[serde(default)]
    left: Option<Box<Pane>>,
    #[serde(default)]
    below: Option<Box<Pane>>,
    #[serde(default)]
    right: Option<Box<Pane>>,
    #[serde(default)]
    above: Option<Box<Pane>>,
}

fn create_tmux_panes_helper(
    pane: &Option<Box<Pane>>,
    session_name: &String,
    root: &String,
    current_id: &mut usize,
    direction: Direction,
) {
    if let Some(split) = pane {
        split.tmux_split(session_name, root, current_id, direction);
        *current_id = *current_id + 1;
        split.create_tmux_panes(session_name, root, current_id);
    }
}
impl Pane {
    pub fn get_command(&self) -> Option<String> {
        self.command.clone()
    }

    pub fn create_tmux_panes(&self, session_name: &String, root: &String, current_id: &mut usize) {
        create_tmux_panes_helper(&self.left, session_name, root, current_id, Direction::Left);
        create_tmux_panes_helper(
            &self.right,
            session_name,
            root,
            current_id,
            Direction::Right,
        );
        create_tmux_panes_helper(
            &self.above,
            session_name,
            root,
            current_id,
            Direction::Above,
        );
        create_tmux_panes_helper(
            &self.below,
            session_name,
            root,
            current_id,
            Direction::Below,
        );
    }

    fn tmux_split(
        &self,
        session_name: &String,
        root: &String,
        target_pane: &usize,
        direction: Direction,
    ) {
        if let Some(cmd) = &self.command {
            Command::new("tmux")
                .arg("split-window")
                .arg("-c")
                .arg(root)
                .arg(direction.get_tmux_flag())
                .arg("-t")
                .arg(format!("{}.{}", session_name, target_pane))
                .arg(cmd)
                .output()
                .expect("Failed to execute command");
        } else {
            Command::new("tmux")
                .arg("split-window")
                .arg("-c")
                .arg(root)
                .arg(direction.get_tmux_flag())
                .arg("-t")
                .arg(format!("{}.{}", session_name, target_pane))
                .output()
                .expect("Failed to execute command");
        }
    }
}

enum Direction {
    Left,
    Right,
    Above,
    Below,
}

impl Direction {
    fn get_tmux_flag(&self) -> String {
        match self {
            Direction::Left => String::from("-hb"),
            Direction::Right => String::from("-h"),
            Direction::Above => String::from("-vb"),
            Direction::Below => String::from("-v"),
        }
    }
}
