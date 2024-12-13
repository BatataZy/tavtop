use crate::unit_types::Size;


struct Cpu_Result {
    pub clock: Vec<Clock>,
    pub util: Vec<Util>,
    pub temp: f32,
}
struct Clock {
    median: u32,
    arithmetic_mean: u32,
    max: u32,
    min:u32,
    values: Vec<u32>
}
struct Util {
    median: u8,
    arithmetic_mean: u8,
    max: u8,
    min: u8,
    values: Vec<u8>
}


struct Gpu_Result {
    pub clock: u32,
    pub util: u8,
    pub temp: f32,
    pub vram: Size,
}