use uuid::Uuid;
use chrono::{
    prelude::*,
    Duration,
};
use serde::{Serialize, Deserialize};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Timer hasn't started yet")]
    NotStartedYet,

    #[error("Timer has already started")]
    AlreadyStarted,

    #[error("Timer has already ended")]
    AlreadyEnded,

    #[error("Timer is not valid")]
    BrokenTimer,
}

pub enum Status {
    Created,
    Running,
    Stopped,
    Invalid,
}

#[derive(Serialize, Deserialize)]
pub struct TimerOptions {
    pub notes: String,
}

#[derive(Serialize, Deserialize)]
pub struct Timer {
    pub id: Uuid,
    pub task: String,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

impl Timer {
    pub fn new(task: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            task: task.into(),
            start_time: None,
            end_time: None,
            notes: None,
        }
    }

    pub fn start(&mut self) -> Result<(), Error> {
        match self.status() {
            Status::Created => {
                self.start_time = Some(Utc::now());
                Ok(())
            },
            Status::Running => Err(Error::AlreadyStarted),
            Status::Stopped => Err(Error::AlreadyEnded),
            Status::Invalid => Err(Error::BrokenTimer),
        }
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        match self.status() {
            Status::Created => Err(Error::NotStartedYet),
            Status::Running => {
                self.end_time = Some(Utc::now());
                Ok(())
            },
            Status::Stopped => Err(Error::AlreadyEnded),
            Status::Invalid => Err(Error::BrokenTimer),
        }
    }

    pub fn status(&self) -> Status {
        match (&self.start_time, &self.end_time) {
            (None, None) => Status::Created,
            (Some(_), None) => Status::Running,
            (None, Some(_)) => Status::Invalid,
            (Some(_), Some(_)) => Status::Stopped,
        }
    }

    pub fn elapsed(&self) -> Result<Duration, Error> {
        match self.status() {
            Status::Created => Err(Error::NotStartedYet),
            Status::Invalid => Err(Error::BrokenTimer),
            Status::Stopped => Ok(self.end_time.unwrap().signed_duration_since(self.start_time.unwrap())),
            Status::Running => Ok(Utc::now().signed_duration_since(self.start_time.unwrap()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TRESHOLD: i64 = 10;

    #[test]
    fn start() {
        let mut timer = Timer::new("dev");
        assert!(timer.start().is_ok());
    }

    #[test]
    fn stop() {
        let mut timer = Timer::new("dev");
        assert!(matches!(timer.stop(), Err(Error::NotStartedYet)));
        assert!(timer.start().is_ok());
        assert!(timer.stop().is_ok());
    }

    #[test]
    fn elapsed() {
        let mut timer = Timer::new("dev");
        assert!(timer.elapsed().is_err());
        timer.start().unwrap();
        std::thread::sleep(Duration::seconds(5).to_std().unwrap());
        let elapsed = timer.elapsed().unwrap();
        assert!(elapsed > Duration::seconds(5) - Duration::milliseconds(TRESHOLD));
        assert!(elapsed < Duration::seconds(5) + Duration::milliseconds(TRESHOLD));
    }
}
