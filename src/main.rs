use rs_ws281x::{ControllerBuilder, ChannelBuilder, StripType};
use rand::prelude::*;
use std::thread;

struct LEDSegment {
	length: u32,
	offset: u32
}

fn main() {
	let mut controller = ControllerBuilder::new()
		.freq(800_000)
		.dma(10)
		.channel(
		0,
		ChannelBuilder::new()
			.pin(18)
			.count(600)
			.strip_type(StripType::Ws2812)
			.brightness(255)
			.build()
		)
		.build().unwrap();

	loop {
		let leds = controller.leds_mut(0);
		for i in 0..600 {
			leds[i] = [rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 0];
		}

		controller.render();
		thread::sleep_ms(1000);
	}
}
