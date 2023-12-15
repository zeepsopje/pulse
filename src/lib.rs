pub mod cli;
pub mod timer;
pub mod project;

pub struct Pulse {
    pub projects: Vec<project::Project>,
}
