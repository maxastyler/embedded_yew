#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt as _;
use defmt_rtt as _;
use panic_probe as _;

embassy_rp::bind_interrupts!(struct Irqs {
});

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {}
