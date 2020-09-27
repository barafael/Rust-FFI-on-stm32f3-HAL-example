#![no_main]
#![no_std]

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]

use aux5::{entry, prelude::*, Delay, Leds};

/* Include the automatically generated bindings */
include!("bindings.rs");

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let mut gpio_handle: *mut GPIO_TypeDef = unsafe { (GPIOE_BASE as *mut GPIO_TypeDef) };

    loop {
            unsafe {
                HAL_GPIO_TogglePin(gpio_handle, 0x0100u16);
            }
            delay.delay_ms(100u16);
    }
}
