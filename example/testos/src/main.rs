#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(panic_info_message)]
#![feature(asm_const)]
#![allow(unused)]
extern crate alloc;

use core::panic::PanicInfo;

use fatfs2::init_fatfs2;
use vf2_driver::sd::SdHost;
use vf2_driver::serial;
use visionfive2_sd::Vf2SdDriver;

use crate::boot::{hart_id, sleep};
use crate::config::UART_BASE;
use crate::console::PrePrint;
use crate::fatfs::init_fatfs;
use crate::sbi::shutdown;
use preprint::init_print;

mod boot;
mod config;
mod console;
mod fatfs;
mod fatfs2;
mod sbi;

pub fn main() {
    boot::clear_bss();
    boot::init_heap();
    console::init_uart(UART_BASE);
    // console::init_logger();
    println!("boot hart_id: {}", hart_id());
    // init_print(&PrePrint);
    // let sd = Vf2SdDriver::new(sleep);
    // sd.init();

    serial::init_log(log::LevelFilter::Error).unwrap();
    let sd = SdHost;
    sd.init().unwrap();

    println!("sd init ok");
    let mut buf = [0; 512];
    sd.read_block(0, &mut buf);
    println!("buf: {:x?}", &buf[..16]);

    init_fatfs2(sd);
    // init_fatfs(sd);
    println!("shutdown.....");
    shutdown();
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    if let Some(p) = info.location() {
        println!(
            "line {}, file {}: {}",
            p.line(),
            p.file(),
            info.message().unwrap()
        );
    } else {
        println!("no location information available");
    }
    loop {}
}
