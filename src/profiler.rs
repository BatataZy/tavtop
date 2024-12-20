use std::time;

use crate::unit_types::Magnitude;

pub struct Profiler {
        times: Magnitude,
        max: u16
    }

impl Profiler {
    pub fn new() -> Profiler {
        Profiler{
            times: Magnitude::new(),
            max: 0
        }
    }

    pub fn update(&mut self, start: time::Instant, end: time::Instant) {

        let delta_time = (end-start).as_micros() as u16;

        self.times.add(delta_time);

        if self.max < self.times.average as u16 {self.max = self.times.average as u16}

        println!("{:?}", (self.times.average, self.max))
        
    }
}