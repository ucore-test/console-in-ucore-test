#![no_std]

use console::log;
use crate::sbi::console_putchar;

use core::ffi::c_char;
use core::ffi::{CStr};

mod sbi;
struct Console;

/// 为 `Console` 实现 `console::Console` trait。
impl console::Console for Console {
    fn put_char(&self, c: u8) {
        console_putchar(c.into());
    }
}

#[no_mangle]
pub extern "C" fn con_init(){
    console::init_console(&Console);
    log::set_max_level(log::LevelFilter::Trace);
}

#[no_mangle]
pub extern "C" fn log_test(){
    log::info!("Hello, world!");
    log::trace!("Hello, world!");
    log::warn!("Hello, world!");
    log::error!("Hello, world!");
}

#[no_mangle]
pub extern "C" fn info(s:*mut c_char){
    let c_str = unsafe{
	CStr::from_ptr(s)
    };
    log::info!("{:#?}",c_str);
}

#[no_mangle]
pub extern "C" fn trace(s:*mut c_char){
    let c_str = unsafe{
        CStr::from_ptr(s)
    };
    log::trace!("{:#?}",c_str);
}

#[no_mangle]
pub extern "C" fn warn(s:*mut c_char){
    let c_str = unsafe{
        CStr::from_ptr(s)
    };
    log::warn!("{:#?}",c_str);
}


#[no_mangle]
pub extern "C" fn error(s:*mut c_char){
    let c_str = unsafe{
        CStr::from_ptr(s)
    };
    log::error!("{:#?}",c_str);
}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}
/*
#[cfg(test)]
mod tests {

    use super::{info,log_test};
    #[test]
    fn work(){
        println!("main test!");
        log_test();
        info("hw hello");
    }
}
*/


