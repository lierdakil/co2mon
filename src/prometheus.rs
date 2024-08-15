use std::sync::LazyLock;

use prometheus::{
    self,
    core::{AtomicF64, GenericGauge},
    register_gauge, register_int_gauge, Encoder, IntGauge, TextEncoder,
};

pub static TEMP: LazyLock<GenericGauge<AtomicF64>> =
    LazyLock::new(|| register_gauge!("temp", "Temperature, C").unwrap());
pub static CO2: LazyLock<IntGauge> =
    LazyLock::new(|| register_int_gauge!("co2", "CO2, PPM").unwrap());

pub fn publish() -> Vec<u8> {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    buffer
}
