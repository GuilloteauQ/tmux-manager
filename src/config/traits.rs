pub trait ToTmux {
    fn to_tmux(&self, session_name: &String);
}
