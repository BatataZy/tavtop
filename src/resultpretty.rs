
#[derive(serde::Serialize)]
pub struct ResultPretty{
    cpu: CpuResultPretty,
    gpu: GpuResultPretty,
    ram: RamResultPretty,
    disk: DiskResultPretty,
    network: NetworkResultPretty
}
impl ResultPretty{
    pub fn new()
}


#[derive(serde::Serialize)]
struct CpuResultPretty {
    pub clock: ClockPretty,
    pub util: UtilPretty,
    pub temp: String,
}
#[derive(serde::Serialize)]
struct ClockPretty {
    median: String,
    arithmetic_mean: String,
    max: String,
    min: String,
    values: Vec<String>
}
#[derive(serde::Serialize)]
struct UtilPretty {
    median: String,
    arithmetic_mean: String,
    max: String,
    min: String,
    values: Vec<String>
}



#[derive(serde::Serialize)]
struct GpuResultPretty {
    pub clock: String,
    pub util: String,
    pub temp: String,
    pub vram: SizePretty,
}
#[derive(serde::Serialize)]
struct SizePretty {
    pub used: String,
    pub total: String,
}



#[derive(serde::Serialize)]
struct RamResultPretty {

}


#[derive(serde::Serialize)]
struct DiskResultPretty {

}



#[derive(serde::Serialize)]
struct NetworkResultPretty {

}