
use std::usize;

use crate::{read, unit_types::{Magnitude, Percent, Size}};

#[derive(Clone, Debug)]
pub struct Gpu{
    pub card: u8,
    pub clock: Magnitude,
    pub util: Percent,
    pub memory: Size,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            card: 1,
            clock: Magnitude::new(),
            util: Percent::new(),
            memory: Size::new(read("/sys/class/drm/card1/device/mem_info_vram_total", 0, 0).parse::<usize>().unwrap()/1048576),
        }
    }


    pub fn update(&mut self) {

        //GPU CLOCK LOGIC

        let current_clock = match read(&("/sys/class/drm/card".to_owned() + &self.card.to_string() + "/device/pp_dpm_sclk"), 14, 17).as_str() {
            " 70" => 700,
            x => x.parse::<u16>().unwrap(),
        };
        self.clock.add(current_clock);

        //GPU UTIL LOGIC
        let current_util = read(&("/sys/class/drm/card".to_owned() + &self.card.to_string() + "/device/gpu_busy_percent"), 0,0).parse::<u8>().unwrap();

        self.util.add(current_util);

        //GPU MEMORY LOGIC

        self.memory.used = read(&("/sys/class/drm/card".to_owned() + &self.card.to_string() + "/device/mem_info_vram_total"), 0, 0).parse::<usize>().unwrap()/1048576;
    }
}