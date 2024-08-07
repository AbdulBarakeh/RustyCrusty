use std::time::{Instant, Duration};

pub struct Timer {
    start_time: Option<Instant>,
    stop_time: Option<Instant>,
}

impl Timer{

    pub fn new() -> Self {
        Timer {
            start_time: None,
            stop_time: None,
        }
    }

    pub fn start_timer(&mut self){
        self.start_time = Some(Instant::now());
    }
    
    
    pub fn stop_timer(&mut self) -> Result<Duration,&'static str>{
        self.stop_time = Some(Instant::now());
        match (self.start_time, self.stop_time) {
            (Some(start), Some(stop)) => Ok(stop.duration_since(start)),
            _ => Err("Timer was not started correctly"),
        }
    }
}
    
    