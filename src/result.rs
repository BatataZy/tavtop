use crate::{cpu::Cpu, gpu::Gpu, median, unit_types::Size};
use serde;

static NULL: &str = " ";

fn prettify(x:&str, size: usize, blank: &str, prefix: &str) -> String {
    return blank.to_owned().repeat(size - x.len()) + x + prefix;
}


#[derive(serde::Serialize)]
struct Clock {
    median: String,
    arithmetic_mean: String,
    max: String,
    min: String,
    values: Vec<String>
}
impl Clock {
    fn prettify(self) -> Clock {
        Clock {
            median: prettify(&self.median, 4, NULL, "MHz"),
            arithmetic_mean: prettify(&self.arithmetic_mean, 4, NULL, "MHz"),
            max: prettify(&self.max, 4, NULL, "MHz"),
            min: prettify(&self.min, 4, NULL, "MHz"),
            values: self.values.iter().map(|x| prettify(x, 4, NULL, "MHz")).collect()
        }
    }
}


#[derive(serde::Serialize)]
struct Util {
    median: String,
    arithmetic_mean: String,
    max: String,
    min: String,
    values: Vec<String>
}
impl Util {
    fn prettify(self) -> Util {
        Util {
            median: prettify(&self.median, 2, "0", "%"),
            arithmetic_mean: prettify(&self.arithmetic_mean, 2, "0", "%"),
            max: prettify(&self.max, 2, "0", "%"),
            min: prettify(&self.min, 2, "0", "%"),
            values: self.values.iter().map(|x| prettify(x, 2, "0", "%")).collect()
        }
    }
}


#[derive(serde::Serialize)]
struct CpuResult {
    pub clock: Clock,
    pub util: Util,
    pub temp: String,
}
impl CpuResult {
    pub fn new(cpu: Cpu) -> CpuResult {

        let clocks = cpu.clock.iter().map(|x| x.average.round() as u16);
        let utils = cpu.util.iter();

        CpuResult {
            clock: Clock {

                median: median(clocks.clone().collect::<Vec<u16>>()).to_string(), 
                arithmetic_mean: (clocks.clone().sum::<u16>() / cpu.threads as u16).to_string(),
                max: clocks.clone().max().unwrap().to_string(),
                min: clocks.clone().min().unwrap().to_string(),
                values: clocks.clone().map(|x| x.to_string()).collect::<Vec<String>>(),
            },
            util: Util {
                median: median(cpu.util.clone()).min(99).to_string(),
                arithmetic_mean: (utils.clone().map(|x| *x as u16).sum::<u16>()/cpu.threads as u16).min(99).to_string(),
                max: utils.clone().max().unwrap().min(&99).to_string(),
                min: utils.clone().min().unwrap().min(&99).to_string(),
                values: cpu.util.iter().map(|x| x.min(&99).to_string()).collect::<Vec<String>>(),
            },
            temp: format!("{:.1}", cpu.temp),
        }
    }


    fn prettify(self) -> CpuResult {
        CpuResult {
            clock: self.clock.prettify(),
            util: self.util.prettify(),
            temp: prettify(&self.temp, 4, NULL, "°C")
        }
    }
}


#[derive(serde::Serialize)]
struct GpuResult {
    pub clock: String,
    pub util: String,
    pub temp: String,
    pub vram: Size,
}
impl GpuResult {
    pub fn new(gpu: Gpu) -> GpuResult {
        GpuResult {
            clock: gpu.clock.average.round().to_string(),
            util: gpu.util.average.round().abs().min(99.).to_string(),
            temp:  format!("{:.1}", gpu.temp),
            vram: gpu.memory,
        }
    }


    fn prettify(self) -> GpuResult {
        GpuResult {
            clock: prettify(&self.clock, 4, NULL, "MHz"),
            util: prettify(&self.util, 2, "0", "%"),
            temp: prettify(&self.temp, 4, "4", "°C"),
            vram: self.vram
        }
    }
}


#[derive(serde::Serialize)]
struct RamResult {
    
}
impl RamResult {
    fn prettify(self) -> RamResult {
        self
    }
}


#[derive(serde::Serialize)]
struct DiskResult {

}
impl DiskResult {
    fn prettify(self) -> DiskResult {
        self
    }
}


#[derive(serde::Serialize)]
struct NetworkResult {

}
impl NetworkResult {
    fn prettify(self) -> NetworkResult {
        self
    }
}


#[derive(serde::Serialize)]
pub struct Result {
    cpu: CpuResult,
    gpu: GpuResult,
    ram: RamResult,
    disk: DiskResult,
    network: NetworkResult
}
impl Result {
    pub fn new(cpu: Cpu, gpu: Gpu) -> Result {
        Result {
            cpu: CpuResult::new(cpu),
            gpu: GpuResult::new(gpu),
            ram: RamResult {  },
            disk: DiskResult {  },
            network: NetworkResult {  }
        }
    }
    
    pub fn prettify(self) -> Result {
        Result{
            cpu: self.cpu.prettify(),
            gpu: self.gpu.prettify(),
            ram: self.ram.prettify(),
            disk: self.disk.prettify(),
            network: self.network.prettify(),
        }
    }
}