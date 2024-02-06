#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod net_stack;
mod server;

use defmt as _;
use defmt_rtt as _;
use net_stack::set_up_network_stack;
use panic_probe as _;
use server::start_server;

embassy_rp::bind_interrupts!(
    struct Irqs {
        PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<embassy_rp::peripherals::PIO0>;
	USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<embassy_rp::peripherals::USB>;
    }
);

const INDEX_HTML: &str = include_str!(env!("YEW_PART_HTML"));
const YEW_JS: &str = include_str!(env!("YEW_PART_JS"));
const YEW_WASM: &[u8] = include_bytes!(env!("YEW_PART_WASM"));

#[embassy_executor::task]
async fn logger_task(usb: embassy_rp::peripherals::USB) {
    let driver = embassy_rp::usb::Driver::new(usb, Irqs);
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = embassy_rp::init(Default::default());

    spawner.must_spawn(logger_task(p.USB));

    let (_, stack) = set_up_network_stack(
        &spawner, p.PIN_23, p.PIN_25, p.PIO0, p.PIN_24, p.PIN_29, p.DMA_CH0,
    )
    .await;

    start_server(&spawner, stack).await;
}
