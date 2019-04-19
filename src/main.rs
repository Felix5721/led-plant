use rs_ws281x::{ControllerBuilder, ChannelBuilder, StripType};
use rand::prelude::*;
use std::thread;
use palette::{Srgb, Lch, Hsv, Hue};


#[derive(Debug, Clone)]
struct LEDSegment {
	length: u32,
	height: u32,
	offset: u32,

	color_data: Vec<[[u8; 4]; 2]>
}

impl LEDSegment {
	fn new(height: u32, offset: u32 ) -> LEDSegment {
		let data = vec![ [[0,0,0,0];2] ; height as usize];
		LEDSegment { length: 2* height, height, offset, color_data : data}
	}

	fn set_all_color(&mut self , color: [u8; 4]) {
		for i in 0..self.height {
			self.color_data[i as usize] = [ color, color ];
		}
	}

	fn set_off(&mut self) {
		for i in 0..self.height {
			self.color_data[i as usize] = [ [0; 4], [0; 4] ];
		}
	}

	fn set_color(&mut self, pos : u32, color: [u8; 4]) {
		self.color_data[pos as usize] = [ color, color ];
	}

	fn update_controller(&self, mut controller: rs_ws281x::Controller) -> rs_ws281x::Controller {
		let leds = controller.leds_mut(0);

		for i in 0..self.height {
			let data = self.color_data[i as usize];
			leds[(self.offset + i) as usize] = data[0];
			leds[(self.offset + (2 * self.height) - i -1) as usize] = data[1];
		}

		controller
	}
}

fn setup_strip_def() -> Vec<LEDSegment>{
	let mut strips : Vec<LEDSegment> = Vec::new();
	let mut current_offset = 0;

	//fill strips array with correct values specific to the current hardware configuration
	strips.push( LEDSegment::new(48, current_offset) );
	current_offset += strips[0].length;

	strips.push( LEDSegment::new(78, current_offset) );
	current_offset += strips[1].length;

	strips.push( LEDSegment::new(48, current_offset) );
	current_offset += strips[2].length;

	strips.push( LEDSegment::new(48, current_offset) );
	current_offset += strips[3].length;

	
	strips.push( LEDSegment::new(78, current_offset) );
	//current_offset += strips[4].length;

	return strips;
}

fn render_strips(mut controller : rs_ws281x::Controller, strips: &Vec<LEDSegment> ) -> rs_ws281x::Controller{
	for strip in strips.iter() {
		controller = strip.update_controller(controller);
	}
	controller.render();
	controller
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
			.brightness(128)
			.build()
		)
		.build().unwrap();

	let mut plant_strips = setup_strip_def();
	let mut colors : [ palette::Srgb; 5] = [ palette::Srgb::new(0., 0., 0.); 5];

	colors[0] = palette::Srgb::new(0., 1. ,0.);
	colors[1] = palette::Srgb::new(0., 0. ,1.);
	colors[2] = palette::Srgb::new(0., 1. ,1.);
	colors[3] = palette::Srgb::new(1., 1. ,0.);
	colors[4] = palette::Srgb::new(1., 0. ,0.);

	loop {
		for i in 0..5 {
			plant_strips[i].set_all_color([(255. * colors[i].blue) as u8, (255. * colors[i].green) as u8, (255. * colors[i].red) as u8 ,0]);
		}
		controller = render_strips(controller, &plant_strips);
		thread::sleep_ms(200);
		for i in 0..5 {
			let mut lch_color: Hsv = colors[i].into();
			lch_color = lch_color.shift_hue(1.0);
			colors[i] = Srgb::from(lch_color);
		}
	}
}
