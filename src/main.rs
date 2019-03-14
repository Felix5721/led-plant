use rs_ws281x::{ControllerBuilder, ChannelBuilder, StripType};

fn main() {
	let mut controller = ControllerBuilder::new()
		.freq(800_000)
		.dma(10)
		.channel(
		0,
		ChannelBuilder::new()
			.pin(18)
			.count(10)
			.strip_type(StripType::Ws2812)
			.brightness(255)
			.build()
		)
		.build().unwrap();

	let leds = controller.leds_mut(0);
	leds[0] = [255, 0, 0, 0];
	leds[1] = [0, 255, 0, 0];
	leds[2] = [0, 0, 255, 0];
	leds[3] = [255, 255, 0, 0];
	leds[4] = [255, 0, 255, 0];
	leds[5] = [0, 255, 255, 0];
	leds[6] = [255, 255, 255, 0];
	leds[7] = [255, 0, 0, 0];
	leds[8] = [0, 255, 0, 0];
	leds[9] = [0, 0, 255, 0];

	controller.render();
}
