// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

use core::cell::UnsafeCell;
use core::marker::PhantomData;

use kernel::bindings;
use kernel::c_str;
use kernel::prelude::*;
use kernel::error::*;
use kernel::types::Opaque;

const USB_INTERFACE_CLASS_HID: u8 = 3;
const USB_INTERFACE_SUBCLASS_BOOT: u8 = 1;
const USB_INTERFACE_PROTOCOL_KEYBOARD: u8 = 1;

pub const fn create_usb_driver<T: UsbDriverTrait>() -> UsbDriver {
    UsbDriver(Opaque::new(bindings::usb_driver {

    }))
}

struct Module {
    _reg: UsbRegistration,
}

pub struct UsbRegistration{
    driver: Pin<UsbDriver>
}

impl UsbRegistration {
    pub fn register(module: &'static kernel::ThisModule) -> Result<Self> {
        let driver = create_usb_driver::<Keyboard>();

        to_result(unsafe {
            bindings::usb_register_driver(driver.0.get(), module as *mut _, b"rust".as_ptr() as *const i8)
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

    unsafe extern "C" fn disconnect_callback(intf: *mut bindings::usb_interface) -> core::ffi::c_int {
        kernel::error::from_result(|| {
            let _intf = unsafe {UsbInterface::from_raw(intf)};
            T::probe(_intf);
            Ok(0)
        })
    }

    unsafe extern "C" fn unlocked_ioctl_callback(intf: *mut bindings::usb_interface, code: i32, buf: &[u8]) -> core::ffi::c_int {
        kernel::error::from_result(|| {
            let _intf = unsafe {UsbInterface::from_raw(intf)};
            T::unlocked_ioctl(_intf, code, buf);
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

unsafe impl Sync for UsbDriver {}

pub struct UsbInterface(Opaque<bindings::usb_interface>);

impl UsbInterface {
    unsafe fn from_raw<'a>(ptr: *mut bindings::usb_interface) -> &'a mut Self {
        let ptr = ptr.cast::<Self>();

        unsafe {&mut *ptr}
    }
}

#[repr(C)]
pub struct UsbDeviceId(bindings::usb_device_id);

unsafe impl Sync for UsbDeviceId {}

impl UsbDeviceId {
    unsafe fn from_raw<'a>(ptr: *mut bindings::usb_device_id) -> &'a mut Self {
        let ptr = ptr.cast::<Self>();

        unsafe {&mut *ptr}
    }

    const fn usb_interface_info(interface_class: u8, interface_subclass: u8, interface_protocol: u8) -> Self {
        Self(usb_device_interface_info(interface_class, interface_subclass, interface_protocol))
    }

    const fn default() -> Self {
        Self(usb_device_default())
    }
}

const fn usb_device_interface_info(interface_class: u8, interface_subclass: u8, interface_protocol: u8) -> bindings::usb_device_id {
    let mut dev = bindings::usb_device_id {
        match_flags: 0,
        idVendor: 0,
        idProduct: 0,
        bcdDevice_lo: 0,
        bcdDevice_hi: 0,
        bDeviceClass: 0,
        bDeviceSubClass: 0,
        bDeviceProtocol: 0,
        bInterfaceClass: 0,
        bInterfaceSubClass: 0,
        bInterfaceProtocol: 0,
        bInterfaceNumber: 0,
        driver_info: 0,
        };
    dev.bInterfaceClass = interface_class;
    dev.bInterfaceSubClass = interface_subclass;
    dev.bInterfaceProtocol = interface_protocol;
    dev
}

const fn usb_device_default() -> bindings::usb_device_id {
   bindings::usb_device_id {
        match_flags: 0,
        idVendor: 0,
        idProduct: 0,
        bcdDevice_lo: 0,
        bcdDevice_hi: 0,
        bDeviceClass: 0,
        bDeviceSubClass: 0,
        bDeviceProtocol: 0,
        bInterfaceClass: 0,
        bInterfaceSubClass: 0,
        bInterfaceProtocol: 0,
        bInterfaceNumber: 0,
        driver_info: 0,
        }
}

pub enum PmMessage {}

struct Keyboard;

impl UsbDriverTrait for Keyboard {
    fn new() -> Self {
        Self {
            Name: c_str!("UsbDriverTest"),
        }
    }
}

#[no_mangle]
pub static __mod_usb_usb_kbd_id_table_device_table: [UsbDeviceId; 2] = [UsbDeviceId::usb_interface_info(USB_INTERFACE_CLASS_HID, USB_INTERFACE_SUBCLASS_BOOT, USB_INTERFACE_PROTOCOL_KEYBOARD), UsbDeviceId::default()];

#[vtable]
pub trait UsbDriverTrait {
    NAME: &'static CStr;

    fn new() -> Self;

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
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust usb driver init");

        let mut reg = UsbRegistration::register(_module)?;

        Ok(Module{ _reg: reg})
    }
}
