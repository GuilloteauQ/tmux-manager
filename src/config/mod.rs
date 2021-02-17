pub mod pane;
pub mod session;
pub mod traits;
pub mod window;

use crate::config::session::Session;
use std::collections::HashMap;

pub type Config = HashMap<String, Session>;
