pub mod cli;
pub mod timer;
pub mod project;

use project::Project;
use serde::{Serialize, Deserialize};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Project already exists")]
    ProjectAlreadyExists,
}

#[derive(Serialize, Deserialize)]
pub struct Pulse {
    pub projects: Vec<Project>,
}

impl Pulse {
    pub fn default() -> Self {
        Self { projects: vec![] }
    }

    pub fn find_project(&self, name: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.name == name)
    }

    pub fn find_project_mut(&mut self, name: &str) -> Option<&mut Project> {
        self.projects.iter_mut().find(|p| p.name == name)
    }

    pub fn add_project(&mut self, name: &str) -> Result<(), Error> {
        match self.find_project(name) {
            Some(_) => Err(Error::ProjectAlreadyExists),
            None => {
                self.projects.push(Project::new(name.into()));
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_project() {
        let mut pulse = Pulse::default();
        // Make sure duplicates aren't possible
        for _ in 0..2 {
            let _ = pulse.add_project("test-project");
            assert_eq!(pulse.projects.len(), 1);
        }
    }
}
