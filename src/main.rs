
#[link(name = "bcm2835", kind = "static")]
extern "C" {
	fn bcm2835_init() -> i32;
	fn bcm2835_gpio_fsel(pin: u8, mode: u8);
	fn bcm2835_gpio_write(pin: u8, state: u8);
	fn bcm2835_delay(millis: u32);
}

fn main() {

    let pin = 17;
    let mode = 1;
    unsafe {
		bcm2835_init();
		bcm2835_gpio_fsel(pin, mode);
    }

    loop {
	    unsafe {
			bcm2835_gpio_write(pin, 1);
			bcm2835_delay(500);
			bcm2835_gpio_write(pin, 0);
			bcm2835_delay(500);
		}
    }
}
