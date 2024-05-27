// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

use core::cell::UnsafeCell;
use core::marker::PhantomData;

use kernel::bindings;
use kernel::prelude::*;
use kernel::error::*;
use kernel::types::Opaque;

macro_rules! module_device_table {
    ($type:ty, $name:ident) => {
        #[no_mangle]
        pub static __mod_{$type}__{$name}_device_table: *const $type = &$name as *const _;
    };
}

pub const fn create_usb_driver<T: UsbDriver>() -> UsbDriver {
    UsbDriver(Opaque::new(bindings::usb_driver {

    }))
}

struct Module {
    _reg: UsbRegistration,
}

pub struct UsbRegistration{
    driver: Pin<&'static mut UsbDriver>
}

impl UsbRegistration {
    pub fn register(module: &'static kernel::ThisModule, driver: Pin<&'static mut UsbDriver>) -> Result<Self> {
        to_result(unsafe {
            bindings::usb_register_driver(driver.0.get(), &mut kernel::bindings::__this_module as *mut _, b"rust".as_ptr() as *const i8)
        })?;

        Ok(UsbRegistration { driver })
    }
}

impl Drop for UsbRegistration {
    fn drop(&mut self){
        unsafe {
            bindings::usb_deregister(self.driver.0.get())
        };
    }
}

struct UsbAdapter<T: UsbDriverTrait> {
    _p: PhantomData<T>,
}

impl<T: UsbDriverTrait> UsbAdapter<T> {
    unsafe extern "C" fn probe_callback(intf: *mut bindings::usb_interface, id: *mut bindings::usb_device_id) -> core::ffi::c_int {
        kernel::error::from_result(|| {
            let _intf = unsafe {UsbInterface::from_raw(intf)};
            let _id = unsafe {UsbDeviceId::from_raw(id)};
            T::probe(_intf, _id)?;
            Ok(0)
        })
    }
}

const __LOG_PREFIX: &[u8] = b"rust_out_of_tree\0";
#[cfg(MODULE)]
#[used]
static __IS_RUST_MODULE: () = ();

static mut __MOD: Option<Module> = None;

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
    match <Module as kernel::Module>::init(&THIS_MODULE) {
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

pub struct UsbDriver(Opaque<bindings::usb_driver>);

pub struct UsbInterface(Opaque<bindings::usb_interface>);

impl UsbInterface {
    unsafe fn from_raw<'a>(ptr: *mut bindings::usb_interface) -> &'a mut Self {
        let ptr = ptr.cast::<Self>();

        unsafe {&mut *ptr}
    }
}

pub struct UsbDeviceId(Opaque<bindings::usb_device_id>);

impl UsbDeviceId {
    unsafe fn from_raw<'a>(ptr: *mut bindings::usb_device_id) -> &'a mut Self {
        let ptr = ptr.cast::<Self>();

        unsafe {&mut *ptr}
    }
}

pub enum PmMessage {}

struct Keyboard;

impl UsbDriverTrait for Keyboard {
    const Name: &'static CStr = c_str!("UsbDriverTest");
    const ID_TABLE: Box[UsbDeviceId] =
}

#[vtable]
pub trait UsbDriverTrait {
    const NAME: &'static CStr;

    const ID_TABLE: Box<[UsbDeviceId]>;

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


impl kernel::Module for Module {
    static mut DRIVER: UsbDriver = create_usb_driver<Keyboard>();

    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust usb driver init");

        let driver = unsafe {&mut DRIVER};
        let mut reg = UsbRegistration::register(_module, Pin::static_mut(driver))?;

        Ok(Modue{ _reg: reg})
    }
}

impl Drop for RustOutOfTree {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("Rust out-of-tree sample (exit)\n");
    }
}
