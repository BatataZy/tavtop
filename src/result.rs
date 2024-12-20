use crate::{cpu::Cpu, median, unit_types::Size};
use serde;

#[derive(serde::Serialize)]
pub struct Result {
    cpu: CpuResult,
    gpu: GpuResult,
    ram: RamResult,
    disk: DiskResult,
    network: NetworkResult
}
impl Result {
    pub fn new(cpu: Cpu) -> Result {
        Result {
            cpu: CpuResult::new(cpu),
            gpu: GpuResult { clock: 0, util: 0, temp: 0.0, vram: Size::new(0) },
            ram: RamResult {  },
            disk: DiskResult {  },
            network: NetworkResult {  }
        }
    }
}


#[derive(serde::Serialize)]
struct CpuResult {
    pub clock: Clock,
    pub util: Util,
    pub temp: f32,
}
#[derive(serde::Serialize)]
struct Clock {
    median: u16,
    arithmetic_mean: u16,
    max: u16,
    min:u16,
    values: Vec<u16>
}
#[derive(serde::Serialize)]
struct Util {
    median: u8,
    arithmetic_mean: u8,
    max: u8,
    min: u8,
    values: Vec<u8>
}
impl CpuResult {
    pub fn new(cpu: Cpu) -> CpuResult {

        let clocks = cpu.clock.iter().map(|x| x.average.round() as u16);
        let utils = cpu.util.iter();

        CpuResult {
            clock: Clock {

                median: median(clocks.clone().collect::<Vec<u16>>()), 
                arithmetic_mean: clocks.clone().sum::<u16>() / cpu.threads as u16,
                max: clocks.clone().max().unwrap(),
                min: clocks.clone().min().unwrap(),
                values: clocks.clone().collect::<Vec<u16>>(),
            },
            util: Util {
                median: median(cpu.util.clone()),
                arithmetic_mean: utils.clone().sum::<u8>()/cpu.threads as u8,
                max: *utils.clone().max().unwrap(),
                min: *utils.clone().min().unwrap(),
                values: cpu.util,
            },
            temp: cpu.temp,
        }

    }
}


#[derive(serde::Serialize)]
struct GpuResult {
    pub clock: u32,
    pub util: u8,
    pub temp: f32,
    pub vram: Size,
}


#[derive(serde::Serialize)]
struct RamResult {

}


#[derive(serde::Serialize)]
struct DiskResult {

}


#[derive(serde::Serialize)]
struct NetworkResult {

}