use super::timer::Timer;

use uuid::Uuid;
use serde::{Serialize, Deserialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub curr_timer: Option<Uuid>,
    pub tasks: Vec<String>,
    pub timers: Vec<Timer>,
}

#[derive(Tabled)]
pub struct ProjectsTable {
    pub name: String,
    pub curr_timer: String,
    pub tasks_len: usize,
    pub timers_len: usize,
}

impl Project {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            curr_timer: None,
            name: name.into(),
            tasks: vec![],
            timers: vec![],
        }
    }

    pub fn track(&mut self, task: &str) {
        self.add_timer(task);
    }

    pub fn add_timer(&mut self, task: &str) {
        self.timers.push(Timer::new(task));
    }

    pub fn add_task(&mut self, task_name: &str) {
        self.tasks.push(task_name.into())
    }
}
