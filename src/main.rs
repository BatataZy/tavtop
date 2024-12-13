use std::{fs, thread::{self}, time};
use home::home_dir;
//use serde_json;

mod result;
mod cpu; use cpu::Cpu;
mod unit_types;

static BUFFER: usize = 4000;
static STEP: i64 = (BUFFER as f32/ITER as f32 *1000.) as i64;
static ITER: usize = 50;

fn main() {
    let write_path = home_dir().unwrap().join(".data");

    println!("{:?}", write_path);

    let mut start:time::Instant;
    let mut end:time::Instant;
    let mut cpu = Cpu::new(read("/sys/devices/system/cpu/online", 0, 0).split('-').collect::<Vec<&str>>()[1].parse::<usize>().unwrap() + 1);


    loop {
        start = time::Instant::now();

        cpu.update();

        let _ = fs::write(write_path.join("cpu_clck_avg"), (cpu.clock.iter().map(|x| x.average).sum::<u32>()/cpu.threads as u32).to_string() );

        let _ = fs::write(write_path.join("cpu_util_avg"), (cpu.util.iter().sum::<u8>()/cpu.threads as u8).to_string() );

        end = time::Instant::now();
        
        //println!("{:?}", ( (end-start).as_micros(), (STEP - (end-start).as_micros() as i64).max(0) ));

        thread::sleep(time::Duration::from_micros((STEP - (end-start).as_micros() as i64).max(0) as u64) );
    }
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
