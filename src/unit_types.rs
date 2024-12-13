use crate::ITER;
use serde;

#[derive(Clone, Debug, serde::Serialize)]
pub struct Magnitude {
    pub index: usize,
    pub values: Vec<u32>,
    pub average: u32,
}

impl Magnitude {
    pub fn new() -> Self {
        Magnitude{
            index: 0,
            values: vec![0; ITER],
            average: 0,
    }}
}


#[derive(Clone, Debug)]
pub struct Delta {
    pub index: usize,
    pub values: Vec<u32>,
    pub delta: u32
}

impl Delta {
    pub fn new() -> Self {
        Delta{
            index: 0,
            values: vec![0; ITER],
            delta: 0,
        }
    }
}


#[derive(Clone, Debug)]
pub struct Percent {
    pub index: usize,
    pub values: Vec<u8>,
    pub average: u8,
}

impl Percent {
    pub fn new() -> Self {
        Percent { 
            index: 0 ,
            values: vec![0; ITER],
            average: 0, 
    }}
}


pub struct Size {
    used: usize,
    total: usize,
}

impl Size {
    pub fn new(total:usize) -> Self {
        Size { 
            total: total,
            used: 0,
    }}
}