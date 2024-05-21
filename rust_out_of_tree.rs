// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

use core::cell::UnsafeCell;

use kernel::bindings;
use kernel::error::VTABLE_DEFAULT_ERROR;
use kernel::prelude::*;
use kernel::types::Opaque;

const __LOG_PREFIX: &[u8] = b"rust_out_of_tree\0";
#[cfg(MODULE)]
#[used]
static __IS_RUST_MODULE: () = ();

static mut __MOD: Option<RustOutOfTree> = None;

#[cfg(MODULE)]
static THIS_MODULE: kernel::ThisModule = unsafe {
    kernel::ThisModule::from_ptr(&kernel::bindings::__this_module as *const _ as *mut _)
};

#[cfg(MODULE)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn init_module() -> core::ffi::c_int {
    __init()
}

#[cfg(MODULE)]
#[no_mangle]
pub extern "C" fn cleanup_module() {
    __exit()
}

fn __init() -> core::ffi::c_int {
    match <RustOutOfTree as kernel::Module>::init(&THIS_MODULE) {
        Ok(m) => {
            unsafe {
                __MOD = Some(m);
            }
            return 0;
        },
        Err(e) => {
            return e.to_errno();
        }
    }
}

fn __exit() {
    unsafe {
        __MOD = None;
    }
}

#[cfg(MODULE)]
#[link_section = ".modinfo"]
#[used]
pub static __rust_out_of_tree_0: [u8; 12] = *b"license=GPL\0";

// pub struct UsbDriver(UnsafeCell<bindings::usb_driver>);

pub struct UsbInterface(Opaque<bindings::usb_interface>);

pub struct UsbDeviceId(Opaque<bindings::usb_device_id>);

pub enum PmMessage {}

#[vtable]
pub trait UsbDriver {
    fn probe(intf: &mut UsbInterface, id: &mut UsbDeviceId) -> Result {
        kernel::build_error!(VTABLE_DEFAULT_ERROR)
    }

    fn disconnect(intf: &mut UsbInterface) {}

    // not oxidized, need to improve
    fn unlocked_ioctl(intf: &mut UsbInterface, code: i32, buf: &[u8]) -> Result {
        kernel::build_error!(VTABLE_DEFAULT_ERROR)
    }

    fn suspend(intf: &mut UsbInterface, message: &PmMessage) -> Result {
        kernel::build_error!(VTABLE_DEFAULT_ERROR)
    }

    fn resume(intf: &mut UsbInterface) -> Result {
        kernel::build_error!(VTABLE_DEFAULT_ERROR)
    }

    fn reset_resume(intf: &mut UsbInterface) -> Result {
        kernel::build_error!(VTABLE_DEFAULT_ERROR)
    }

    fn pre_reset(intf: &mut UsbInterface) -> Result {
        kernel::build_error!(VTABLE_DEFAULT_ERROR)
    }

    fn post_reset(intf: &mut UsbInterface) -> Result {
        kernel::build_error!(VTABLE_DEFAULT_ERROR)
    }
}


struct RustOutOfTree {
    numbers: Vec<i32>,
}

impl kernel::Module for RustOutOfTree {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust out-of-tree sample (init)\n");

        let mut numbers = Vec::new();
        numbers.push(72, GFP_KERNEL)?;
        numbers.push(108, GFP_KERNEL)?;
        numbers.push(200, GFP_KERNEL)?;

        Ok(RustOutOfTree { numbers })
    }
}

impl Drop for RustOutOfTree {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("Rust out-of-tree sample (exit)\n");
    }
}
