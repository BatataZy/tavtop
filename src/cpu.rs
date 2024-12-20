
use crate::{read, unit_types::{Delta, Magnitude}};

#[derive(Clone, Debug)]
pub struct Cpu {
    pub threads:usize,
    pub clock: Vec<Magnitude>,
    pub util: Vec<u8>,
    pub total_time: Vec<Delta>,
    pub idle_time: Vec<Delta>,
    pub temp: f32,
}

impl Cpu {
    pub fn new(threads: usize) -> Self {
        
        Cpu{
            threads,
            clock: vec![Magnitude::new(); threads],
            util: vec![0;threads],
            total_time: vec![Delta::new(); threads],
            idle_time: vec![Delta::new(); threads],
            temp: 0.,
    }}


    pub fn update(&mut self) {

        //CPU CLOCK LOGIC
        self.clock.iter_mut().zip(0..).for_each(|(clock, i)| {
            
            let current_clock = (read(&("/sys/devices/system/cpu/cpu".to_owned() + &i.to_string() + "/cpufreq/scaling_cur_freq"), 0, 0)
                .parse::<f32>().unwrap()/1000.).round() as u16;

            clock.add(current_clock);
        });
        
        //CPU UTIL LOGIC
        self.total_time.iter_mut().zip(self.idle_time.iter_mut()).zip(self.util.iter_mut()).zip(

            read("/proc/stat", 0, 0)
                .split('\n').zip(0..).filter(|(_, i)| *i <= self.threads && *i >= 1).map(|x|
                    x.0.to_owned()
                ).map(|x|
                    x.split(" ").filter_map(|x| x.parse::<u32>().ok()).collect::<Vec<u32>>()
                )).map(|(((total, idle), util),cur)| (total, idle, util, cur))
                    .for_each(|(total, idle, util, cur)| {

                        total.add(cur.iter().sum::<u32>());

                        idle.add(cur[3] + cur[4]);

                        *util = 100 - (idle.delta as f32/total.delta as f32 * 100.).round() as u8; 
                    }
        );

        self.temp = 0.;
    }
}