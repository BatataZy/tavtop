use std::{fs, io::Read, thread, time};
use home::home_dir;
use serde_json;
use num_traits::{Num, NumCast};


mod profiler; use profiler::Profiler;
mod result; use result::Result;
mod cpu; use cpu::Cpu;
mod gpu; use gpu::Gpu;
mod ram; use ram::Memory;
mod unit_types;


static BUFFER: usize = 4000;
static STEP: i64 = (BUFFER as f32/ITER as f32 *1000.) as i64;
static ITER: usize = 40;


fn main() {

    let mut buf: String = String::with_capacity(4096);
    
    let profiling = false;

    let write_path = home_dir().unwrap().join(".data");

    let mut start:time::Instant;
    let mut end:time::Instant;
    
    let mut cpu = Cpu::new(&mut buf);
    let mut gpu = Gpu::new(&mut buf);
    let mut mem = Memory::new(&mut buf);

    let mut profiler = Profiler::new();

    loop {
        start = time::Instant::now();

        cpu.update(&mut buf);

        gpu.update(&mut buf);

        mem.update(&mut buf);

        let _ = fs::write(&write_path.join("result").to_str().unwrap().to_owned(),serde_json::to_string_pretty(&Result::new(&cpu, &gpu, &mem)).unwrap());

        let _ = fs::write(&write_path.join("result_pretty").to_str().unwrap().to_owned(),serde_json::to_string_pretty(&Result::new(&cpu, &gpu, &mem).prettify()).unwrap());

        end = time::Instant::now();

        if profiling {profiler.update(start, end)}

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


pub fn read(path: &str, buf: &mut String, start: usize, end: usize) -> String {

    let mut file = fs::File::open(path).unwrap();

    buf.clear();

    let read = file.read_to_string(buf);

    return match read {
        Ok(_) => {
            buf[start..buf.len()-1-end].to_owned()
        },
        Err(_) => ("0").to_owned(),
    };
}
