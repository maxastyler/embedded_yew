#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt as _;
use defmt_rtt as _;
use panic_probe as _;

embassy_rp::bind_interrupts!(
    struct Irqs {}
);

const INDEX_HTML: &[u8] = include_bytes!(env!("YEW_PART_HTML"));
const YEW_JS: &[u8] = include_bytes!(env!("YEW_PART_JS"));
const YEW_WASM: &[u8] = include_bytes!(env!("YEW_PART_WASM"));

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {}
