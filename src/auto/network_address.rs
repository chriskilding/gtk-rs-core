// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use SocketConnectable;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct NetworkAddress(Object<ffi::GNetworkAddress, ffi::GNetworkAddressClass>): SocketConnectable;

    match fn {
        get_type => || ffi::g_network_address_get_type(),
    }
}

impl NetworkAddress {
    pub fn new(hostname: &str, port: u16) -> NetworkAddress {
        unsafe {
            from_glib_full(ffi::g_network_address_new(hostname.to_glib_none().0, port))
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    pub fn new_loopback(port: u16) -> NetworkAddress {
        unsafe {
            from_glib_full(ffi::g_network_address_new_loopback(port))
        }
    }

    pub fn parse(host_and_port: &str, default_port: u16) -> Result<NetworkAddress, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_network_address_parse(host_and_port.to_glib_none().0, default_port, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn parse_uri(uri: &str, default_port: u16) -> Result<NetworkAddress, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_network_address_parse_uri(uri.to_glib_none().0, default_port, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

pub trait NetworkAddressExt {
    fn get_hostname(&self) -> Option<String>;

    fn get_port(&self) -> u16;

    fn get_scheme(&self) -> Option<String>;

    fn connect_property_hostname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<NetworkAddress> + IsA<glib::object::Object>> NetworkAddressExt for O {
    fn get_hostname(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_network_address_get_hostname(self.to_glib_none().0))
        }
    }

    fn get_port(&self) -> u16 {
        unsafe {
            ffi::g_network_address_get_port(self.to_glib_none().0)
        }
    }

    fn get_scheme(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_network_address_get_scheme(self.to_glib_none().0))
        }
    }

    fn connect_property_hostname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hostname",
                transmute(notify_hostname_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::port",
                transmute(notify_port_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::scheme",
                transmute(notify_scheme_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_hostname_trampoline<P>(this: *mut ffi::GNetworkAddress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NetworkAddress> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NetworkAddress::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_port_trampoline<P>(this: *mut ffi::GNetworkAddress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NetworkAddress> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NetworkAddress::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_scheme_trampoline<P>(this: *mut ffi::GNetworkAddress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NetworkAddress> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NetworkAddress::from_glib_borrow(this).downcast_unchecked())
}
