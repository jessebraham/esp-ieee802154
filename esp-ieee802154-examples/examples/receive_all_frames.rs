#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{peripherals::Peripherals, prelude::*};
use esp_ieee802154::*;
use esp_println::println;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take();
    let mut ieee802154 = Ieee802154::new(peripherals.IEEE802154, &mut peripherals.RADIO_CLK);

    ieee802154.set_config(Config {
        channel: 15,
        promiscuous: true,
        rx_when_idle: true,
        auto_ack_rx: false,
        auto_ack_tx: false,
        ..Config::default()
    });

    println!("Start receiving:");
    ieee802154.start_receive();

    loop {
        if let Some(frame) = ieee802154.get_received() {
            println!("Received: {:?}\n", &frame);
        }
    }
}
