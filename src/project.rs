use super::timer::Timer;

use uuid::Uuid;

#[derive(Debug)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub timers: Vec<Timer>,
}

impl Project {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            timers: vec![],
        }
    }

    pub fn add_timer(&mut self, name: &str) {
        self.timers.push(Timer::new(name));
    }
}
