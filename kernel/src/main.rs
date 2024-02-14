#![no_std]
#![no_main]

use bootloader_api::BootInfo;
use core::panic::PanicInfo;

mod framebuffer;
mod serial;

bootloader_api::entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    serial_println!("Starting LiamOS");

    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        framebuffer.buffer_mut().fill(0);
        let info = framebuffer.info();
        for x in 0..info.width {
            for y in 0..info.height {
                framebuffer::set_pixel_in(
                    framebuffer,
                    framebuffer::Position { x, y },
                    framebuffer::Color {
                        red: (x * 255 / info.width) as u8,
                        green: (y * 255 / info.height) as u8,
                        blue: 0,
                    },
                );
            }
        }
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("PANIC: {:#?}", info);
    loop {}
}
