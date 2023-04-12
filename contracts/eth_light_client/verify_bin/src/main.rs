#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

macro_rules! debug {
    ($fmt:literal $(,$args:expr)* $(,)?) => {
        #[cfg(feature = "debugging")]
        ckb_std::syscalls::debug(alloc::format!($fmt $(,$args)*));
    };
}

mod entry;
mod error;

use ckb_std::default_alloc;

ckb_std::entry!(program_entry);
default_alloc!();

fn program_entry() -> i8 {
    match entry::main() {
        Ok(_) => 0,
        Err(err) => err.into(),
    }
}
