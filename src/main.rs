use std::{fs::{self}, thread::{self}, time};
use home::home_dir;
use serde_json;
use num_traits::{Num, NumCast};

mod profiler; use profiler::Profiler;
mod result; use result::Result;
mod cpu; use cpu::Cpu;
mod gpu; use gpu::Gpu;
use unit_types::Magnitude;
mod unit_types;

static BUFFER: usize = 4000;
static STEP: i64 = (BUFFER as f32/ITER as f32 *1000.) as i64;
static ITER: usize = 40;


fn main() {
    let write_path = home_dir().unwrap().join(".data").join("result").to_str().unwrap().to_owned();

    let mut start:time::Instant;
    let mut end:time::Instant;
    
    let mut cpu = Cpu::new(read("/sys/devices/system/cpu/online", 0, 0).split('-').collect::<Vec<&str>>()[1].parse::<usize>().unwrap() + 1);
    let mut gpu = Gpu::new();

    let mut profiler = Profiler::new();

    //let file = OpenOptions::new().read(true).write(false).open("/sys/devices/system/cpu/cpu".to_owned() + &i.to_string() + "/cpufreq/scaling_cur_freq").unwrap();

    loop {
        start = time::Instant::now();

        cpu.update();

        gpu.update();

        let _ = fs::write(&write_path,serde_json::to_string_pretty(&Result::new(cpu.clone())).unwrap());

        end = time::Instant::now();

        //profiler.update(start, end);

        thread::sleep(time::Duration::from_micros((STEP - (end-start).as_micros() as i64).max(0) as u64) );
    }
}


pub fn median<T:Num + NumCast + Copy + Ord>(mut list: Vec<T>) ->  T {

    list.sort();
    
    return match list.len() % 2 {
        0 => (list[list.len()/2] + list[list.len()/2 - 1]) / T::from(2).unwrap(),
        _ => list[list.len()/2]
    };
}


pub fn read(path: &str, start: usize, end: usize) -> String {

    let read = fs::read_to_string(path);

    return match read {
        Ok(x) => {
            x.as_str()[start..x.len()-1-end].to_owned()
        },
        Err(..) => ("0").to_owned(),
    };
}
