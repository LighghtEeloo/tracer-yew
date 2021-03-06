use std::{fmt::Debug, time::{SystemTime, Duration}};
use chrono::{DateTime, Local, Utc};
use serde::{Serialize, Deserialize};
pub trait TimeRep {
    fn human_local_detail(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn human_local(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn human_utc(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl TimeRep for SystemTime {
    fn human_local_detail(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt: DateTime<Local> = self.clone().into();
        write!(f, "{} (Local)", dt.format("%Y-%m-%d %a %H:%M:%S").to_string())
    }
    fn human_local(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt: DateTime<Local> = self.clone().into();
        write!(f, "{} (Local)", dt.format("%Y-%m-%d %H:%M:%S").to_string())
    }
    fn human_utc(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt: DateTime<Utc> = self.clone().into();
        write!(f, "{} (UTC)", dt.format("%Y-%m-%d %H:%M:%S").to_string())
    }
}

#[derive(Default)]
#[derive(Clone, Serialize, Deserialize)]
pub struct TimeLog {
    start: Option<SystemTime>,
    end: Option<SystemTime>
}

impl TimeLog {
    pub fn new() -> TimeLog {
        TimeLog::default()
    }
    pub fn set_start(&mut self, start: SystemTime) -> &mut Self {
        self.start = Some(start);
        self
    }
    pub fn set_end(&mut self, end: SystemTime) -> &mut Self {
        self.end = Some(end);
        self
    }
    pub fn length(&self) -> Duration {
        match (self.start, self.end) {
            (Some(s), Some(e)) => e.duration_since(s).unwrap_or(Duration::new(0, 0)),
            _ => Duration::new(0, 0)
        }
    }
}

impl TimeRep for TimeLog {
    fn human_local_detail(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[:")?;
        self.start.map(|t: SystemTime| t.human_local_detail(f));
        write!(f, " ~ ")?;
        self.end.map(|t: SystemTime| t.human_local_detail(f));
        write!(f, ":]")?;
        write!(f, "")
    }

    fn human_local(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[:")?;
        self.start.map(|t: SystemTime| t.human_local(f));
        write!(f, " ~ ")?;
        self.end.map(|t: SystemTime| t.human_local(f));
        write!(f, ":]")?;
        write!(f, "")
    }

    fn human_utc(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[:")?;
        self.start.map(|t: SystemTime| t.human_utc(f));
        write!(f, " ~ ")?;
        self.end.map(|t: SystemTime| t.human_utc(f));
        write!(f, ":]")?;
        write!(f, "")
    }
}

impl Debug for TimeLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.human_local(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    #[test]
    fn time_log() {
        let u_dur = 200;
        let mut time = TimeLog::new();
        time.set_start(SystemTime::now());
        sleep(Duration::from_millis(u_dur));
        time.set_end(SystemTime::now());
        let dur = time.length();
        println!("{:?}", dur);
        assert!(dur > Duration::from_millis(u_dur));
        println!("{:?}", time)
    }
}
