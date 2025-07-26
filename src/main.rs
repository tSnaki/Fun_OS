#![no_main]
#![no_std]

use core::ptr::NonNull;

use log::info;
use uefi::{boot::{self, get_handle_for_protocol}, entry, helpers, proto::console::text, system, table::system_table_raw, Status};

use crate::kernel::kernel_init;

mod kernel;

#[entry]
fn main() -> Status {
    helpers::init().unwrap();
    
    let map = unsafe { boot::exit_boot_services(None) };
    
    kernel_init();
}
