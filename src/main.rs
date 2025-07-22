#![no_main]
#![no_std]

use log::info;
use uefi::{boot, entry, helpers, Status};

#[entry]
fn main() -> Status {
    helpers::init().unwrap();
    info!("Hi");
    boot::stall(10_000_000);

    Status::SUCCESS
}
