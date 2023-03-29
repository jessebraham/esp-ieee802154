#![no_std]
#![no_main]

use esp32c6_hal::{
    clock::{ClockControl, CpuClock},
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    Delay, Rtc,
};
use esp_backtrace as _;
use esp_ieee802154::*;
use esp_println::println;
use ieee802154::mac::{Header, PanId, ShortAddress};

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger(log::LevelFilter::Info);

    let peripherals = Peripherals::take();
    let mut system = peripherals.PCR.split();
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock160MHz).freeze();

    let mut rtc = Rtc::new(peripherals.LP_CLKRST);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    // Disable watchdog timers
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let mut delay = Delay::new(&clocks);

    println!("Start");
    let mut ieee802154 = Ieee802154::new(&mut system.radio_clock_control);

    ieee802154.set_config(Config {
        channel: 11,
        promiscuous: false,
        pan_id: Some(0x4242),
        short_addr: Some(0x2323),
        ..Config::default()
    });

    let mut seq_number = 0u8;
    loop {
        ieee802154
            .transmit(&Frame {
                header: Header {
                    frame_type: ieee802154::mac::FrameType::Data,
                    frame_pending: false,
                    ack_request: false,
                    pan_id_compress: false,
                    seq_no_suppress: false,
                    ie_present: false,
                    version: ieee802154::mac::FrameVersion::Ieee802154_2003,
                    seq: seq_number,
                    destination: Some(ieee802154::mac::Address::Short(
                        PanId(0xffff),
                        ShortAddress(0xffff),
                    )),
                    source: None,
                    auxiliary_security_header: None,
                },
                content: ieee802154::mac::FrameContent::Data,
                payload: heapless::Vec::from_slice(b"Hello World").unwrap(),
                footer: [0u8; 2],
            })
            .ok();

        println!("Send frame with sequence number {seq_number}");
        delay.delay_ms(1000u32);
        seq_number = seq_number.wrapping_add(1);
    }
}
